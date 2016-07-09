#![feature(start, libc, lang_items)]
#![no_std]
#![no_main]
#![feature(alloc, heap_api)]
#![feature(unique)]

extern crate libc;
extern crate alloc;
mod io;
mod args;
mod vectors;

#[no_mangle]
pub extern fn main(nargs: i32, args: *const *const u8) -> i32 {
    let (flags, arguments) = args::get(nargs, args);

    for &arg in arguments.iter() {
        if flags.escape {
            let mut check = false;
            for byte in arg.bytes() {
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
            io::print(arg);
        }
        if !flags.no_spaces { io::print_char(b' ' as i8); }
    }

    if !flags.no_newline { io::print_char(b'\n' as i8); }

    0
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! { loop {} }
