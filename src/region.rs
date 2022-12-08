#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryRegion {
    pub base: usize,
    pub size: usize,
    pub exec: bool,
    pub read: bool,
    pub write: bool,
}

impl MemoryRegion {
    pub fn new(base: usize, size: usize, exec: bool, read: bool, write: bool) -> Self {
        Self {
            base,
            size,
            exec,
            read,
            write,
        }
    }
}
