use libc;
use std::ffi::CString;
use std::mem;
use std::mem::forget;

extern {
    fn rs_query(s: *const libc::c_char) -> libc::c_int;
    fn rs_html_set(i: libc::c_int, s: *const libc::c_char);
    fn rs_release(i: libc::c_int);
    fn rs_alert(s: *const libc::c_char);
    fn rs_on(i: libc::c_int, s: *const libc::c_char, cb: *const libc::c_void, cb2: extern fn(*const libc::c_void));
}

pub struct HtmlNode {
    id: libc::c_int,
}

extern fn rust_caller<F: Fn()>(a: *const libc::c_void) {
    let v:&F = unsafe { mem::transmute(a) };
    v();
}

impl HtmlNode {
    pub fn query(s: &str) -> Option<HtmlNode> {
        let id;
        unsafe {
            id = rs_query(CString::new(s).unwrap().as_ptr());
        }

        if id < 0 {
            None
        } else {
            Some(HtmlNode {
                id: id,
            })
        }
    }

    pub fn html_set(&self, s: &str) {
        unsafe {
            rs_html_set(self.id, CString::new(s).unwrap().as_ptr());
        }
    }

    pub fn on<F: Fn()>(&mut self, s: &str, f: F) {
        unsafe {
            let a = &f as *const _;
            forget(f);
            rs_on(self.id, CString::new(s).unwrap().as_ptr(), a as *const libc::c_void, rust_caller::<F>);
        }
    }
}

impl Drop for HtmlNode {
    fn drop(&mut self) {
        unsafe {
            rs_release(self.id);
        }
    }
}

pub fn alert(s: &str) {
    unsafe {
        rs_alert(CString::new(s).unwrap().as_ptr());
    }
}
