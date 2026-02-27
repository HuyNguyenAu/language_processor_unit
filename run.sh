#!/bin/bash

RUST_BACKTRACE=1 cargo run build $1 --debug && cargo run run build/$(basename "$1" .aasm).lpu --debug