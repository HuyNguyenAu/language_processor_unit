#!/bin/bash

RUST_BACKTRACE=1 cargo run build $1 && cargo run run build/$(basename "$1" .aasm).lpu