# rust-webplatform

```rust
let body = HtmlNode::query("body").unwrap();
body.html_set("<h1>HELLO FROM RUST</h1> <button>CLICK ME</button>");
let mut button = HtmlNode::query("button").unwrap();
button.on("click", || alert("WITNESS ME"));
```

Soon!

Shamelessly stolen from [tomaka](https://gist.github.com/tomaka/24c058db5ae31dfafb3f) and then [AerialX](https://github.com/AerialX).

## Building

See [cargo-build](https://github.com/AerialX/cargo-build) for instructions on
how to get the environment set up. Then run `make`.

Tips:

* Install LLVM 3.5.
* When installing emscripten, use `./emsdk install emscripten-1.30.0; ./emsdk activate emscripten-1.30.0; ./emsdk install clang-tag-e1.30.0-64bit; ./emsdk activate clang-tag-e1.30.0-64bit`
* Install [multirust](https://github.com/brson/multirust).
* Use [this branch of cargo-build](https://github.com/AerialX/cargo-build/pull/4) and cargo-build with `1.2.0`.
* Build [rust-rt-minimal](https://github.com/AerialX/rust-rt-minimal) with `nightly-2015-03-12`.
* Build this project with with `nightly-2015-03-12`.

## License

MIT/Apache-2.0
