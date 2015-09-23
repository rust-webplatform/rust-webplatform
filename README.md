# rust-webplatform

```rust
let body = HtmlNode::query("body").unwrap();
body.html_set("<h1>HELLO FROM RUST</h1> <button>CLICK ME</button>");
let mut button = HtmlNode::query("button").unwrap();
button.on("click", || alert("WITNESS ME"));
```

Thanks to the high heavens for [tomaka](https://gist.github.com/tomaka/24c058db5ae31dfafb3f) and then [AerialX](https://github.com/AerialX) for making this possible.

## Running Rust in your Browser

To use `rust-webplatform` and Rust in your browser, you'll need a very particular setup. This is based off an older approach of cross-compilation; see [this thread](https://github.com/rust-lang/rfcs/issues/604) the status of merging Emscripten support into latest Rust.

**Note:** The version of Rust used is from March 12, which is a 1.0.0 beta. Some APIs are slightly changed and not yet stable in this version, and features in newer version of Rust will require modification.

* First, install [multirust](https://github.com/brson/multirust).
* Create a new folder. In that folder, save the [`setup.sh` file](https://github.com/tcr/rust-webplatform/blob/master/setup.sh).
* Run `source setup.sh`. It will download and install all the dependencies for the [cargo-build](https://github.com/AerialX/cargo-build) setup.
* Clone this repository into your folder, cd into it, and run `make`.

You should run `source setup.sh` in the root directory each time you have a new terminal session to make building this project work.

## License

MIT/Apache-2.0
