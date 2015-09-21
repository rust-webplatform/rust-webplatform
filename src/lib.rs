#![feature(plugin)]
#![feature(unsafe_destructor)]
#![plugin(concat_bytes)]

extern crate libc;

use std::ffi::{CString, CStr};
use std::{mem, fmt};
use std::str;
use std::borrow::ToOwned;
use std::ops::Deref;
use std::cell::RefCell;
use std::clone::Clone;
use std::rc::Rc;
use std::collections::HashSet;
use std::char;
use std::iter::IntoIterator;

mod webplatform {
    pub use emscripten_asm_const;
    pub use emscripten_asm_const_int;
}

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
        {
            let mut arena:Vec<CString> = Vec::new();
            unsafe { ::webplatform::emscripten_asm_const_int(concat_bytes!($y, b"\0").as_ptr() as *const libc::c_char, $(Interop::as_int($x, &mut arena)),*) }
        }
    };
    ( $y:expr ) => {
        {
            unsafe { ::webplatform::emscripten_asm_const_int(concat_bytes!($y, b"\0").as_ptr() as *const libc::c_char) }
        }
    };
}

extern {
    pub fn emscripten_asm_const(s: *const libc::c_char);
    pub fn emscripten_asm_const_int(s: *const libc::c_char, ...) -> libc::c_int;
    pub fn emscripten_pause_main_loop();
    pub fn emscripten_set_main_loop(m: extern fn(), fps: libc::c_int, infinite: libc::c_int);
}

pub struct HtmlNode<'a> {
    id: libc::c_int,
    doc: *const Document<'a>,
}

impl<'a> fmt::Debug for HtmlNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HtmlNode({:?})", self.id)
    }
}

#[unsafe_destructor]
impl<'a> Drop for HtmlNode<'a> {
    fn drop(&mut self) {
        println!("dropping HTML NODE {:?}", self.id);
    }
}

pub struct JSRef<'a> {
    ptr: *const HtmlNode<'a>,
}

impl<'a> fmt::Debug for JSRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "JSRef(HtmlNode({:?}))", self.id)
    }
}

impl<'a> Clone for JSRef<'a> {
    fn clone(&self) -> JSRef<'a> {
        JSRef {
            ptr: self.ptr,
        }
    }
}

impl<'a> HtmlNode<'a> {
    pub fn root_ref(&self) -> JSRef<'a> {
        JSRef {
            ptr: &*self,
        }
    }
}

impl<'a> Deref for JSRef<'a> {
    type Target = HtmlNode<'a>;

    fn deref(&self) -> &HtmlNode<'a> {
        unsafe {
            &*self.ptr
        }
    }
}

pub struct Event<'a> {
    pub target: Option<HtmlNode<'a>>
}

extern fn rust_caller<F: FnMut(Event)>(a: *const libc::c_void, docptr: *const libc::c_void, id: i32) {
    let v:&mut F = unsafe { mem::transmute(a) };
    v(Event {
        target: if id == -1 {
            None
        } else {
            Some(HtmlNode {
                id: id,
                doc: unsafe { mem::transmute(docptr) },
            })
        }
        // target: None,
    });
}

impl<'a> HtmlNode<'a> {
    pub fn tagname(&self) -> String {
        let a = js! { (self.id) br#"
            var str = WEBPLATFORM.rs_refs[$0].tagName.toLowerCase();
            return allocate(intArrayFromString(str), 'i8', ALLOC_STACK);
        "#};
        unsafe {
            str::from_utf8(CStr::from_ptr(a as *const libc::c_char).to_bytes()).unwrap().to_owned()
        }
    }

    pub fn focus(&self) {
        js! { (self.id) br#"
            WEBPLATFORM.rs_refs[$0].focus();
        "#};
    }

    pub fn html_set(&self, s: &str) {
        js! { (self.id, s) br#"
            WEBPLATFORM.rs_refs[$0].innerHTML = UTF8ToString($1);
        "#};
    }

    pub fn html_get(&self) -> String {
        let a = js! { (self.id) br#"
            return allocate(intArrayFromString(WEBPLATFORM.rs_refs[$0].innerHTML), 'i8', ALLOC_STACK);
        "#};
        unsafe {
            str::from_utf8(CStr::from_ptr(a as *const libc::c_char).to_bytes()).unwrap().to_owned()
        }
    }

    pub fn class_get(&self) -> HashSet<String> {
        let a = js! { (self.id) br#"
            return allocate(intArrayFromString(WEBPLATFORM.rs_refs[$0].className), 'i8', ALLOC_STACK);
        "#};
        let class = unsafe {
            str::from_utf8(CStr::from_ptr(a as *const libc::c_char).to_bytes()).unwrap().to_owned()
        };
        class.trim().split(char::is_whitespace).map(|x| x.to_string()).collect()
    }

    pub fn class_add(&self, s: &str) {
        js! { (self.id, s) br#"
            WEBPLATFORM.rs_refs[$0].classList.add(UTF8ToString($1));
        "#};
    }

    pub fn class_remove(&self, s: &str) {
        js! { (self.id, s) br#"
            WEBPLATFORM.rs_refs[$0].classList.remove(UTF8ToString($1));
        "#};
    }

    pub fn parent(&self) -> Option<HtmlNode<'a>> {
        let id = js! { (self.id) br#"
            var value = WEBPLATFORM.rs_refs[$0].parentNode;
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
                doc: self.doc,
            })
        }
    }
    
    pub fn data_set(&self, s: &str, v: &str) {
        js! { (self.id, s, v) br#"
            WEBPLATFORM.rs_refs[$0].dataset[UTF8ToString($1)] = UTF8ToString($2);
        "#};
    }

    pub fn data_get(&self, s: &str) -> Option<String> {
        let a = js! { (self.id, s) br#"
            var str = WEBPLATFORM.rs_refs[$0].dataset[UTF8ToString($1)]
            if (str == null) return -1;
            return allocate(intArrayFromString(str), 'i8', ALLOC_STACK);
        "#};
        if a == -1 {
            None
        } else {
            Some(unsafe {
                str::from_utf8(CStr::from_ptr(a as *const libc::c_char).to_bytes()).unwrap().to_owned()
            })
        }
    }
    
    pub fn style_set_str(&self, s: &str, v: &str) {
        js! { (self.id, s, v) br#"
            WEBPLATFORM.rs_refs[$0].style[UTF8ToString($1)] = UTF8ToString($2);
        "#};
    }

    pub fn style_get_str(&self, s: &str) -> String {
        let a = js! { (self.id, s) br#"
            return allocate(intArrayFromString(WEBPLATFORM.rs_refs[$0].style[UTF8ToString($1)]), 'i8', ALLOC_STACK);
        "#};
        unsafe {
            str::from_utf8(CStr::from_ptr(a as *const libc::c_char).to_bytes()).unwrap().to_owned()
        }
    }
    
    pub fn prop_set_i32(&self, s: &str, v: i32) {
        js! { (self.id, s, v) br#"
            WEBPLATFORM.rs_refs[$0][UTF8ToString($1)] = $2;
        "#};
    }
    
    pub fn prop_set_str(&self, s: &str, v: &str) {
        js! { (self.id, s, v) br#"
            WEBPLATFORM.rs_refs[$0][UTF8ToString($1)] = UTF8ToString($2);
        "#};
    }
    
    pub fn prop_get_i32(&self, s: &str) -> i32 {
        return js! { (self.id, s) br#"
            return Number(WEBPLATFORM.rs_refs[$0][UTF8ToString($1)])
        "#};
    }
    
    pub fn prop_get_str(&self, s: &str) -> String {
        let a = js! { (self.id, s) br#"
            return allocate(intArrayFromString(WEBPLATFORM.rs_refs[$0][UTF8ToString($1)]), 'i8', ALLOC_STACK);
        "#};
        unsafe {
            str::from_utf8(CStr::from_ptr(a as *const libc::c_char).to_bytes()).unwrap().to_owned()
        }
    }

    pub fn append(&self, s: &HtmlNode) {
        js! { (self.id, s.id) br#"
            WEBPLATFORM.rs_refs[$0].appendChild(WEBPLATFORM.rs_refs[$1]);
        "#};
    }

    pub fn html_append(&self, s: &str) {
        js! { (self.id, s) br#"
            WEBPLATFORM.rs_refs[$0].insertAdjacentHTML('beforeEnd', UTF8ToString($1));
        "#};
    }

    pub fn html_prepend(&self, s: &str) {
        js! { (self.id, s) br#"
            WEBPLATFORM.rs_refs[$0].insertAdjacentHTML('afterBegin', UTF8ToString($1));
        "#};
    }

    pub fn on<F: FnMut(Event) + 'a>(&self, s: &str, f: F) {
        unsafe {
            let b = Box::new(f);
            let a = &*b as *const _;
            js! { (self.id, s, a as *const libc::c_void,
                rust_caller::<F> as *const libc::c_void,
                self.doc as *const libc::c_void)
                br#"
                WEBPLATFORM.rs_refs[$0].addEventListener(UTF8ToString($1), function (e) {
                    Runtime.dynCall('viii', $3, [$2, $4, e.target ? WEBPLATFORM.rs_refs.push(e.target) - 1 : -1]);
                }, false);
            "#};
            (&*self.doc).refs.borrow_mut().push(b);
        }
    }

    pub fn captured_on<F: FnMut(Event) + 'a>(&self, s: &str, f: F) {
        unsafe {
            let b = Box::new(f);
            let a = &*b as *const _;
            js! { (self.id, s, a as *const libc::c_void,
                rust_caller::<F> as *const libc::c_void,
                self.doc as *const libc::c_void)
                br#"
                WEBPLATFORM.rs_refs[$0].addEventListener(UTF8ToString($1), function (e) {
                    Runtime.dynCall('viii', $3, [$2, $4, e.target ? WEBPLATFORM.rs_refs.push(e.target) - 1 : -1]);
                }, true);
            "#};
            (&*self.doc).refs.borrow_mut().push(b);
        }
    }

    pub fn remove_self(&self) {
        js! { (self.id) br#"
            var s = WEBPLATFORM.rs_refs[$0];
            s.parentNode.removeChild(s);
        "#};
    }
}

pub fn alert(s: &str) {
    js! { (s) br#"
        alert(UTF8ToString($0));
    "#};
}

pub struct Document<'a> {
    refs: Rc<RefCell<Vec<Box<FnMut(Event<'a>) + 'a>>>>,
}

impl<'a> Document<'a> {
    pub fn element_create<'b>(&'b self, s: &str) -> Option<HtmlNode<'a>> {
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
                doc: &*self,
            })
        }
    }

    pub fn location_hash_get(&self) -> String {
        let a = js! { concat_bytes!(br#"
            return allocate(intArrayFromString(window.location.hash), 'i8', ALLOC_STACK);
        "#)};
        unsafe {
            str::from_utf8(CStr::from_ptr(a as *const libc::c_char).to_bytes()).unwrap().to_owned()
        }
    }

    pub fn on<F: FnMut(Event) + 'a>(&self, s: &str, f: F) {
        unsafe {
            let b = Box::new(f);
            let a = &*b as *const _;
            js! { (0, s, a as *const libc::c_void,
                rust_caller::<F> as *const libc::c_void,
                &*self as *const _ as *const libc::c_void)
                br#"
                window.addEventListener(UTF8ToString($1), function (e) {
                    Runtime.dynCall('viii', $3, [$2, $4, e.target ? WEBPLATFORM.rs_refs.push(e.target) - 1 : -1]);
                }, false);
            "#};
            self.refs.borrow_mut().push(b);
        }
    }

    pub fn element_query<'b>(&'b self, s: &str) -> Option<HtmlNode<'a>> {
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
                doc: self,
            })
        }
    }
}

pub struct LocalStorageInterface;

pub struct LocalStorageIterator {
    index: i32,
}

impl LocalStorageInterface {
    pub fn len(&self) -> i32 {
        js! { br#"
            return window.localStorage.length;
        "#}
    }

    pub fn clear(&self) {
        js! { br#"
            window.localStorage.clear();
        "#};
    }

    pub fn remove(&self, s: &str) {
        js! { (s) br#"
            window.localStorage.removeItem(UTF8ToString($0));
        "#};
    }

    pub fn set(&self, s: &str, v: &str) {
        js! { (s, v) br#"
            window.localStorage.setItem(UTF8ToString($0), UTF8ToString($1));
        "#};
    }

    pub fn get(&self, name: &str) -> Option<String> {
        let a = js! { (name) br#"
            var str = window.localStorage.getItem(UTF8ToString($0));
            if (str == null) {
                return -1;
            }
            return allocate(intArrayFromString(str), 'i8', ALLOC_STACK);
        "# };
        if a == -1 {
            None
        } else {
            Some(unsafe {
                str::from_utf8(CStr::from_ptr(a as *const libc::c_char).to_bytes()).unwrap().to_owned()
            })
        }
    }

    pub fn key(&self, index: i32) -> String {
        let a = js! { (index) br#"
            var key = window.localStorage.key($0);
            return allocate(intArrayFromString(str), 'i8', ALLOC_STACK);
        "# };
        unsafe {
            str::from_utf8(CStr::from_ptr(a as *const libc::c_char).to_bytes()).unwrap().to_owned()
        }
    }
}

impl IntoIterator for LocalStorageInterface {
    type Item = String;
    type IntoIter = LocalStorageIterator;

    fn into_iter(self) -> LocalStorageIterator {
        LocalStorageIterator { index: 0 }
    }
}

impl Iterator for LocalStorageIterator {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        if self.index >= LocalStorage.len() {
            None
        } else {
            LocalStorage.get(&LocalStorage.key(self.index))
        }
    }
}

#[allow(non_upper_case_globals)]
pub const LocalStorage: LocalStorageInterface = LocalStorageInterface;

pub fn init<'a>() -> Document<'a> {
    js! { br#"
        this.WEBPLATFORM || (this.WEBPLATFORM = {
            rs_refs: [],
        });
    "#};
    Document {
        refs: Rc::new(RefCell::new(Vec::new())),
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

#[no_mangle]
pub extern "C" fn syscall(a: i32) -> i32 {
    if a == 355 {
        return 55
    }
    return -1
}
