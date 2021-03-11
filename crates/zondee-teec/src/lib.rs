mod error;
#[cfg(feature = "framework")]
pub mod framework;
pub mod wrapper;

pub use error::*;

pub type Result<T> = core::result::Result<T, Error>;
