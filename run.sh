#!/bin/bash
DIR="$( dirname "${BASH_SOURCE[0]}" )"
cd "$DIR/build/"
cargo run --release -- "$@"
