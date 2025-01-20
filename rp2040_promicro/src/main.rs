#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;
use sparkfun_pro_micro_rp2040::hal::gpio::Pins;
use sparkfun_pro_micro_rp2040::hal::{clocks::init_clocks_and_plls, pac, watchdog::Watchdog, Sio};
use sparkfun_pro_micro_rp2040::XOSC_CRYSTAL_FREQ;

#[entry]
fn main() -> ! {
    // Take the peripherals
    let mut pac = pac::Peripherals::take().unwrap();

    // Initialize watchdog (needed for resetting clocks)
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    // Initialize system clocks
    let _clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ, // External crystal frequency (12 MHz)
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Initialize the SIO block to configure GPIO
    let sio = Sio::new(pac.SIO);

    // Configure GPIO pins
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Configure GPIO 17 as an output pin
    let mut pin17 = pins.gpio17.into_push_pull_output();

    // Main loop
    loop {
        pin17.set_high().unwrap();
        asm::delay(500_000);
        pin17.set_low().unwrap();
        asm::delay(500_000);
    }
}
