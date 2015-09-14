# rust-webplatform

```rust
let body = HtmlNode::query("body").unwrap();
body.html_set("<h1>HELLO FROM RUST</h1> <button>CLICK ME</button>");
let mut button = HtmlNode::query("button").unwrap();
button.on("click", || alert("WITNESS ME"));
```

Thanks to the high heavens for [tomaka](https://gist.github.com/tomaka/24c058db5ae31dfafb3f) and then [AerialX](https://github.com/AerialX) for making this possible.

## Building on Your Machine

First install [multirust](https://github.com/brson/multirust).

Create a new folder. In that folder, save the `setup.sh` file, then run `source setup.sh`. It will download and install all the dependencies for the [cargo-build](https://github.com/AerialX/cargo-build) setup.

Then clone this repository, cd into it, and run `make`. Run `source setup.sh` in your root directory each time you want to set up the environment.

## License

MIT/Apache-2.0
