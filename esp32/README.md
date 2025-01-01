
# ESP32 LilyGo T-Display 

## Installation

### USB to TTL seria driver for MacOS 

The required driver for this board is `CH9102F` but the driver in 
`../usb2ttl_driver/CH341SER_MAC.ZIP` also includes this chip model.

Unzip the driver and open `CH34xVCPDriver.dmg` file 
Drag and drop the file to `Application` like any other app.
It will ask to give permission, this might change from on OS version to another,
so check the PDF in the zip file.

* Source:

https://github.com/WCHSoftGroup/ch34xser_macos

* Forum refrences:

https://arduino.stackexchange.com/a/86787

## Select port

Check under `/dev/*` if the driver was installed.

```sh
lla /dev | grep usb

crw-rw-rw-   1 root   wheel      0x9000003 Dec 31 16:31 cu.usbserial-58AB0097191
crw-rw-rw-   1 root   wheel      0x9000005 Dec 31 16:31 cu.wchusbserial58AB0097191
crw-rw-rw-   1 root   wheel      0x9000002 Dec 31 16:31 tty.usbserial-58AB0097191
crw-rw-rw-   1 root   wheel      0x9000004 Dec 31 16:31 tty.wchusbserial58AB0097191
```

Try selecting `cu.wchusbserial58AB0097191` in Arduino IDE under Tools > Port.

If the device doesn’t respond, try cu.usbserial-58AB0097191.
Both should work identically.

## Board info

Once the port is detected and connected with Arduino IDE, 
try to read the board information.
Go to `Tools/Get board info`:

```
BN: LilyGo T-Display
VID: 0x1A86
PID: 0x55D4
SN: 58AB009719
```

Details: 

This is a LilyGO TTGO T-Display (clone), which is a popular ESP32 development
board with an integrated 1.14" TFT display (driven by the ST7789V controller).

- `LilyGO` is the brand/manufacturer
- `TTGO` is the product line for their ESP32 boards with displays

Features 

- ESP32-D0WDQ6 (dual-core 240MHz processor)
- Integrated 1.14" TFT LCD (ST7789V, SPI interface)
- Two user buttons (can double as reset/boot).
- USB-C for power and programming.

# Project Setup

## Rust

reference `https://lilymara.xyz/posts/2023/01/images-esp32/`

Install cross compiler target

```sh
rustup target install riscv32imc-unknown-none-elf
```

```sh
cargo install espup
espup install

# at this moment setup the environment variables

cargo install cargo-espflash
cargo install cargo-generate
cargo install ldproxy
```
About [espup](https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html)

- checkpoint

```sh
xtensa-esp32-elf-gcc --version

xtensa-esp-elf-gcc (crosstool-NG esp-14.2.0_20240906) 14.2.0
```

**Environment varaiables**

`espup` requires environment variables from `~/export-esp.sh`
to avoid polluting `.zshrc`, the variables are set in `.env`

 '. /Users/mac/export-esp.sh'
This step must be done every time you open a new terminal.
See other methods for setting the environment in: 

https://esp-rs.github.io/book/installation/riscv-and-xtensa.html#3-set-up-the-environment-variables

### Example

```sh
cargo generate https://github.com/esp-rs/esp-idf-template cargo
```

Repositories naming convetions

- `esp-*` are focused on `no_std` approach
- `esp-idf-*` are focused on `std` approach

if the command `cargo run` does not work, 
try build and communicate manually.

- connecting to the board 

```sh
cargo build
cargo espflash board-info

Chip type:         esp32 (revision v1.1)
Crystal frequency: 40 MHz
Flash size:        16MB
Features:          WiFi, BT, Dual Core, 240MHz, Coding Scheme None
MAC address:       ac:15:18:dd:aa:ff
```

- flashing the board

```sh
cargo espflash flash

App/part. size:    499,552/1,048,576 bytes, 47.64%
[00:00:01] [========================================]16/16  0 
[00:00:00] [========================================]1/1    0
[00:00:25] [========================================]255/255 0x10000
[2025-01-01T13:32:28Z INFO ] Flashing has completed!
```

- running the app with logs

```sh
cargo espflash monitor

I (434) main_task: Started on CPU0
I (444) main_task: Calling app_main()
I (444) esp_display: Hello, world!
```

### Docker Containers

https://hub.docker.com/r/espressif/idf-rust/tags

## Arduino IDE 

In Arduino IDE, go to File > Preferences add the extra sources

`https://dl.espressif.com/dl/package_esp32_index.json`

### Selecting the board

Select the Correct Board:

Go to Tools > Board > ESP32 Arduino > ESP32 Dev Module.
This is the closest and most compatible option for the TTGO T-Display.

### changing the TFT library

Open the Arduino IDE do the following:

1. download the `TFT_eSPI` library into your Arduino IDE 
2. select ESP32 Dev Module. 
3. go to `/Users/<USER>/Documents/Arduino/libraries/TFT_eSPI`
4. edit the `User_Setup_Select.h` file in the `TFT_eSPI` library 
5. select the Setup file for ESP32 and TTGO T-Display ST7789V SPI bus TFT 

This is a clone of the TTGO T-Display module, so select the 
setup file for ESP32 and TTGO T-Display ST7789V SPI bus TFT

```c
//...
//#include <User_Setups/Setup24_ST7789.h>            
#include <User_Setups/Setup25_TTGO_T_Display.h>    
//#include <User_Setups/Setup26_TTGO_T_Wristband.h>  
//...
```


