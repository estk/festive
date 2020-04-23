#![deny(unsafe_code)]

pub use festive_macros::*;

use std::io;
use thiserror::Error;

mod cmdline;
#[macro_use]
mod fork;

pub use fork::{fork, ForkId};

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Unexpected flag '{0}' in 'festivities' test process argument list.")]
    UnknownFlag(String),

    #[error("Illegal flag '{flag}' was passed to the test process. Reason: {message}")]
    DisallowedFlag { flag: String, message: String },

    #[error("Spawn failed: {0}")]
    SpawnError(#[from] io::Error),
}
