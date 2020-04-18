use std::io;
use thiserror::Error;

use crate::cmdline;

#[derive(Debug, Error)]
pub enum Error {
    /// An unknown flag was encountered when examining the current
    /// process's argument list.
    ///
    /// The string is the flag that was encountered.
    #[error("The flag '{0}' was passed to the Rust test \
                    process, but rusty-fork does not know how to \
                    handle it.\n\
                    If you are using the standard Rust \
                    test harness and have the latest version of the \
                    rusty-fork crate, please report a bug to\n\
                    \thttps://github.com/AltSysrq/rusty-fork/issues\n\
                    In the mean time, you can tell rusty-fork how to \
                    handle this flag by setting the environment variable \
                    `{}` to one of the following values:\n\
                    \tpass - Pass the flag (alone) to the child process\n\
                    \tpass-arg - Pass the flag and its following argument \
                    to the child process.\n\
                    \tdrop - Don't pass the flag to the child process.\n\
                    \tdrop-arg - Don't pass the flag or its following \
                    argument to the child process.",
                cmdline::env_var_for_flag(&.0))]
    UnknownFlag(String),

    /// A flag was encountered when examining the current process's
    /// argument list which is known but cannot be handled in any sensible
    /// way.
    ///
    /// The strings are the flag encountered and a human-readable message
    /// about why the flag could not be handled.
    #[error(
        "The flag '{flag} was passed to the Rust test \
                    process, but rusty-fork cannot handle it; \
                    reason: {message}"
    )]
    DisallowedFlag { flag: String, message: String },

    /// Spawning a subprocess failed.
    #[error("Spawn failed: {0}")]
    SpawnError(#[from] io::Error),
}

pub type Result<T> = ::std::result::Result<T, Error>;
