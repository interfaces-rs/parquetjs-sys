//! Raw bindings to the apache-arrow API for projects using wasm-bindgen

#![deny(clippy::all)]

pub(crate) mod class;
pub(crate) mod interface;
pub(crate) mod module;

pub use class::*;
pub use interface::*;
pub use module::*;
