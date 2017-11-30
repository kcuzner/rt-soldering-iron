#!/bin/sh

BINEXT=".bin"
HEXEXT=".hex"

arm-none-eabi-objcopy -O binary $RUST_ARTIFACT $RUST_ARTIFACT$BINEXT
arm-none-eabi-objcopy -O ihex $RUST_ARTIFACT $RUST_ARTIFACT$HEXEXT

