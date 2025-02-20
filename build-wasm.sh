#!/bin/sh

export RUSTFLAGS="-Ctarget-feature=+simd128"

cargo \
	build \
	--target wasm32-unknown-unknown \
	--profile release-wasm \
