pub const STDOUT:u32 = 1;

// #[cfg(target_family = "unix")]
#[link(name = "c")]
extern "C" {
    pub fn write(fd: u32, buf: *const u8, count: usize) -> i32;
    // pub fn getcwd(buf: )
}