use core::slice::from_raw_parts;
use core::str;
use vectors::Vec;
use libc::strlen;

pub fn get<'a>(nargs: i32, args: *const *const u8) -> (Flags, Vec<&'a str>) {
    unsafe { Flags::new(from_raw_parts(args, nargs as usize)) }
}

pub struct Flags {
    pub no_newline:           bool,
    pub no_spaces:            bool,
    pub escape:               bool,
}

impl Flags {
    fn new<'a>(input_args: &[*const u8]) -> (Flags, Vec<&'a str>) {
        // Initialize the flag arguments
        let mut flags = Flags { no_newline: false, no_spaces: false, escape: false };
        // Create a vector to store the arguments which are not flags
        let mut output_args = Vec::new();
        // Iterate across each argument given to the program
        for &input_arg in input_args.iter().skip(1) {
            // The argument is stored as a `*const u8`, so we want to transform it into `&[u8]`
            let input_arg = unsafe { &from_raw_parts(input_arg, strlen(input_arg as *const i8)) };
            // Check the `input_arg` for flags, else push it as a string.
            if input_arg.len() == 2 && input_arg[0] == b'-' {
                match input_arg[1] {
                    b'n' => flags.no_newline = true,
                    b's' => flags.no_spaces = true,
                    b'e' => flags.escape = true,
                    _ => output_args.push(unsafe { str::from_utf8_unchecked(input_arg) })
                }
            } else {
                output_args.push(unsafe { str::from_utf8_unchecked(input_arg) })
            }
        }
        (flags, output_args)
    }
}
