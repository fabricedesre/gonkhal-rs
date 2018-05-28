#!/bin/bash

set -e

./build.sh --release --example $1
./install.sh $1
adb wait-for-device
adb shell $1
