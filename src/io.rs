#![allow(dead_code)]
extern crate libc;

extern {
    pub fn putchar(c: i8) -> i8;
}

// Print a string to standard output without flushing
pub fn print(input: &str) {
    unsafe {
        for character in input.as_bytes() {
            let _ = putchar(*character as i8);
        }
    }
}

pub fn print_char(input: i8) {
    unsafe { let _ = putchar(input); }
}

pub fn println(input: &str) {
    unsafe {
        for character in input.as_bytes() {
            let _ = putchar(*character as i8);
        }
        let _ = putchar(b'\n' as i8);
    }
}
