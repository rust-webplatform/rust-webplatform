[Online Version](http://bl.ocks.org/AerialX/1041460cb9dd5876658c)

Shamelessly stolen from [tomaka](https://gist.github.com/tomaka/24c058db5ae31dfafb3f).
Doesn't really do anything interesting, but does illustrate how to build Cargo
projects for the web.

## Building

See [cargo-build](https://github.com/AerialX/cargo-build) for instructions on
how to get the environment set up.

    cargo-build --sysroot some/rust/sysroot --target i386-unknown-emscripten --emit em-html
