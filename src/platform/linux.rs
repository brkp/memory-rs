use crate::process::Process;

pub type Pid = libc::pid_t;

impl Process {
    pub fn new(pid: Pid) -> crate::Result<Self> {
        if std::path::Path::new(&format!("/proc/{}", pid)).exists() {
            Ok(Self { pid })
        } else {
            Err(crate::Error::ProcessNotFound)
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
