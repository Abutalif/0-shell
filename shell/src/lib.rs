pub mod command;
pub mod ffi;


use std::{io::{self, Write}, process::exit};

pub fn read_stdin() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // TODO: no unwrap()!
    input
}

pub fn write_stdout(msg: &str) {
    let msg_ptr = msg.as_ptr();
    let len = msg.len();
    let res = unsafe {ffi::write(ffi::STDOUT, msg_ptr, len)};
    if res == -1 {
        eprintln!("could not return ffi");
        exit(1);
    }
}

fn check() {
}