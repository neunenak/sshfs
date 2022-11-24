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

    pub fn init(&mut self, size: u64) {
        if size == 0 {
            self.p = std::ptr::null_mut();
        } else {
            unsafe {
                self.p = libc::malloc(size as usize) as *mut u8;
                if self.p.is_null() {
                    eprintln!("sshfs: memory allocation failed");
                    libc::abort();
                }
            }
        }
        self.size = size;
        self.len = 0;
    }

    pub fn free(&mut self) {
        unsafe {
            libc::free(self.p as *mut libc::c_void);
        }
    }
}
