#!/bin/bash

set -e

HOST_OS=$(uname -s)
if [ "$HOST_OS" == "Darwin" ]; then
    HOST_ARCH_S=darwin-x86
else
    HOST_ARCH_S=linux-x86
fi

# Check that the GONK_DIR environment variable is set
# and build the .cargo/config file from it.
if [ -z ${GONK_DIR+x} ];
then
    echo "Please set GONK_DIR to the root of your Gonk directory first.";
    exit 1;
else
    # Get the product name from .config
    source $GONK_DIR/.config
    CARGO_CONFIG=`pwd`/.cargo/config
    echo "Using '$GONK_DIR' to create '$CARGO_CONFIG' for '$PRODUCT_NAME'";
    mkdir -p `pwd`/.cargo
    cat << EOF > $CARGO_CONFIG
[target.armv7-linux-androideabi]
linker="$GONK_DIR/prebuilts/gcc/$HOST_ARCH_S/arm/arm-linux-androideabi-4.9/bin/arm-linux-androideabi-gcc"
rustflags = [
  "-C", "link-arg=--sysroot=$GONK_DIR/out/target/product/$PRODUCT_NAME/obj/",
]
EOF
fi

cargo build --target=armv7-linux-androideabi $@
