pub mod command;
pub mod syscall;

use std::{ffi::CStr, io::{self}, path::PathBuf};

use syscall::PATH_MAX;

pub struct Shell {
    cwd: PathBuf,
    _history: Vec<String>, //TODO: Must be Vec<String>, as it gives of exactly this
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            cwd: current_dir().expect("Error: Could not get current working directory!"),
            _history: Vec::new(),
        }
    }

    pub fn show_cwd(&self) -> String {
        self.cwd.display().to_string().to_owned()
    }

    pub fn save_command(&mut self, command: String) {
        self._history.push(command);
    }
}

pub fn read_stdin() -> io::Result<String> {
    let mut input = String::new();
    let _bytes = io::stdin().read_line(&mut input)?;
    Ok(input)
}

pub fn write_stdout(msg: &str) -> io::Result<()> {
    let msg_ptr = msg.as_ptr();
    let len = msg.len();
    let res = unsafe {syscall::write(syscall::STDOUT, msg_ptr, len)};
    if res == -1 {
        return Err(io::Error::last_os_error())
    }
    Ok(())
}

fn current_dir() -> io::Result<PathBuf> {
    let mut buffer = [0i8; PATH_MAX as usize]; // might be u8
    let ptr = unsafe{syscall::getcwd(buffer.as_mut_ptr(), PATH_MAX)};

    if ptr.is_null() {
        return Err(io::Error::last_os_error());
    }

    let c_str = unsafe {CStr::from_ptr(buffer.as_ptr() as *const _)};

    // let c_str = CStr::from_bytes_until_nul(bytes) // TODO: check if I can use this

    c_str.to_str()
        .map(|s| PathBuf::from(s.to_owned()))
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 Path"))
}

// TODO: might use this for reading from files.
// impl Read for Shell {
//     fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
//         todo!()
//     }
// }

// Simple
// fn current_dir() -> io::Result<PathBuf> {
//     env::current_dir()
// }

// TODO: Use something From STDLIB to improve your code
// pub fn getcwd() -> io::Result<PathBuf> {
//     let mut buf = Vec::with_capacity(512);
//     loop {
//         unsafe {
//             let ptr = buf.as_mut_ptr() as *mut libc::c_char;
//             if !libc::getcwd(ptr, buf.capacity()).is_null() {
//                 let len = CStr::from_ptr(buf.as_ptr() as *const libc::c_char).to_bytes().len();
//                 buf.set_len(len);
//                 buf.shrink_to_fit();
//                 return Ok(PathBuf::from(OsString::from_vec(buf)));
//             } else {
//                 let error = io::Error::last_os_error();
//                 if error.raw_os_error() != Some(libc::ERANGE) {
//                     return Err(error);
//                 }
//             }

//             // Trigger the internal buffer resizing logic of `Vec` by requiring
//             // more space than the current capacity.
//             let cap = buf.capacity();
//             buf.set_len(cap);
//             buf.reserve(1);
//         }
//     }
// }