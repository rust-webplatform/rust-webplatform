.PHONY: build

build:
	RUST_BACKTRACE=1 $(HOME)/cargo-build/target/cargo-build --sysroot $(HOME)/rust-rt-minimal/sysroot/ --target i386-unknown-emscripten --emit em-html --release --verbose --emcc ./emcc
