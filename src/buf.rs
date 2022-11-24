
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Buffer {
    pub p: *mut u8,
    pub len: u64,
    pub size: u64,
}

impl Buffer {
    pub fn new() -> Self {
        Buffer {
            p: 0 as *mut u8,
            len: 0,
            size: 0,
        }
    }
}
