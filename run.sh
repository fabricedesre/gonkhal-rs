#!/bin/bash

set -e

cargo build --target=armv7-linux-androideabi --release --example $1
./install.sh $1
adb wait-for-device
adb shell $1
