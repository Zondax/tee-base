#![no_std]

mod error;
mod utils;
mod uuid;

pub use {error::*, utils::*, uuid::*};

pub type Result<T> = core::result::Result<T, Error>;
