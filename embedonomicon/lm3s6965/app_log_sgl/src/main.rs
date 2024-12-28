// This is an app example of logging using Singleton
#![no_main]
#![no_std]

use cortex_m::interrupt;
use cortex_m_semihosting::{
    debug,
    hio::{self, HStdout},
};

use core::cell::RefCell;
use spin::Mutex;
use spin::Once;

use log::{global_logger, log, GlobalLog};
use rt::entry;

static HSTDOUT: Once<Mutex<RefCell<Option<HStdout>>>> = Once::new();

struct Logger;

impl GlobalLog for Logger {
    fn log(&self, address: u8) {
        interrupt::free(|_| {
            let stdout = HSTDOUT.call_once(|| Mutex::new(RefCell::new(None)));

            let stdout_ref = stdout.lock();

            if stdout_ref.borrow().is_none() {
                *stdout_ref.borrow_mut() = Some(hio::hstdout().unwrap());
            }

            stdout_ref
                .borrow_mut()
                .as_mut()
                .unwrap()
                .write_all(&[address])
                .ok();
        });
    }
}

static LOGGER_INSTANCE: Logger = Logger;
global_logger!(&LOGGER_INSTANCE);

entry!(main);

fn main() -> ! {
    log!("Hello, world!");

    log!("Goodbye");

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
