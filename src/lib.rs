mod platform;
mod process;
mod region;

#[cfg(target_os = "linux")]
mod common {
    pub use super::platform::linux::Pid;
}

#[cfg(target_os = "windows")]
mod common {
    pub use super::platform::linux::Pid;
}

pub use common::*;

pub enum Error {
    ProcessNotFound,
    OsError(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::OsError(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
