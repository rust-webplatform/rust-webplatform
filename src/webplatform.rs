#![allow(dead_code)]

use libc;
use std::ffi::{CString, CStr};
use std::{mem, fmt};
use std::mem::forget;
use std::str;
use std::borrow::ToOwned;
use std::cell::RefCell;
use std::rc::{Rc, is_unique};

trait Interop {
    fn as_int(self, _:&mut Vec<CString>) -> libc::c_int;
}

impl Interop for i32 {
    fn as_int(self, _:&mut Vec<CString>) -> libc::c_int {
        return self;
    }
}

impl<'a> Interop for &'a str {
    fn as_int(self, arena:&mut Vec<CString>) -> libc::c_int {
        let c = CString::new(self).unwrap();
        let ret = c.as_ptr() as libc::c_int;
        arena.push(c);
        return ret;
    }
}

impl<'a> Interop for *const libc::c_void {
    fn as_int(self, _:&mut Vec<CString>) -> libc::c_int {
        return self as libc::c_int;
    }
}

macro_rules! js {
    ( ($( $x:expr ),*) $y:expr ) => {
        unsafe {
            use webplatform;
            let mut arena:Vec<CString> = Vec::new();
            webplatform::emscripten_asm_const_int(concat_bytes!($y, b"\0").as_ptr() as *const libc::c_char, $(Interop::as_int($x, &mut arena)),*)
        }
    };
    ( $y:expr ) => {
        unsafe {
            use webplatform;
            webplatform::emscripten_asm_const_int(concat_bytes!($y, b"\0").as_ptr() as *const libc::c_char)
        }
    };
}

extern {
    pub fn emscripten_asm_const(s: *const libc::c_char);
    pub fn emscripten_asm_const_int(s: *const libc::c_char, ...) -> libc::c_int;
    pub fn emscripten_pause_main_loop();
    pub fn emscripten_set_main_loop(m: extern fn(), fps: libc::c_int, infinite: libc::c_int);
}

#[derive(Clone)]
pub struct HtmlNode {
    id: libc::c_int,
    rc: Rc<()>,
}

impl fmt::Debug for HtmlNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HtmlNode({:?})", self.id)
    }
}

impl Drop for HtmlNode {
    fn drop(&mut self) {
        println!("dropping HTML NODE {:?} {:?}", self.id, is_unique(&self.rc));
    }
}

extern fn rust_caller<F: FnMut()>(a: *const libc::c_void) {
    let v:&mut F = unsafe { mem::transmute(a) };
    v();
}

impl HtmlNode {
    pub fn html_set(&mut self, s: &str) {
        js! { (self.id, s) br#"
            WEBPLATFORM.rs_refs[$0].innerHTML = UTF8ToString($1);
        "#};
    }
    
    pub fn prop_set_i32(&mut self, s: &str, v: i32) {
        js! { (self.id, s, v) br#"
            WEBPLATFORM.rs_refs[$0][UTF8ToString($1)] = $2;
        "#};
    }
    
    pub fn prop_set_str(&mut self, s: &str, v: &str) {
        js! { (self.id, s, v) br#"
            console.log($0)
            WEBPLATFORM.rs_refs[$0][UTF8ToString($1)] = UTF8ToString($2);
        "#};
    }
    
    pub fn prop_get_i32(&self, s: &str) -> i32 {
        return js! { (self.id, s) concat_bytes!(br#"
            return WEBPLATFORM.rs_refs[$0][UTF8ToString($1)]
        "#)};
    }
    
    pub fn prop_get_str(&self, s: &str) -> String {
        let a = js! { (self.id, s) concat_bytes!(br#"
            return allocate(intArrayFromString(WEBPLATFORM.rs_refs[$0][UTF8ToString($1)]), 'i8', ALLOC_STACK);
        "#)};
        unsafe {
            str::from_utf8(CStr::from_ptr(a as *const libc::c_char).to_bytes()).unwrap().to_owned()
        }
    }

    pub fn append(&mut self, s: &HtmlNode) {
        js! { (self.id, s.id) br#"
            WEBPLATFORM.rs_refs[$0].appendChild(WEBPLATFORM.rs_refs[$1]);
        "#};
    }

    pub fn html_append(&mut self, s: &str) {
        js! { (self.id, s) br#"
            WEBPLATFORM.rs_refs[$0].insertAdjacentHTML('beforeEnd', UTF8ToString($1));
        "#};
    }

    pub fn html_prepend(&mut self, s: &str) {
        js! { (self.id, s) br#"
            WEBPLATFORM.rs_refs[$0].insertAdjacentHTML('afterBegin', UTF8ToString($1));
        "#};
    }

    // pub fn on<'a, F: FnMut() + Clone>(&'a mut self, s: &str, f: F) {
    //     unsafe {
    //         let a = &f as *const _;
    //         // forget(f);
    //         js! { (self.id, s, a as *const libc::c_void, rust_caller::<F> as *const libc::c_void) br#"
    //             WEBPLATFORM.rs_refs[$0].addEventListener(UTF8ToString($1), function () {
    //                 Runtime.dynCall('vi', $3, [$2]);
    //             }, false);
    //         "#};
    //         self.wp.borrow_mut().refs.push(Box::new(f.clone()));
    //     }
    // }
}

pub fn alert(s: &str) {
    js! { (s) br#"
        alert(UTF8ToString($0));
    "#};
}

pub struct WebPlatform {
    pub refs: Vec<Box<FnMut()>>,
}

impl WebPlatform {
    pub fn create(&self, s: &str) -> Option<HtmlNode> {
        let id = js! { (s) br#"
            var value = document.createElement(UTF8ToString($0));
            if (!value) {
                return -1;
            }
            return WEBPLATFORM.rs_refs.push(value) - 1;
        "#};

        if id < 0 {
            None
        } else {
            Some(HtmlNode {
                id: id,
                rc: Rc::new(()),
            })
        }
    }

    pub fn query(&self, s: &str) -> Option<HtmlNode> {
        let id = js! { (s) br#"
            var value = document.querySelector(UTF8ToString($0));
            if (!value) {
                return -1;
            }
            return WEBPLATFORM.rs_refs.push(value) - 1;
        "#};

        if id < 0 {
            None
        } else {
            Some(HtmlNode {
                id: id,
                rc: Rc::new(()),
            })
        }
    }
}

pub fn init() -> WebPlatform {
    js! { br#"
        this.WEBPLATFORM = {
            rs_refs: [],
        };
    "#};
    WebPlatform {
        refs: Vec::new()
    }
}

extern fn leavemebe() {
    unsafe {
        emscripten_pause_main_loop();
    }
}

pub fn spin() {
    unsafe {
        emscripten_set_main_loop(leavemebe, 0, 1);
        
    }
}
