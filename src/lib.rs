//! # UI ICONS
//! 
//! #### serve icons from various sources to the frontend
//!
//! ```toml
//! [build-dependencies]
//! uiicons = { version = "*", features = ["build"] }
//! 
//! [dependencies]
//! uiicons = "*"
//! ```
//! 
//! **build.rs**
//! ```rust, no_run
//! uiicons::build().unwrap();
//! ```
//!
//! **main.rs**
//! ```rust, no_run
//! // icon holder
//! let icons = uiicons::embeded_icons();
//! // serve bytes to your front end
//! let svg_bytes = icons.get("my_icon");
//! ```

#[cfg(feature = "build")]
mod build;
#[cfg(feature = "build")]
pub use build::{build, Error as BuildError};

#[cfg(feature = "build")]
mod utils;

mod holder;
pub use holder::EmbededIcons;

#[macro_export]
/// Icon Holder with all Icons ready to serve to your frontend
macro_rules! embeded_icons {
    () => {
        EmbededIcons::load(include_bytes!(concat!(
            env!("OUT_DIR"),
            //"/../../../..",
            "/uiicons",
            "/generated",
            "/uiicons.bin",
        )))
    };
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[cfg(feature = "build")]
    #[error(transparent)]
    Build(#[from] build::Error),
    #[cfg(feature = "build")]
    #[error(transparent)]
    Utils(#[from] utils::Error),
}
