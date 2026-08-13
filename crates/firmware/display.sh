#!/bin/zsh
# after ./load_pico.sh, run ls /dev/tty.usbmodem* to find all modems
# 115200 is the baud rate for the usb
# og: screen /dev/tty.usbmodem21201 115200

DEVICE=$(ls /dev/tty.usbmodem* 2>/dev/null | head -n 1)
BAUD_RATE = 115200

if [ -z "$DEVICE" ]; then
    echo "No device found"
    exit 1
fi 

echo "Connection to $DEVICE..."
screen "$DEVICE" "$BAUD_RATE"