#![allow(dead_code)]
extern crate libc;

extern {
    pub fn putchar(c: i8) -> i8;
}

pub fn print(input: &[u8]) {
    unsafe {
        for character in input {
            let _ = putchar(*character as i8);
        }
    }
}

pub fn print_char(input: i8) {
    unsafe { let _ = putchar(input); }
}
