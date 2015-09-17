#![feature(plugin)]
#![feature(unsafe_destructor)]
#![plugin(concat_bytes)]

#[macro_use] extern crate webplatform;

use std::borrow::ToOwned;

fn main() {
    let document = webplatform::init();
    {
        let body = document.element_query("body").unwrap();

        let hr = document.element_create("hr").unwrap();
        body.append(&hr);

        body.html_prepend("<h1>HELLO FROM RUST</h1>");
        body.html_append("<button>CLICK ME</button>");

        let mut button = document.element_query("button").unwrap();

        let bodyref = body.root_ref();
        let bodyref2 = body.root_ref();
    	button.on("click", move || {
            bodyref2.prop_set_str("bgColor", "blue");
        });
        
        println!("This should be blue: {:?}", bodyref.prop_get_str("bgColor"));
        println!("Width?: {:?}", bodyref.prop_get_i32("clientWidth"));
    
        webplatform::spin();
    }

    println!("NO CALLING ME.");
}
