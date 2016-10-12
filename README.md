# rust-webplatform

A Rust library for use with emscripten to access the DOM.

[Read the documentation](http://docs.rs/webplatform), read [brson's post on how
Rust works with emscripten](https://users.rust-lang.org/t/compiling-to-the-web-with-rust-and-emscripten/7627),
or see an example app with [rust-todomvc](http://github.com/tcr/rust-todomvc).

```rust
extern crate webplatform;

use webplatform::HtmlNode;

let body = HtmlNode::query("body").unwrap();
body.html_set("<h1>HELLO FROM RUST</h1> <button>CLICK ME</button>");
let mut button = HtmlNode::query("button").unwrap();
button.on("click", || alert("WITNESS ME"));
```

Used with `cargo build --target=asmjs-unknown-emscripten`.

## License

MIT or Apache-2.0, at your option.
