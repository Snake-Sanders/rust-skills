
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


