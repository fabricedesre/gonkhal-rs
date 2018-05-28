# GonkHal

## Goal

This crate provides access to the Android Hardware Abstraction Layer provided by AOSP and Gonk. We expect the directory layout to be Gonk for now.

__*The current version is based on the KitKat HAL, which is very old. This will likely be scrapped to be based on the Android O HAL instead as soon as possible.*__

## Usage

In order to build, you need to set two environment variables:
- GONK_DIR : the path of your Gonk source directory.
- GONK_PRODUCT_NAME: the codename of your device (eg. aries for a z3c).

Then run `build.sh` to build the library, or pass `--example vibrator` to
build the vibrator example.

The `install.sh --example $example_name` script will copy the specified example
to /system/bin (The /system partition must be mounted in read-write mode with
`adb remount`).

The `run.sh --example $example_name` script will install and run the specified example.
