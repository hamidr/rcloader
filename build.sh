#!/bin/bash

rm ./docker/rcloader
cargo build --target=x86_64-unknown-linux-musl --features vendored --release
cp target/x86_64-unknown-linux-musl/release/rcloader ./docker/
strip ./docker/rcloader
