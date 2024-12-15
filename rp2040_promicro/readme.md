# Raspberry pico 

Model rb2040 pro micro

## Getting started

Sources

https://github.com/rp-rs/rp-hal-boards/tree/main/boards/sparkfun-pro-micro-rp2040

## Dependencies

Install the following dependencies

    rustup target install thumbv6m-none-eabi

    cargo add rp-pico 
    cargo add cortex-m-rt 
    cargo add panic-halt

    cargo install flip-link
    cargo install probe-run 
    cargo install --locked probe-rs-tools
    cargo install --locked elf2uf2-rs

## Build and Flash

Compiled with: 

    cargo build --release 

Note: `--target` parameter is not needed, the build is already configured in 
`.cargo/config.toml`

Convert the image to UF2 format:

    elf2uf2-rs target/thumbv6m-none-eabi/release/rp2040_promicro rp2040_promicro.uf2

The images `.uf2` will be generated in the root folder.

To flash the new image press the `BOOTSEL` bottom on the PCB and 
connect it to the computer USB, then release the button.

Drag and drop the uf2 file onto the `RPI-RP2` drive detected as USB storage.
Once the file is copied, the board will automatically reboot and run your code.

