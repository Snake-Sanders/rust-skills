// This is an app example of logging
//
// __log_warning_start__ is the first warn log code, 01
//
//cargo objdump --bin app_log -- -t | grep '\.log'
//
//Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
//00000000 g     O .log   00000001 Goodbye
//00000001 g     O .log   00000001 Hello, world!
//00000001 g       .log   00000000 __log_warning_start__

#![no_main]
#![no_std]

use cortex_m_semihosting::{
    debug,
    hio::{self, HStdout},
};

use log::{error, warn, Log};
use rt::entry;

struct Logger {
    hstdout: HStdout,
}

impl Log for Logger {
    type Error = ();

    fn log(&mut self, address: u8) -> Result<(), ()> {
        self.hstdout.write_all(&[address])
    }
}

entry!(main);

fn main() -> ! {
    let hstdout = hio::hstdout().unwrap();
    let mut logger = Logger { hstdout };

    let _ = warn!(logger, "Hello, world!");

    let _ = error!(logger, "Goodbye");

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
