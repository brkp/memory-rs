use crate::{MemoryRegion, Process};

pub use windows::{
    core::{Error, Result, HRESULT},
    Win32::{
        Foundation::{
            CloseHandle, GetLastError, ERROR_ACCESS_DENIED, ERROR_INVALID_PARAMETER, HANDLE,
            NTSTATUS, STILL_ACTIVE,
        },
        System::{
            Diagnostics::Debug::ReadProcessMemory,
            Memory::{
                VirtualQueryEx, MEMORY_BASIC_INFORMATION, MEM_COMMIT, MEM_MAPPED, MEM_PRIVATE,
            },
            Threading::{
                GetExitCodeProcess, OpenProcess, PROCESS_ACCESS_RIGHTS, PROCESS_QUERY_INFORMATION,
                PROCESS_VM_READ,
            },
        },
    },
};

pub type Pid = u32;

impl Process {
    pub fn new(pid: Pid) -> crate::Result<Self> {
        match unsafe { OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, false, pid) } {
            Ok(handle) => {
                let mut exit_code = 0;
                if unsafe { GetExitCodeProcess(handle, &mut exit_code).as_bool() } {
                    if NTSTATUS(exit_code as i32) == STILL_ACTIVE {
                        return Ok(Self { pid, handle });
                    }
                }

                unsafe { CloseHandle(handle) };
                return Err(crate::Error::ProcessNotFound);
            }
            Err(err) => {
                if err.code() == ERROR_INVALID_PARAMETER.to_hresult() {
                    return Err(crate::Error::ProcessNotFound);
                }

                let err: std::io::Error = err.into();
                return Err(crate::Error::from(err));
            }
        }
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.handle) };
    }
}
