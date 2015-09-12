extern crate libc;

mod webplatform;

use webplatform::{HtmlNode, alert};

fn main() {
    let body = HtmlNode::query("body").unwrap();
    body.html_set("<h1>HELLO FROM RUST</h1> <button>CLICK ME</button>");
    let mut button = HtmlNode::query("button").unwrap();
    button.on("click", || alert("WITNESS ME"));
}
