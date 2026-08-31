use std::{
    fs::File,
    io::{copy, Read, Write},
    path::{Path, PathBuf},
};

use blake3::Hasher;
use git2::{Direction, Remote};
use reqwest::blocking::Client;
use zip::ZipArchive;

pub fn latest_commit_sha(owner: &str, repo: &str) -> Result<String, git2::Error> {
    let url = format!("https://github.com/{owner}/{repo}.git");
    let mut remote = Remote::create_detached(url)?;
    remote.connect(Direction::Fetch)?;

    // Look for HEAD (or a specific branch if you prefer)
    for head in remote.list()?.iter() {
        if head.name() == "HEAD" {
            return Ok(head.oid().to_string());
        }
    }

    Err(git2::Error::from_str("HEAD not found"))
}

pub fn download_extract_svg_at_sha(
    owner: &str,
    repo: &str,
    sha: &str,
    output_dir: impl AsRef<Path>,
) -> Result<(PathBuf, String, Vec<PathBuf>), Error> {
    let output_dir = output_dir.as_ref();
    std::fs::create_dir_all(output_dir)?;

    let zip_path = output_dir.join(format!("{repo}-{sha}.zip"));
    let url = format!("https://github.com/{owner}/{repo}/archive/{sha}.zip");

    // ---------- Download + hash ----------
    let response = Client::new().get(&url).send()?.error_for_status()?;

    let mut file = File::create(&zip_path)?;
    let mut hasher = Hasher::new();
    let mut reader = response;
    let mut buf = [0u8; 64 * 1024];

    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
        file.write_all(&buf[..n])?;
    }
    file.flush()?;

    let hash = hasher.finalize().to_hex().to_string();

    // ---------- Extract SVGs ----------
    let extract_dir = output_dir.join("svg");

    std::fs::create_dir_all(&extract_dir)?;

    let svg_files = extract_svgs(&zip_path, &extract_dir)?;

    Ok((zip_path, hash, svg_files))
}

fn extract_svgs(zip_path: &Path, output_dir: &Path) -> Result<Vec<PathBuf>, Error> {
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;
    let mut extracted = Vec::new();

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;

        if entry.is_dir() {
            continue;
        }

        let name = entry.name();
        if !name.to_ascii_lowercase().ends_with(".svg") {
            continue;
        }

        // GitHub archives look like: {repo}-{sha}/path/to/file.svg
        let relative: PathBuf = Path::new(name).components().skip(1).collect();
        if relative.as_os_str().is_empty() {
            continue;
        }

        let output_path = output_dir.join(&relative);
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut out = File::create(&output_path)?;
        copy(&mut entry, &mut out)?;
        extracted.push(output_path);
    }
    Ok(extracted)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("HTTP error")]
    Http(#[from] reqwest::Error),
    #[error("ZIP error")]
    Zip(#[from] zip::result::ZipError),
}
