#[cfg(target_os = "linux")]
use crate::platform::linux;

#[cfg(target_os = "windows")]
use crate::platform::windows;

use crate::Pid;

#[derive(Debug)]
pub struct Process {
    pub pid: Pid,
    #[cfg(target_os = "windows")]
    pub handle: windows::HANDLE,
}
