//use embedded_hal::spi::MODE_3;

//use esp_idf_hal::delay::Ets;
//use esp_idf_hal::gpio::*;
//use esp_idf_hal::peripherals::Peripherals;
//use esp_idf_hal::spi::*;
//
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
//use esp_idf_svc::hal::spi::SpiSingleDeviceDriver;

// Provides the builder for Display
use mipidsi::{models::ST7789, Builder};

// Provides the required color type
use embedded_graphics::prelude::RgbColor;
use embedded_graphics::{pixelcolor::Rgb666, prelude::*};

use std::error::Error;

// esp_idf_svc wraps other crates
// - esp-idf0-hal is in esp_idf_svc::hal
// - esp-idf-sys is in esp_idf_svc::sys
// For SPI driver No Chip Select use SpiDeviceDriver::new
// https://github.com/esp-rs/esp-idf-hal/blob/master/src/spi.rs
//

//const BL: i32 = 4;
//const CS: i32 = 5;
//const DC: i32 = 16;
//const MOSI: i32 = 19;
//const RST: i32 = 23;
//const SCLK: i32 = 18;

fn main() -> Result<(), Box<dyn Error>> {
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    log::set_max_level(log::LevelFilter::Debug);
    log::info!("Demo color display");

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
    let mut rst = PinDriver::output(rst)?;
    let mut dc = PinDriver::output(dc)?;
    //let mut cs = PinDriver::output(cs)?;
    let mut bl = PinDriver::output(bl)?;

    // configuring the spi interface,
    // note that in order for the ST7789 to work,
    // the data_mode needs to be set to MODE_3
    //let config = config::Config::new()
    //    .baudrate(26.MHz().into())
    //    .data_mode(MODE_3);

    // SpiDriver – Represents the raw SPI bus where
    // one SpiDriver per SPI bus, this handles many SPI devices
    // connected to this bus.
    //
    let sdi: Option<AnyIOPin> = None;
    //unsafe { Some(AnyIOPin::new(-1)) }; // Pin -1 as a "NoPin"

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
    display.clear(Rgb666::RED.into()).unwrap();

    Ok(())
}
