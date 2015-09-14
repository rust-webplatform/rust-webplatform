.PHONY: build

build:
	multirust override nightly-2015-03-12
	RUST_BACKTRACE=1 ../cargo-build/target/debug/cargo-build --sysroot ../rust-rt-minimal/sysroot/ --target i386-unknown-emscripten --emit em-js --release --verbose --emcc ./emcc

watch:
	nodemon --watch src --exec "make || true" -e rs

serve:
	@echo 'http://0.0.0.0:8080/webplatform.html'
	@echo ''
	cd target/i386-unknown-emscripten/release/; hs
