
To build and flash your program:

Hold the BOOTSEL button on your Pico while connecting it to your computer via USB
Release the BOOTSEL button after connecting
The Pico should appear as a mass storage device
Run the following command to build and flash:


    cargo build --release
    cargo run --release

The LED on your Pico should now be blinking with a 1-second period (500ms on, 500ms off).
Some additional notes:
The LED is connected to GPIO pin 25 on the Pico
If you want to change the blink rate, modify the delay_ms() values in the main loop
Make sure you have the elf2uf2-rs tool installed by running: cargo install elf2uf2-rs
If you encounter any issues or need to modify the blin rate, just let me know!
