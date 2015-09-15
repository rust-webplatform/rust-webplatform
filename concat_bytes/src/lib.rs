#![feature(io)]
#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]
#![feature(collections)]
extern crate syntax;
extern crate rustc;

mod expand;

use rustc::plugin::Registry;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("concat_bytes", expand::expand_syntax_ext);
}
