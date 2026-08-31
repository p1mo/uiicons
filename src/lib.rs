//! # UIIcons
//!
//! **build.rs**
//! ```
//! fn main() {
//!     uiicons::build().unwrap();
//! }
//! ```
//!
//! **main.rs**
//! ```
//! // Optional: with feature flag `js` serve component to frontend
//! let js_bytes = uiicons::embeded_js();
//! 
//! // icon holder
//! let icons = uiicons::embeded_icons();
//! 
//! // serve bytes to your front end
//! let svg_bytes = icons.get("my_icon");
//! ```

#[cfg(feature = "build")]
mod build;
#[cfg(feature = "build")]
pub use build::{ build, Error as BuildError };

#[cfg(feature = "build")]
mod utils;


mod holder;
pub use holder::EmbededIcons;



#[macro_export]
/// Icon Holder with all Icons ready to serve to your frontend
macro_rules! embeded_icons {
    () => {
        EmbededIcons::load(include_bytes!(concat!(env!("OUT_DIR"), "/../../../..", "/uiicons/generated", "/uiicons.bin")))
    };
}



#[cfg(feature = "js")]
#[macro_export]
/// HTML Component using Vanilla Custom Component API ready to serve to your frontend
macro_rules! embeded_js {
    () => {
        include_bytes!(concat!(env!("OUT_DIR"), "/../../../..", "/uiicons/generated", "/component.js"))
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