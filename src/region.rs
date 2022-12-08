#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryRegion {
    base: *const u8,
    size: usize,
    exec: bool,
    read: bool,
    write: bool,
}

impl MemoryRegion {
    pub fn new(base: *const u8, size: usize, exec: bool, read: bool, write: bool) -> Self {
        Self {
            base,
            size,
            exec,
            read,
            write,
        }
    }
}
