// to see the log strings and their addresses
// > cargo objdump --bin logger -- -t | grep '\.log'
//
// to execute the program and see the log output
// > cargo run | xxd -p
// > 0001
// these are the 00 and 01 that need to be mapped to the
// exported names
// we use u8 for the address then we can only have 255 log lines

#![no_main]
#![no_std]

use cortex_m_semihosting::{debug, hio};

use rt::entry;

entry!(main);

fn main() -> ! {
    let mut hstdout = hio::hstdout().unwrap();

    #[export_name = "Hello, world!"]
    #[link_section = ".log"]
    static A: u8 = 0;

    let address = &A as *const u8 as usize as u8;
    hstdout.write_all(&[address]).unwrap();

    #[export_name = "Goodbye"]
    #[link_section = ".log"]
    static B: u8 = 0;

    let address = &B as *const u8 as usize as u8;
    hstdout.write_all(&[address]).unwrap();

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
