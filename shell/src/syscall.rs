pub const STDOUT: i32 = 1;
pub const PATH_MAX: i32 = 4096;

// #[cfg(target_family = "unix")]
#[link(name = "c")]
extern "C" {
    pub fn write(fd: i32, buf: *const u8, count: usize) -> isize;
    pub fn getcwd(buf: *mut i8, size: i32) -> *mut i8;
}