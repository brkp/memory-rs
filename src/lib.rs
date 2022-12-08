mod platform;
mod process;
mod region;

#[cfg(target_os = "linux")]
pub use platform::linux::Pid;
#[cfg(target = "windows")]
pub use platform::windows::Pid;

pub use process::Process;
pub use region::MemoryRegion;

#[derive(Debug)]
pub enum Error {
    ProcessNotFound,
    ProcessNotRunning,
    PermissionDenied,
    OsError(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        use std::io::ErrorKind;

        match err.kind() {
            ErrorKind::NotFound => Self::ProcessNotFound,
            ErrorKind::PermissionDenied => Self::PermissionDenied,
            _ => Self::OsError(err),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
