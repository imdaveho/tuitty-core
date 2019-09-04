//! `tuitty` is a cross platform library that is meant for FFI.

use std::ffi::CStr;
use std::os::raw::c_char;

mod tty;
use tty::Tty;

mod ffi;
use ffi::{Coord, Size, SyncInput, AsyncInput};


#[no_mangle]
pub extern fn init() -> *mut Tty {
    Box::into_raw(Box::new(Tty::init()))
}

#[no_mangle]
pub extern fn terminate(ptr: *mut Tty) {
    unsafe {
        assert!(!ptr.is_null());
        if ptr.is_null() { return }
        (&mut *ptr).terminate();
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern fn size(ptr: *mut Tty) -> Size {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).size().into()
    }
}

#[no_mangle]
pub extern fn raw(ptr: *mut Tty) {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).raw();
    }
}

#[no_mangle]
pub extern fn cook(ptr: *mut Tty) {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).cook();
    }
}

#[no_mangle]
pub extern fn enable_mouse(ptr: *mut Tty) {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).enable_mouse();
    }
}

#[no_mangle]
pub extern fn disable_mouse(ptr: *mut Tty) {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).disable_mouse();
    }
}

#[no_mangle]
pub extern fn read_char(ptr: *mut Tty) -> u32 {
    // NOTE: Since Rust char and C char are different implementations from each
    // other, instead we send a u32 over the FFI boundary. This allows for
    // flexibility in the implemenation language to transform the u32 as a
    // byte array of [u8; 4] and decode as the application expects.
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).read_char() as u32
    }
}

#[no_mangle]
pub extern fn read_sync(ptr: *mut Tty) -> *mut SyncInput {
    unsafe {
        assert!(!ptr.is_null());
        Box::into_raw(Box::new(SyncInput {
            iter: (&mut *ptr).read_sync(),
            event: Default::default(),
        }))
    }
}

#[no_mangle]
pub extern fn sync_next(ptr: *mut SyncInput) {
    let input = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    if let Some(ev) = input.iter.next() {
        ffi::match_event(ev, &mut input.event);
    }
}

#[no_mangle]
pub extern fn get_sync_kind(ptr: *mut SyncInput) -> u8 {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).event.kind
    }
}

#[no_mangle]
pub extern fn get_sync_label(ptr: *mut SyncInput) -> u8 {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).event.label
    }
}

#[no_mangle]
pub extern fn get_sync_btn(ptr: *mut SyncInput) -> u8 {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).event.btn
    }
}

#[no_mangle]
pub extern fn get_sync_coord(ptr: *mut SyncInput) -> Coord {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).event.coord.into()
    }
}

#[no_mangle]
pub extern fn get_sync_ch(ptr: *mut SyncInput) -> u32 {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).event.ch
    }
}

#[no_mangle]
pub extern fn sync_free(ptr: *mut SyncInput) {
    if ptr.is_null() { return }
    unsafe { Box::from_raw(ptr); }
}

#[no_mangle]
pub extern fn read_async(ptr: *mut Tty) -> *mut AsyncInput {
    unsafe {
        assert!(!ptr.is_null());
        Box::into_raw(Box::new(AsyncInput {
            iter: (&mut *ptr).read_async(),
            event: Default::default(),
        }))
    }
}

#[no_mangle]
pub extern fn read_until_async(ptr: *mut Tty, d: u8) -> *mut AsyncInput {
    unsafe {
        assert!(!ptr.is_null());
        Box::into_raw(Box::new(AsyncInput {
            iter: (&mut *ptr).read_until_async(d),
            event: Default::default(),
        }))
    }
}

#[no_mangle]
pub extern fn async_next(ptr: *mut AsyncInput) -> bool {
    let input = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    if let Some(ev) = input.iter.next() {
        ffi::match_event(ev, &mut input.event);
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern fn get_async_kind(ptr: *mut AsyncInput) -> u8 {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).event.kind
    }
}

#[no_mangle]
pub extern fn get_async_label(ptr: *mut AsyncInput) -> u8 {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).event.label
    }
}

#[no_mangle]
pub extern fn get_async_btn(ptr: *mut AsyncInput) -> u8 {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).event.btn
    }
}

#[no_mangle]
pub extern fn get_async_coord(ptr: *mut AsyncInput) -> Coord {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).event.coord.into()
    }
}

#[no_mangle]
pub extern fn get_async_ch(ptr: *mut AsyncInput) -> u32 {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).event.ch
    }
}

#[no_mangle]
pub extern fn async_free(ptr: *mut AsyncInput) {
    if ptr.is_null() { return }
    unsafe { Box::from_raw(ptr); }
}

#[no_mangle]
pub extern fn clear(ptr: *mut Tty, m: u8) {
    let method = ffi::match_method(m);
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).clear(method);
    }
}

#[no_mangle]
pub extern fn resize(ptr: *mut Tty, w: i16, h: i16) {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).resize(w, h);
    }
}

#[no_mangle]
pub extern fn switch(ptr: *mut Tty) {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).switch();
    }
}

#[no_mangle]
pub extern fn to_main(ptr: *mut Tty) {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).to_main();
    };
}

#[no_mangle]
pub extern fn switch_to(ptr: *mut Tty, index: usize) {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).switch_to(index);
    }
}


#[no_mangle]
pub extern fn goto(ptr: *mut Tty, col: i16, row: i16) {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).goto(col, row);
    }
}

#[no_mangle]
pub extern fn up(ptr: *mut Tty) {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).up();
    }
}

#[no_mangle]
pub extern fn dn(ptr: *mut Tty) {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).dn();
    }
}

#[no_mangle]
pub extern fn left(ptr: *mut Tty) {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).left();
    }
}

#[no_mangle]
pub extern fn right(ptr: *mut Tty) {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).right();
    }
}

#[no_mangle]
pub extern fn dpad(ptr: *mut Tty, d: u8, n: i16) {
    let dir = ffi::match_direction(d);
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).dpad(dir, n);
    }
}

#[no_mangle]
pub extern fn pos(ptr: *mut Tty) -> Coord {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).pos().into()
    }
}

#[no_mangle]
pub extern fn mark(ptr: *mut Tty) {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).mark();
    }
}

#[no_mangle]
pub extern fn load(ptr: *mut Tty) {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).load();
    }
}

#[no_mangle]
pub extern fn hide_cursor(ptr: *mut Tty) {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).hide_cursor();
    }
}

#[no_mangle]
pub extern fn show_cursor(ptr: *mut Tty) {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).show_cursor();
    }
}

#[no_mangle]
pub extern fn set_fg(ptr: *mut Tty, f: u8) {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).set_fg(ffi::match_color(f));
    }
}

#[no_mangle]
pub extern fn set_bg(ptr: *mut Tty, b: u8) {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).set_bg(ffi::match_color(b));
    };
}

#[no_mangle]
pub extern fn set_tx(ptr: *mut Tty, s: u8) {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).set_tx(ffi::match_style(s));
    }
}

#[no_mangle]
pub extern fn set_fg_rgb(ptr: *mut Tty, r: u8, g: u8, b: u8) {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).set_fg_rgb(r, g, b);
    }
}

#[no_mangle]
pub extern fn set_bg_rgb(ptr: *mut Tty, r: u8, g: u8, b: u8) {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).set_bg_rgb(r, g, b);
    }
}

#[no_mangle]
pub extern fn set_fg_ansi(ptr: *mut Tty, v: u8) {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).set_fg_ansi(v);
    }
}

#[no_mangle]
pub extern fn set_bg_ansi(ptr: *mut Tty, v: u8) {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).set_bg_ansi(v);
    }
}

#[no_mangle]
pub extern fn set_style(ptr: *mut Tty, f: u8, b: u8, s: u8) {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).set_style(
            ffi::match_color(f),
            ffi::match_color(b),
            ffi::match_style(s));
    }
}

#[no_mangle]
pub extern fn reset(ptr: *mut Tty) {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).reset();
    }
}

#[no_mangle]
pub extern fn prints(ptr: *mut Tty, c_str: *const c_char) {
    unsafe {
        assert!(!c_str.is_null());
        assert!(!ptr.is_null());
        (&mut *ptr).prints(
            CStr::from_ptr(c_str).to_str().unwrap());
    }
}

#[no_mangle]
pub extern fn flush(ptr: *mut Tty) {
    unsafe {
        assert!(!ptr.is_null());
        (&mut *ptr).flush();
    }
}
