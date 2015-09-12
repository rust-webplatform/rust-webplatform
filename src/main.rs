extern crate libc;

mod webplatform;

use webplatform::{HtmlNode, alert};

fn main() {
    let body = HtmlNode::query("body").unwrap();

    let hr = HtmlNode::create("hr").unwrap();
    body.append(&hr);

    body.html_prepend("<h1>HELLO FROM RUST</h1>");
    body.html_append("<button>CLICK ME</button>");
    let mut button = HtmlNode::query("button").unwrap();
    button.on("click", || alert("WITNESS ME"));
}
