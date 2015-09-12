.PHONY: build

build:
	RUST_BACKTRACE=1 $(HOME)/cargo-build/target/debug/cargo-build --sysroot $(HOME)/rust-rt-minimal/sysroot/ --target i386-unknown-emscripten --emit em-js --release --verbose --emcc ./emcc
