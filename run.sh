#!/bin/bash

BUILD_PATH="build/$(basename "$1" .aasm).lpu"

# Remove existing build if it exists, then build the new LPU. If the build is successful, run the LPU; otherwise, print an error message.
if [ -f "$BUILD_PATH" ]; then
    rm "$BUILD_PATH"
fi

RUST_BACKTRACE=1 cargo run build "$1"

if [ -f "$BUILD_PATH" ]; then
    RUST_BACKTRACE=1 cargo run run "$BUILD_PATH"
fi