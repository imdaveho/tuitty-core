//! `tuitty` is a cross platform library that is meant for FFI.

use std::str;
use std::ffi::CStr;

mod tty;
pub use tty::{Tty, AsyncReader, SyncReader};


// pub struct Tui(Tty);

#[repr(C)]
pub struct Tuple {
    x: i16,
    y: i16,
}

impl From<(i16, i16)> for Tuple {
    fn from(tup: (i16, i16)) -> Tuple {
        Tuple { x: tup.0, y: tup.1 }
    }
}

impl From<Tuple> for (i16, i16) {
    fn from(tup: Tuple) -> (i16, i16) {
        (tup.x, tup.y)
    }
}


#[no_mangle]
pub extern fn tty() -> *mut Tty {
    Box::into_raw(Box::new(Tty::init()))
}

#[no_mangle]
pub extern fn tty_free(ptr: *mut Tty) {
    if ptr.is_null() { return }
    unsafe { Box::from_raw(ptr); }
}

#[no_mangle]
pub extern fn exit(ptr: *mut Tty) {
    let tty = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    tty.exit();
}

#[no_mangle]
pub extern fn size(ptr: *mut Tty) -> Tuple {
    let tty = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    tty.size().into()
}

#[no_mangle]
pub extern fn raw(ptr: *mut Tty) {
    let tty = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    tty.raw();
}

#[no_mangle]
pub extern fn cook(ptr: *mut Tty) {
    let tty = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    tty.cook();
}

#[no_mangle]
pub extern fn enable_mouse(ptr: *mut Tty) {
    let tty = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    tty.enable_mouse();
}

#[no_mangle]
pub extern fn disable_mouse(ptr: *mut Tty) {
    let tty = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    tty.disable_mouse();
}

// TODO: confirm how to send char values through the FFI boundary
#[no_mangle]
pub extern fn read_char(ptr: *mut Tty) -> u32 {
    let tty = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    tty.read_char() as u32
}

#[no_mangle]
pub extern fn read_sync(ptr: *mut Tty) -> *mut SyncReader {
    let tty = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    Box::into_raw(Box::new(tty.read_sync()))
}

#[no_mangle]
pub extern fn sync_free(ptr: *mut SyncReader) {
    if ptr.is_null() { return }
    unsafe { Box::from_raw(ptr); }
}

#[no_mangle]
pub extern fn read_async(ptr: *mut Tty) -> *mut AsyncReader {
    let tty = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    Box::into_raw(Box::new(tty.read_async()))
}

// TODO: confirm if u8's need to have special considerations when going through
// the FFI boundary
#[no_mangle]
pub extern fn read_until_async(ptr: *mut Tty, d: u8) -> *mut AsyncReader {
    let tty = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    Box::into_raw(Box::new(tty.read_until_async(d)))
}

#[no_mangle]
pub extern fn async_free(ptr: *mut AsyncReader) {
    if ptr.is_null() { return }
    unsafe { Box::from_raw(ptr); }
}

// TODO: confirm if strings passed through FFI boundary are i8 or u8
#[no_mangle]
pub extern fn clear(ptr: *mut Tty, s: *const i8) {
    let c_str = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s)
    };

    let method = c_str.to_str().unwrap();

    let tty = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    tty.clear(method);
}

#[no_mangle]
pub extern fn resize(ptr: *mut Tty, w: i16, h: i16) {
    let tty = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    tty.resize(w, h);
}

#[no_mangle]
pub extern fn switch(ptr: *mut Tty) {
    let tty = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    tty.switch();
}