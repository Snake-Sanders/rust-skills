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

    // dbg
    //
    unsafe {
        /*
        // bss is initialized by the liner with NOLOAD
        let sbss = &_sbss as *const u8 as usize;
        let ebss = &_ebss as *const u8 as usize;
        let bss_size = sbss - ebss;
        // set memory to zero
        ptr::write_bytes(&raw mut _sbss as *mut u8, 0, bss_size);
        */

        let data_addr = &raw mut _sdata as *mut u8;
        let sdata = &raw const _sdata as *const u8 as usize;
        let edata = &raw const _edata as *const u8 as usize;
        let sidata = &_sidata as *const u8;

        let data_size = edata - sdata;
        //ptr::copy_nonoverlapping(sidata, data_addr, data_size);
    }

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

pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

extern "C" {
    fn NMI();
    fn HardFault();
    fn MemManage();
    fn BusFault();
    fn UsageFault();
    fn SVCall();
    fn PendSV();
    fn SysTick();
}

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static EXCEPTIONS: [Vector; 14] = [
    Vector { handler: NMI },
    Vector { handler: HardFault },
    Vector { handler: MemManage },
    Vector { handler: BusFault },
    Vector {
        handler: UsageFault,
    },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: SVCall },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: PendSV },
    Vector { handler: SysTick },
];

#[no_mangle]
pub extern "C" fn DefaultExceptionHandler() {
    loop {}
}
