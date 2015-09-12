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
how to get the environment set up.

    cargo-build --sysroot some/rust/sysroot --target i386-unknown-emscripten --emit em-html
