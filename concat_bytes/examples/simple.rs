#![feature(plugin)]
#![plugin(concat_bytes)]

pub static mut NAME: *const u8 = concat_bytes!("KokaKiwi", b'\0') as *const u8;

fn main() {
}
