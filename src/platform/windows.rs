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
                PAGE_EXECUTE, PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE, PAGE_EXECUTE_WRITECOPY,
                PAGE_READONLY, PAGE_READWRITE, PAGE_WRITECOPY,
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

    pub fn get_memory_regions(&self) -> crate::Result<Vec<MemoryRegion>> {
        let mut vec = Vec::new();
        let mut ptr = std::ptr::null_mut();
        let mut inf = MEMORY_BASIC_INFORMATION::default();

        loop {
            unsafe {
                if VirtualQueryEx(
                    self.handle,
                    Some(ptr as *const _),
                    &mut inf,
                    std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
                ) == 0
                {
                    break;
                }

                if inf.State == MEM_COMMIT && ((inf.Type & (MEM_MAPPED | MEM_PRIVATE)).0 != 0) {
                    vec.push(inf.into());
                }
                ptr = inf.BaseAddress.add(inf.RegionSize);
            }
        }

        Ok(vec)
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.handle) };
    }
}

impl From<MEMORY_BASIC_INFORMATION> for MemoryRegion {
    fn from(info: MEMORY_BASIC_INFORMATION) -> Self {
        Self {
            base: info.BaseAddress as usize,
            size: info.RegionSize,
            exec: (info.Protect
                & (PAGE_EXECUTE
                    | PAGE_EXECUTE_READ
                    | PAGE_EXECUTE_READWRITE
                    | PAGE_EXECUTE_WRITECOPY))
                .0
                != 0,
            read: (info.Protect
                & (PAGE_EXECUTE_READ
                    | PAGE_EXECUTE_READWRITE
                    | PAGE_EXECUTE_WRITECOPY
                    | PAGE_READONLY
                    | PAGE_READWRITE
                    | PAGE_WRITECOPY))
                .0
                != 0,
            write: (info.Protect & (PAGE_EXECUTE_READWRITE | PAGE_READWRITE)).0 != 0,
        }
    }
}
