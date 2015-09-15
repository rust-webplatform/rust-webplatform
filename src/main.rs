#![feature(plugin)]
#![feature(alloc)]
#![plugin(concat_bytes)]

extern crate libc;

#[macro_use]
mod webplatform;

use webplatform::{HtmlNode, alert, spin};

fn main() {
	let mut wp = webplatform::init();

    let mut body = wp.query("body").unwrap();

    let hr = wp.create("hr").unwrap();
    body.append(&hr);

    body.html_prepend("<h1>HELLO FROM RUST</h1>");
    body.html_append("<button>CLICK ME</button>");
    let mut button = wp.query("button").unwrap();
    
    // button.on("click", || {
    // 	body.prop_set_str("bgColor", "blue");
    // });

	let mut b2 = body.clone();
	wp.refs.push(Box::new(move || {
    	b2.prop_set_str("bgColor", "blue");
    }));

    println!("This should be blue: {:?}", body.prop_get_str("bgColor"));
    println!("Width?: {:?}", body.prop_get_i32("clientWidth"));

    spin();

    println!("NO CALLING ME.");
}
