#![allow(dead_code, non_camel_case_types, non_snake_case)]

use libc;

mod ffi {
	use libc;

    pub type EM_BOOL = libc::c_int;
    pub type EM_UTF8 = libc::c_char;
    pub type EMSCRIPTEN_WEBGL_CONTEXT_HANDLE = libc::c_int;
    pub type EMSCRIPTEN_RESULT = libc::c_int;

    pub type em_webgl_context_callback = extern fn(libc::c_int, *const libc::c_void, *mut libc::c_void)
        -> EM_BOOL;

    #[repr(C)]
    pub struct EmscriptenWebGLContextAttributes {
        pub alpha: EM_BOOL,
        pub depth: EM_BOOL,
        pub stencil: EM_BOOL,
        pub antialias: EM_BOOL,
        pub premultipliedAlpha: EM_BOOL,
        pub preserveDrawingBuffer: EM_BOOL,
        pub preferLowPowerToHighPerformance: EM_BOOL,
        pub failIfMajorPerformanceCaveat: EM_BOOL,
        pub majorVersion: libc::c_int,
        pub minorVersion: libc::c_int,
        pub enableExtensionsByDefault: EM_BOOL,
    }

    // values for EMSCRIPTEN_RESULT
    pub const EMSCRIPTEN_RESULT_SUCCESS: libc::c_int = 0;
    pub const EMSCRIPTEN_RESULT_DEFERRED: libc::c_int = 1;
    pub const EMSCRIPTEN_RESULT_NOT_SUPPORTED: libc::c_int = -1;
    pub const EMSCRIPTEN_RESULT_FAILED_NOT_DEFERRED: libc::c_int = -2;
    pub const EMSCRIPTEN_RESULT_INVALID_TARGET: libc::c_int = -3;
    pub const EMSCRIPTEN_RESULT_UNKNOWN_TARGET: libc::c_int = -4;
    pub const EMSCRIPTEN_RESULT_INVALID_PARAM: libc::c_int = -5;
    pub const EMSCRIPTEN_RESULT_FAILED: libc::c_int = -6;
    pub const EMSCRIPTEN_RESULT_NO_DATA: libc::c_int = -7;

    extern {
        pub fn glClear(_: libc::c_int);
        pub fn glClearColor(_: libc::c_float, _: libc::c_float, _: libc::c_float, _: libc::c_float);
    }

    extern {
        pub fn emscripten_webgl_init_context_attributes(attributes: *mut EmscriptenWebGLContextAttributes);
        pub fn emscripten_webgl_create_context(target: *const libc::c_char,
            attributes: *const EmscriptenWebGLContextAttributes) -> EMSCRIPTEN_WEBGL_CONTEXT_HANDLE;

        pub fn emscripten_webgl_make_context_current(context: EMSCRIPTEN_WEBGL_CONTEXT_HANDLE)
        -> EMSCRIPTEN_RESULT;

        pub fn emscripten_webgl_get_current_context() -> EMSCRIPTEN_WEBGL_CONTEXT_HANDLE;

        pub fn emscripten_webgl_destroy_context(context: EMSCRIPTEN_WEBGL_CONTEXT_HANDLE)
            -> EMSCRIPTEN_RESULT;

        pub fn emscripten_webgl_enable_extension(context: EMSCRIPTEN_WEBGL_CONTEXT_HANDLE,
            extension: *const libc::c_char) -> EM_BOOL;

        pub fn emscripten_set_webglcontextlost_callback(target: *const libc::c_char,
            userData: *mut libc::c_void, useCapture: EM_BOOL, callback: em_webgl_context_callback)
            -> EMSCRIPTEN_RESULT;
        pub fn emscripten_set_webglcontextrestored_callback(target: *const libc::c_char,
            userData: *mut libc::c_void, useCapture: EM_BOOL, callback: em_webgl_context_callback)
            -> EMSCRIPTEN_RESULT;

        pub fn emscripten_is_webgl_context_lost(target: *const libc::c_char) -> EM_BOOL;

        // note: this function is not documented but is used by the ports of glfw, SDL and EGL
        pub fn emscripten_GetProcAddress(name: *const libc::c_char) -> *const libc::c_void;


        pub fn emscripten_request_fullscreen(target: *const libc::c_char,
            deferUntilInEventHandler: EM_BOOL) -> EMSCRIPTEN_RESULT;

        pub fn emscripten_exit_fullscreen() -> EMSCRIPTEN_RESULT;

        pub fn emscripten_set_element_css_size(target: *const libc::c_char, width: libc::c_double,
            height: libc::c_double) -> EMSCRIPTEN_RESULT;

        pub fn emscripten_get_element_css_size(target: *const libc::c_char, width: *mut libc::c_double,
            height: *mut libc::c_double) -> EMSCRIPTEN_RESULT;
    }
}

pub struct Window {
    context: ffi::EMSCRIPTEN_WEBGL_CONTEXT_HANDLE,
}

pub struct MonitorID;

pub fn get_available_monitors() -> Vec<MonitorID> {
    vec![MonitorID]
}

pub fn get_primary_monitor() -> MonitorID {
    MonitorID
}

impl MonitorID {
    pub fn get_name(&self) -> Option<String> {
        Some("Canvas".to_string())
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        unimplemented!()
    }
}

impl Window {
    pub fn new() -> Result<Window, String> {
        // getting the default values of attributes
        let attributes = unsafe {
            use std::mem;
            let mut attributes: ffi::EmscriptenWebGLContextAttributes = mem::uninitialized();
            ffi::emscripten_webgl_init_context_attributes(&mut attributes);
            attributes
        };

        // creating the context
        let context = unsafe {
            use std::ptr;
            let context = ffi::emscripten_webgl_create_context(ptr::null(), &attributes);
            context
        };

        // TODO: emscripten_set_webglcontextrestored_callback

        Ok(Window {
            context: context
        })
    }

    pub fn is_closed(&self) -> bool {
        use std::ptr;
        unsafe { ffi::emscripten_is_webgl_context_lost(ptr::null()) != 0 }
    }

    pub fn set_title(&self, _title: &str) {
    }

    pub fn get_position(&self) -> Option<(isize, isize)> {
        Some((0, 0))
    }

    pub fn set_position(&self, _: isize, _: isize) {
    }

    pub fn get_inner_size(&self) -> Option<(usize, usize)> {
        unsafe {
            use std::{mem, ptr};
            let mut width = mem::uninitialized();
            let mut height = mem::uninitialized();

            if ffi::emscripten_get_element_css_size(ptr::null(), &mut width, &mut height)
                != ffi::EMSCRIPTEN_RESULT_SUCCESS
            {
                None
            } else {
                Some((width as usize, height as usize))
            }
        }
    }

    pub fn get_outer_size(&self) -> Option<(usize, usize)> {
        self.get_inner_size()
    }

    pub fn set_inner_size(&self, width: usize, height: usize) {
        unsafe {
            use std::ptr;
            ffi::emscripten_set_element_css_size(ptr::null(), width as libc::c_double, height
                as libc::c_double);
        }
    }

    pub unsafe fn make_current(&self) {
        // TOOD: check if == EMSCRIPTEN_RESULT
        ffi::emscripten_webgl_make_context_current(self.context);
    }

    pub fn swap_buffers(&self) {
        // there is no need to swap buffers in webgl
        // the browser avoids drawing our buffer as long as we continue to execute code
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            ffi::emscripten_exit_fullscreen();
            ffi::emscripten_webgl_destroy_context(self.context);
        }
    }
}

fn error_to_str(code: ffi::EMSCRIPTEN_RESULT) -> &'static str {
    match code {
        ffi::EMSCRIPTEN_RESULT_SUCCESS | ffi::EMSCRIPTEN_RESULT_DEFERRED
            => "Internal error in the library (success detected as failure)",

        ffi::EMSCRIPTEN_RESULT_NOT_SUPPORTED => "Not supported",
        ffi::EMSCRIPTEN_RESULT_FAILED_NOT_DEFERRED => "Failed not deferred",
        ffi::EMSCRIPTEN_RESULT_INVALID_TARGET => "Invalid target",
        ffi::EMSCRIPTEN_RESULT_UNKNOWN_TARGET => "Unknown target",
        ffi::EMSCRIPTEN_RESULT_INVALID_PARAM => "Invalid parameter",
        ffi::EMSCRIPTEN_RESULT_FAILED => "Failed",
        ffi::EMSCRIPTEN_RESULT_NO_DATA => "No data",

        _ => "Undocumented error"
    }
}

pub fn emmain() -> bool {
    let win = match Window::new() {
        Ok(w) => w,
        Err(_) => { return false; }
    };

    unsafe {
        win.make_current();

        ffi::glClearColor(1.0, 0.0, 0.0, 1.0);
        ffi::glClear(0x4000);
    }

    true
}
