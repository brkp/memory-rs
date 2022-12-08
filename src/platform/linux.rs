use crate::MemoryRegion;
use crate::Process;

use std::fs::File;
use std::io::Read;

pub type Pid = libc::pid_t;

impl Process {
    pub fn new(pid: Pid) -> crate::Result<Self> {
        if std::path::Path::new(&format!("/proc/{}", pid)).exists() {
            Ok(Self { pid })
        } else {
            Err(crate::Error::ProcessNotFound)
        }
    }

    pub fn get_memory_regions(&self) -> crate::Result<Vec<MemoryRegion>> {
        let mut maps_file = File::open(format!("/proc/{}/maps", self.pid))?;
        let mut maps = String::new();
        maps_file.read_to_string(&mut maps)?;
        println!("{}", maps);

        let mut ret = Vec::new();

        for line in maps.split("\n") {
            match line.split_whitespace().collect::<Vec<&str>>()[..] {
                [address, perms, ..] => {
                    let addr = address.split("-").collect::<Vec<&str>>();
                    let start = usize::from_str_radix(addr[0], 16).unwrap();
                    let end = usize::from_str_radix(addr[1], 16).unwrap();

                    ret.push(MemoryRegion::new(
                        start as *const u8,
                        end - start,
                        perms.contains("x"),
                        perms.contains("r"),
                        perms.contains("w"),
                    ));
                }
                _ => {}
            }
        }

        Ok(ret)
    }

    pub fn read_memory(&self, addr: *const u8, size: usize) -> crate::Result<Vec<u8>> {
        let mut buffer = vec![0u8; size];
        let local = libc::iovec {
            iov_base: buffer.as_mut_ptr() as *mut _,
            iov_len: size,
        };
        let remote = libc::iovec {
            iov_base: addr as *mut _,
            iov_len: size,
        };

        unsafe {
            if libc::process_vm_readv(self.pid, &local, 1, &remote, 1, 0) as usize == size {
                Ok(buffer)
            } else {
                Err(crate::Error::OsError(std::io::Error::last_os_error()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_process_self() {
        assert!(Process::new(std::process::id() as Pid).is_ok());
    }
}
