#!/bin/bash

set -e

cargo build --target=armv7-linux-androideabi $@
