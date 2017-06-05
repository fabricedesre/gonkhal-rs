#!/bin/bash

set -e

adb wait-for-device
adb push ./target/armv7-linux-androideabi/release/examples/$1 /system/bin
