
# USB to TTL serial driver for MacOS 

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


