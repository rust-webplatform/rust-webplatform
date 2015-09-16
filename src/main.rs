#![feature(plugin)]
#![feature(unsafe_destructor)]
#![plugin(concat_bytes)]

extern crate libc;

#[macro_use]
mod webplatform;

use std::borrow::ToOwned;

fn main() {
    let document = webplatform::init();
    {
        let body = document.element_query("body").unwrap();

     //    let hr = document.element_create("hr").unwrap();
     //    body.append(&hr);

     //    body.html_prepend("<h1>HELLO FROM RUST</h1>");
     //    body.html_append("<button>CLICK ME</button>");

     //    let mut button = document.element_query("button").unwrap();

     //    let bodyref = body.root_ref();
     //    let bodyref2 = body.root_ref();
    	// button.on("click", move || {
     //        bodyref2.prop_set_str("bgColor", "blue");
     //    });
        
     //    println!("This should be blue: {:?}", bodyref.prop_get_str("bgColor"));
     //    println!("Width?: {:?}", bodyref.prop_get_i32("clientWidth"));

        body.html_set(r##"
<title>VanillaJS • TodoMVC</title>
<link rel="stylesheet" href="/base.css">
<link rel="stylesheet" href="/index.css">
<section class="todoapp">
    <header class="header">
        <h1>todos</h1>
        <input class="new-todo" placeholder="What needs to be done?" autofocus>
    </header>
    <section class="main">
        <input class="toggle-all" type="checkbox">
        <label for="toggle-all">Mark all as complete</label>
        <ul class="todo-list"></ul>
    </section>
    <footer class="footer">
        <span class="todo-count"></span>
        <ul class="filters">
            <li>
                <a href="#/" class="selected">All</a>
            </li>
            <li>
                <a href="#/active">Active</a>
            </li>
            <li>
                <a href="#/completed">Completed</a>
            </li>
        </ul>
        <button class="clear-completed">Clear completed</button>
    </footer>
</section>
<footer class="info">
    <p>Double-click to edit a todo</p>
    <p>Created by <a href="http://twitter.com/oscargodson">Oscar Godson</a></p>
    <p>Refactored by <a href="https://github.com/cburgmer">Christoph Burgmer</a></p>
    <p>Part of <a href="http://todomvc.com">TodoMVC</a></p>
</footer>
        "##);

        let todo_new = document.element_query(".new-todo").unwrap();
        let list = document.element_query(".todo-list").unwrap();

        println!("todo_new {:?}, list {:?}", todo_new, list);

        let t1 = todo_new.root_ref();
        println!("ok {:?}", t1);
        todo_new.on("change", move || {
            println!("okno {:?}", t1);
            let value = t1.prop_get_str("value");
            list.html_append(&(r#"
<li data-id="{{id}}" class="{{completed}}">'
  <div class="view">
    <input class="toggle" type="checkbox" {{checked}}>
    <label>"#.to_owned() + &value + r#"</label>
    <button class="destroy"></button>
  </div>'
</li>"#));
        });
    
        webplatform::spin();
    }

    println!("NO CALLING ME.");
}
