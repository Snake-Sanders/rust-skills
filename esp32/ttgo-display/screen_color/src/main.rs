// Provides the builder for DisplayInterface
use display_interface_spi::SPIInterface;
use esp_idf_svc::hal::delay::Ets;
use esp_idf_svc::hal::gpio::AnyIOPin;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::spi::SpiConfig;
use esp_idf_svc::hal::spi::SpiDeviceDriver;
use esp_idf_svc::hal::spi::SpiDriver;
use esp_idf_svc::hal::spi::SpiDriverConfig;
use esp_idf_svc::hal::spi::SPI2;
// Provides the builder for Display
use mipidsi::{models::ST7789, Builder};
// Provides the required color type
use embedded_graphics::prelude::RgbColor;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

use std::error::Error;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    log::set_max_level(log::LevelFilter::Debug);
    log::info!("Starting display...v1");

    let peripherals = Peripherals::take()?;

    /* Define the SPI interface as the variable `spi` */
    let mut delay = Ets;

    // Pin definitions
    let sclk = peripherals.pins.gpio18; // SCLK
    let sdo = peripherals.pins.gpio19; // SDO aka MOSI
    let rst = peripherals.pins.gpio23; // Reset
    let dc = peripherals.pins.gpio16; // Data/Command
    let cs = peripherals.pins.gpio5; // Chip Select
    let bl = peripherals.pins.gpio4; // Backlight

    // Initialize GPIO
    let rst = PinDriver::output(rst)?;
    let dc = PinDriver::output(dc)?;
    let mut bl = PinDriver::output(bl)?;
    let sdi: Option<AnyIOPin> = None;

    // SpiDriver – Represents the raw SPI bus where
    // one SpiDriver per SPI bus, this handles many SPI devices
    // connected to this bus.

    let spi = SpiDriver::new::<SPI2>(
        peripherals.spi2,
        sclk,
        sdo,
        sdi, // no MISO
        &SpiDriverConfig::default(),
    )?;

    // SpiDeviceDriver - A higher-level abstraction representing
    // a single device on the SPI bus.
    let spi_device = SpiDeviceDriver::new(spi, Some(cs), &SpiConfig::default())?;

    // Create a DisplayInterface from SPI and DC pin, with no manual CS control
    let di = SPIInterface::new(spi_device, dc);

    // Create the ILI9486 display driver from the display interface and optional RST pin
    let mut display = Builder::new(ST7789, di)
        .reset_pin(rst)
        .init(&mut delay)
        .unwrap();

    // Clear the display to red
    display.clear(Rgb565::RED.into()).unwrap();

    // back light on
    bl.set_high()?;

    log::info!("Display initialized.");

    loop {
        thread::sleep(Duration::from_secs(1));
        display.clear(Rgb565::BLUE.into()).unwrap();

        thread::sleep(Duration::from_secs(1));
        display.clear(Rgb565::MAGENTA.into()).unwrap();

        thread::sleep(Duration::from_secs(1));
        display.clear(Rgb565::GREEN.into()).unwrap();

        thread::sleep(Duration::from_secs(1));
        display.clear(Rgb565::YELLOW.into()).unwrap();
        log::info!("repeat");
    }

    //Ok(())
}
