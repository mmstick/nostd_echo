#![feature(start, libc, lang_items)]
#![no_std]
#![no_main]
extern crate libc;
mod io;

use core::slice;

#[no_mangle]
pub extern fn main(nargs: i32, args: *const *const u8) -> i32 {
    // Initialize default flag values
    let mut no_newline = false;
    let mut no_spaces  = false;
    let mut escape     = false;

    // Check for specific flags
    let arguments = unsafe { slice::from_raw_parts(args, nargs as usize) };
    for &argument in arguments.iter().skip(1) {
        let argument = unsafe { &slice::from_raw_parts(argument, libc::strlen(argument as *const i8))  };
        if argument.len() == 2 && argument[0] == b'-' {
            match argument[1] {
                b'n' => no_newline = true,
                b's' => no_spaces = true,
                b'e' => escape = true,
                _ => ()
            }
        }
    }

    // Print the input to standard output
    for &argument in arguments.iter().skip(1) {
        let argument = unsafe { &slice::from_raw_parts(argument, libc::strlen(argument as *const i8))  };
        if argument.len() == 2 && argument[0] == b'-' {
            match argument[1] {
                b'n' | b's' | b'e' => (),
                _ => io::print(argument)
            }
        } else {
            if escape {
                let mut check = false;
                for &byte in argument.iter() {
                    match byte {
                        b'\\' if check => {
                            io::print_char(byte as i8);
                            check = false;
                        },
                        b'\\' => check = true,
                        b'a' if check => {
                            io::print_char(7i8); // bell
                            check = false;
                        },
                        b'b' if check => {
                            io::print_char(8i8); // backspace
                            check = false;
                        },
                        b'c' if check => {
                            unsafe { libc::exit(0i32); }
                        },
                        b'e' if check => {
                            io::print_char(27i8); // escape
                            check = false;
                        },
                        b'f' if check => {
                            io::print_char(12i8); // form feed
                            check = false;
                        },
                        b'n' if check => {
                            io::print_char(b'\n' as i8); // newline
                            check = false;
                        },
                        b'r' if check => {
                            io::print_char(b'\r' as i8);
                            check = false;
                        },
                        b't' if check => {
                            io::print_char(b'\t' as i8);
                            check = false;
                        },
                        b'v' if check => {
                            io::print_char(11i8); // vertical tab
                            check = false;
                        },
                        _ if check => {
                            io::print_char(b'\\' as i8);
                            io::print_char(byte as i8);
                            check = false;
                        },
                        _ => io::print_char(byte as i8)
                    }
                }
            } else {
                io::print(argument);
            }
            if !no_spaces { io::print_char(b' ' as i8); }
        }
    }

    if !no_newline { io::print_char(b'\n' as i8); }

    0
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop {} }
