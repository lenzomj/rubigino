# rubigino

## Enable SPI on Raspberry PI
https://www.raspberrypi.org/documentation/hardware/raspberrypi/spi/README.md

By default, the SPI kernel driver is NOT enabled on the Raspberry Pi. To
enable, uncomment the following line in /boot/config.txt:

```
dtparam=spi=on
```

Restart the device. Verify that that the SPI kernel driver is loaded:

```bash
lsmod | grep -i spi
ls -l /dev/spi*
```
