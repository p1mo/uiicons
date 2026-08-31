mod utils;

use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;

use rayon::prelude::*;
use speedy::Writable;

use crate::utils::config::load_config;
use crate::utils::config::Config;
use crate::utils::github::download_extract_svg_at_sha;
use crate::utils::github::latest_commit_sha;
use crate::utils::sources::Source;

type ReposPath = PathBuf;
type CheckPath = PathBuf;
type SchemaPath = PathBuf;
type BinaryPath = PathBuf;
type ConfigPath = PathBuf;
type JSPath = PathBuf;

fn builder_paths() -> Result<
    (
        ReposPath,
        CheckPath,
        SchemaPath,
        BinaryPath,
        ConfigPath,
        JSPath,
    ),
    Error,
> {
    let output_path = PathBuf::from(std::env::var("OUT_DIR")?);
    let project_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);

    let target_root = output_path
        .ancestors()
        .nth(4)
        .expect("failed to determine current Cargo target directory")
        .to_path_buf();

    let profile_path = target_root.join(std::env::var("PROFILE").unwrap());
    let crate_full_dir = profile_path.join("uiicons");

    let repos = crate_full_dir.join("repositories");
    if !repos.exists() {
        std::fs::create_dir_all(&repos)?;
    }

    let generated = crate_full_dir.join("generated");
    if !generated.exists() {
        std::fs::create_dir_all(&generated)?;
    }

    let gen_dir = project_path.join("gen");
    if !gen_dir.exists() {
        std::fs::create_dir_all(&gen_dir)?;
    }

    Ok((
        // [0] path to temp repos
        repos,
        // [1] hash of the user config to check for changes
        generated.join("uiicons.checksum"),
        // [2] path to schema file
        gen_dir.join("uiicons.schema.json"),
        // [3] path to binary holding icons
        generated.join("uiicons.bin"),
        // [4] path to user settings uiicons.json
        project_path.join("uiicons.json"),
        // [3] path to binary holding icons
        generated.join("component.js"),
    ))
}

fn has_changes(config: impl AsRef<Path>, checksum: impl AsRef<Path>) -> Result<bool, Error> {
    if checksum.as_ref().exists() {
        let hash = utils::hasher(&std::fs::read(config.as_ref())?);
        let hash_old = std::fs::read_to_string(checksum.as_ref())?;
        if hash == hash_old {
            return Ok(false);
        } else {
            return Ok(true);
        }
    }
    Ok(true)
}

fn update_checksum(config: impl AsRef<Path>, checksum: impl AsRef<Path>) -> Result<(), Error> {
    let hash = utils::hasher(&std::fs::read(config.as_ref())?);
    Ok(std::fs::write(checksum.as_ref(), hash)?)
}

fn clone_repos(
    config: &Config,
    repo_path: impl AsRef<Path>,
    config_path: impl AsRef<Path>,
    checksum: impl AsRef<Path>,
) -> Result<(), Error> {
    if has_changes(config_path, checksum)? {
        let sources: HashSet<Source> = config
            .icons
            .par_iter()
            .map(|(_, icon)| icon.source.clone())
            .collect();
        for source in sources {
            let source_info = source.get_info()?;
            let github = source_info.info.github;

            let gh_owner = github.get(0).unwrap();
            let gh_repo = github.get(1).unwrap();

            let hashed_name = utils::hasher(&format!("{}/{}", &gh_owner, &gh_repo).into_bytes());

            let gh_repo_path = repo_path.as_ref().join(&hashed_name);
            if !gh_repo_path.exists() {
                std::fs::create_dir_all(&gh_repo_path)?;
            }

            let repo_sha_file = gh_repo_path.join("repo.sha");
            if !repo_sha_file.exists() {
                std::fs::write(&repo_sha_file, "0")?;
            }

            let old_sha = std::fs::read_to_string(&repo_sha_file)?;
            let new_sha = latest_commit_sha(gh_owner, gh_repo).unwrap();

            if !repo_sha_file.exists() || old_sha != new_sha {
                download_extract_svg_at_sha(gh_owner, gh_repo, &new_sha, &gh_repo_path).unwrap();
                std::fs::write(repo_sha_file, new_sha)?;
            }
        }
    }
    Ok(())
}

/// ### Icon Builder
/// Builds all files needed
pub fn build() -> Result<(), Error> {
    let buildpath = builder_paths()?;

    println!(
        "cargo:rerun-if-changed={}",
        buildpath.4.display().to_string().replace("\\", "/"),
    );

    utils::dump_schema(&buildpath.2)?;
    utils::dump_default(&buildpath.4)?;

    let config = load_config(&buildpath.4)?;

    clone_repos(&config, &buildpath.0, &buildpath.4, &buildpath.1)?;

    if has_changes(&buildpath.4, &buildpath.1)? {
        #[cfg(feature = "js")]
        {
            let raw_js = include_str!("../../components/element.js")
                .replace("[UI_NAME]", &config.component.icon)
                .replace("[UI_SIZE]", &config.component.size)
                .replace("[UI_COLOR]", &config.component.kind)
                .replace("[COMPONENT_NAME]", &config.component.name);
            std::fs::write(buildpath.5, raw_js)?;
        }

        let mut map: HashMap<String, Vec<u8>> = HashMap::new();

        for (name, icon) in config.icons {
            let source_info = icon.source.get_info()?;
            let github = source_info.info.github;
            let gh_owner = github.get(0).unwrap();
            let gh_repo = github.get(1).unwrap();

            let hashed_name = utils::hasher(&format!("{}/{}", &gh_owner, &gh_repo).into_bytes());

            let gh_repo_path = buildpath.0.join(&hashed_name);
            if !gh_repo_path.exists() {
                std::fs::create_dir_all(&gh_repo_path)?;
            }

            let icon_kind = icon.kind.unwrap_or("default".to_string());
            let filepath = source_info
                .kind
                .get(&icon_kind)
                .unwrap()
                .replace("[NAME]", &icon.icon);
            for entry in jwalk::WalkDir::new(gh_repo_path)
                .sort(true)
                .into_iter()
                .filter_map(Result::ok)
            {
                if entry.path().ends_with(&filepath) {
                    map.insert(name.clone(), std::fs::read(entry.path())?);
                }
            }
        }

        update_checksum(buildpath.4, buildpath.1)?;
        std::fs::write(
            buildpath.3,
            crate::holder::EmbededIcons::new(map).write_to_vec()?,
        )?;
    }
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    // std errors
    #[error(transparent)]
    Io(#[from] std::io::Error),
    // std errors
    #[error(transparent)]
    VarError(#[from] std::env::VarError),
    // std errors
    #[error(transparent)]
    SpeedyError(#[from] speedy::Error),
    // std errors
    #[error(transparent)]
    SourcesError(#[from] crate::utils::sources::Error),
}
