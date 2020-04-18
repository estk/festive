#![deny(unsafe_code)]

#[macro_use]
mod sugar;
mod cmdline;
mod error;
mod fork;

pub use error::{Error, Result};
pub use fork::fork;
pub use sugar::ForkId;
