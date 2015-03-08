#![feature(libc)]

extern crate libc;
extern crate "rustc-serialize" as rustc_serialize;

mod test;
use test::emmain;
use rustc_serialize::Encodable;
use rustc_serialize::json::encode;

fn main() {
    println!("hello?");
    emmain();
    println!("webgl stuff done, time for a fibonacci sequence...");

    let fib = Fib { curr: 1, next: 1 };
    let fib : Vec<u32> = fib.take(20).collect();
    for x in fib {
        println!("fib {}", x);
    }

    println!("Test out rustc-serialize just to see if dependencies work...");
    #[derive(RustcEncodable)]
    struct SerializeTest {
        some_string: String,
        some_int: Option<isize>,
    };

    println!("{}", encode(&SerializeTest { some_string: "Hey there".to_string(), some_int: Some(0xf001) }).unwrap());
}

struct Fib {
    curr: u32,
    next: u32,
}

impl Iterator for Fib {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}
