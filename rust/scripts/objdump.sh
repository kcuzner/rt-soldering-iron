#!/bin/sh

OUTEXT=".lst"

arm-none-eabi-objdump -D $RUST_ARTIFACT > $RUST_ARTIFACT$OUTEXT

