#!/bin/bash

SCRIPTDIR=$(dirname $0)
BINEXT=".bin"
PID=`pidof openocd`
WORKDIR=`mktemp -d`

ARTIFACT_NAME=$(basename $RUST_ARTIFACT)

arm-none-eabi-objcopy -O binary $RUST_ARTIFACT $WORKDIR/$ARTIFACT_NAME$BINEXT

ENDWORD="exit"

if [ -z PID ]; then
    openocd -f $SCRIPTDIR/openocd.cfg &
    ENDWORD="shutdown"
fi

nc localhost 4444 <<EOF
init
reset halt
program $WORKDIR/$UST_ARTIFACT$BINEXT verify reset 0x08000000
$ENDWORD
EOF

if [ -z PID ]; then
    wait
fi

