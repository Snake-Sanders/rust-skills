#![no_std]

use core::panic::PanicInfo;
use core::ptr;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn ResetHandler() -> ! {
    // references the linker symbols to
    // initialize the memory sections
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;
        static mut _sdata: u8;
        static mut _edata: u8;
        static _sidata: u8;
    }

    // calculates the size of bss section
    let count = &raw const _ebss as *const u8 as usize - &raw const _sbss as *const u8 as usize;
    // set memory to zero
    ptr::write_bytes(&raw mut _sbss as *mut u8, 0, count);

    // calculates the size of data section
    let count = &raw const _edata as *const u8 as usize - &raw const _sdata as *const u8 as usize;
    // initializes the symbols in data
    ptr::copy_nonoverlapping(&_sidata as *const u8, &raw mut _sdata as *mut u8, count);

    extern "Rust" {
        fn main() -> !;
    }

    main()
}

// The reset vector, a pointer into the reset handler
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = ResetHandler;

// export a macro to define the entry point for app,
// call as entry!(main)
#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub unsafe fn _main() -> ! {
            let f: fn() -> ! = $path;

            f()
        }
    };
}
