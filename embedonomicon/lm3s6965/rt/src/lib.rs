#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn ResetHandler() -> ! {
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
