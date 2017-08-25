#!/bin/sh

rm -rf target

alias rust-musl-builder='docker run --rm -it -v "$(pwd):/home/rust/src" ekidd/rust-musl-builder'

rust-musl-builder cargo build --release

docker build . -t reitermarkus/server
