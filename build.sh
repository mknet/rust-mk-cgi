#!/usr/bin/env bash

docker run --rm -it -v "$(pwd)":/home/rust/src nwtgck/rust-musl-builder cargo build --release