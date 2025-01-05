use display_interface_spi::SPIInterface; // Provides the builder for DisplayInterface
use embedded_graphics::{pixelcolor::Rgb666, prelude::*};
use mipidsi::{models::ST7789, Builder}; // Provides the builder for Display // Provides the required color type

use embedded_graphics::prelude::RgbColor;
use esp_idf_svc::sys;
use std::error::Error;

use esp_idf_hal::{
    delay::Ets,
    //gpio::{AnyIOPin, Gpio16, Gpio18, Gpio19, Gpio23, Gpio5, PinDriver},
    spi::{config::Config, Dma, SpiDeviceDriver, SpiDriver, SPI2},
};

fn main() -> Result<(), Box<dyn Error>> {
    sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    log::info!("Demo color display");

    /* Define the SPI interface as the variable `spi` */
    //let spi = todo!();
    let spi = unsafe { SPI2::new() };

    /* Define the DC digital output pin as the variable `dc` */
    let dc = todo!();
    /* Define the Reset digital output pin as the variable `rst` */
    let rst = todo!();

    let delay = todo!();

    // Create a DisplayInterface from SPI and DC pin, with no manual CS control
    let di = SPIInterface::new(spi, dc);

    // Create the ILI9486 display driver from the display interface and optional RST pin
    let mut display = Builder::new(ST7789, di)
        .reset_pin(rst)
        .init(&mut delay)
        .unwrap();

    // Clear the display to red
    display.clear(Rgb666::RED.into()).unwrap();

    Ok(())
}
