#!/bin/bash

if [ ! -d emsdk_portable ]; then
	curl https://s3.amazonaws.com/mozilla-games/emscripten/releases/emsdk-portable.tar.gz | tar -xvzf - -C .
	cd emsdk_portable
	./emsdk install emscripten-1.30.0;
	./emsdk install clang-tag-e1.30.0-64bit;
	cd ..
fi

if [ ! -d llvm-3.5.2.src ]; then
	curl http://llvm.org/releases/3.5.2/llvm-3.5.2.src.tar.xz | tar -xvjf - -C .
	cd llvm-3.5.2.src
	./configure --enable-shared --enable-keep-symbols
	make -j64
	cd ..
fi

if [ ! -d cargo-build ]; then
	git clone https://github.com/tcr/cargo-build -b next
fi

if [ ! -d rust-rt-minimal ]; then
	git clone https://github.com/AerialX/rust-rt-minimal.git --depth 1
fi

ROOT=$(pwd)

cd emsdk_portable
./emsdk activate emscripten-1.30.0;
./emsdk activate clang-tag-e1.30.0-64bit
. ./emsdk_env.sh

export LLVM_PREFIX=$ROOT/llvm-3.5.2.src/Release+Asserts
export PATH=$LLVM_PREFIX/bin:$PATH
export TRIPLE=i386-unknown-emscripten

cd $ROOT/cargo-build
multirust override 1.2.0
cargo build

cd $ROOT/rust-rt-minimal
multirust override nightly-2015-03-12
tee Cargo.lock <<'EOF'
[root]
name = "std_group"
version = "1.0.0"
dependencies = [
 "flate 1.0.0",
 "std 1.0.0",
]

[[package]]
name = "alloc"
version = "1.0.0"
dependencies = [
 "core 1.0.0",
 "libc 1.0.0",
]

[[package]]
name = "collections"
version = "1.0.0"
dependencies = [
 "alloc 1.0.0",
 "core 1.0.0",
 "unicode 1.0.0",
]

[[package]]
name = "core"
version = "1.0.0"

[[package]]
name = "flate"
version = "1.0.0"
dependencies = [
 "libc 1.0.0",
 "miniz 1.0.0",
 "std 1.0.0",
]

[[package]]
name = "gcc"
version = "0.3.1"
source = "registry+https://github.com/rust-lang/crates.io-index"

[[package]]
name = "libc"
version = "1.0.0"
dependencies = [
 "core 1.0.0",
]

[[package]]
name = "miniz"
version = "1.0.0"
dependencies = [
 "gcc 0.3.1 (registry+https://github.com/rust-lang/crates.io-index)",
]

[[package]]
name = "rand"
version = "1.0.0"
dependencies = [
 "core 1.0.0",
]

[[package]]
name = "rust_builtin"
version = "1.0.0"
dependencies = [
 "gcc 0.3.1 (registry+https://github.com/rust-lang/crates.io-index)",
]

[[package]]
name = "rustc_bitflags"
version = "1.0.0"
dependencies = [
 "core 1.0.0",
]

[[package]]
name = "std"
version = "1.0.0"
dependencies = [
 "alloc 1.0.0",
 "collections 1.0.0",
 "core 1.0.0",
 "libc 1.0.0",
 "rand 1.0.0",
 "rust_builtin 1.0.0",
 "rustc_bitflags 1.0.0",
 "unicode 1.0.0",
]

[[package]]
name = "unicode"
version = "1.0.0"
dependencies = [
 "core 1.0.0",
]
EOF

cargo build --release --target $TRIPLE
mkdir -p sysroot/lib/rustlib/$TRIPLE/lib
cp target/$TRIPLE/release/deps/lib*.rlib sysroot/lib/rustlib/$TRIPLE/lib/

cd $ROOT
echo 'You are now free to compile.'
