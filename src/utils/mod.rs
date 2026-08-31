pub mod config;
pub mod github;
pub mod sources;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Github error")]
    Github(#[from] github::Error),
    #[error("Sources error")]
    Sources(#[from] sources::Error),
}
