// //! `tuitty` is a cross platform library that is meant for FFI.

// use std::ffi::CStr;
// use std::os::raw::c_char;

// mod tty;
// mod terminal;

// use tty::{
//     Tty, AsyncReader, SyncReader,
//     InputEvent, KeyEvent, MouseEvent, MouseButton
// };


// #[no_mangle]
// pub extern fn init() -> *mut Tty {
//     Box::into_raw(Box::new(Tty::init()))
// }

// #[no_mangle]
// pub extern fn manual(ptr: *mut Tty) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).manual();
//     }
// }

// #[no_mangle]
// pub extern fn automatic(ptr: *mut Tty) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).automatic();
//     }
// }

// #[no_mangle]
// pub extern fn terminate(ptr: *mut Tty) {
//     unsafe {
//         if ptr.is_null() { return }
//         assert!(!ptr.is_null());
//         (&mut *ptr).terminate();
//         Box::from_raw(ptr);
//     }
// }

// // #[no_mangle]
// // pub extern fn size(ptr: *mut Tty) -> u32 {
// //     // NOTE: instead of a Tuple, we are sending a u32
// //     // that has the first 16 bits containing `w: i16`
// //     // and the second 16 bits containing `h: i16`.
// //     unsafe {
// //         assert!(!ptr.is_null());
// //         let (w, h) = (&mut *ptr).size();
// //         ((w as u32) << 16) | h as u32
// //     }
// // }

// #[no_mangle]
// pub extern fn screen_size(ptr: *mut Tty) -> u32 {
//     // NOTE: instead of a Tuple, we are sending a u32
//     // that has the first 16 bits containing `w: i16`
//     // and the second 16 bits containing `h: i16`.
//     unsafe {
//         assert!(!ptr.is_null());
//         let (w, h) = (&mut *ptr).screen_size();
//         ((w as u32) << 16) | h as u32
//     }
// }

// #[no_mangle]
// pub extern fn raw(ptr: *mut Tty) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).raw();
//     }
// }

// #[no_mangle]
// pub extern fn cook(ptr: *mut Tty) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).cook();
//     }
// }

// #[no_mangle]
// pub extern fn enable_mouse(ptr: *mut Tty) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).enable_mouse();
//     }
// }

// #[no_mangle]
// pub extern fn disable_mouse(ptr: *mut Tty) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).disable_mouse();
//     }
// }

// #[no_mangle]
// pub extern fn read_char(ptr: *mut Tty) -> u32 {
//     // NOTE: Since Rust char and C char are different implementations from each
//     // other, instead we send a u32 over the FFI boundary. This allows for
//     // flexibility in the implemenation language to transform the u32 as a
//     // byte array of [u8; 4] and decode as the application expects.
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).read_char() as u32
//     }
// }

// #[no_mangle]
// pub extern fn read_sync(ptr: *mut Tty) -> *mut SyncReader {
//     unsafe {
//         assert!(!ptr.is_null());
//         Box::into_raw(Box::new((&mut *ptr).read_sync()))
//     }
// }

// #[no_mangle]
// pub extern fn sync_next(ptr: *mut SyncReader, event: &mut Event) {
//     let stdin = unsafe {
//         assert!(!ptr.is_null());
//         &mut *ptr
//     };

//     if let Some(evt) = stdin.next() {
//         c_event(evt, event);
//     }
// }

// #[no_mangle]
// pub extern fn sync_free(ptr: *mut SyncReader) {
//     if ptr.is_null() { return }
//     unsafe { Box::from_raw(ptr); }
// }

// #[no_mangle]
// pub extern fn read_async(ptr: *mut Tty) -> *mut AsyncReader {
//     unsafe {
//         assert!(!ptr.is_null());
//         Box::into_raw(Box::new((&mut *ptr).read_async()))
//     }
// }

// #[no_mangle]
// pub extern fn read_until_async(ptr: *mut Tty, d: u8) -> *mut AsyncReader {
//     unsafe {
//         assert!(!ptr.is_null());
//         Box::into_raw(Box::new((&mut *ptr).read_until_async(d)))
//     }
// }

// #[no_mangle]
// pub extern fn async_next(ptr: *mut AsyncReader, event: &mut Event) -> bool {
//     let stdin = unsafe {
//         assert!(!ptr.is_null());
//         &mut *ptr
//     };

//     if let Some(evt) = stdin.next() {
//         c_event(evt, event);
//         return true
//     } else { return false }
// }

// #[no_mangle]
// pub extern fn async_free(ptr: *mut AsyncReader) {
//     if ptr.is_null() { return }
//     unsafe { Box::from_raw(ptr); }
// }

// #[no_mangle]
// pub extern fn clear(ptr: *mut Tty, c_str: *const c_char) {
//     unsafe {
//         assert!(!c_str.is_null());
//         assert!(!ptr.is_null());
//         (&mut *ptr).clear(
//             CStr::from_ptr(c_str).to_str().unwrap());
//     }
// }

// #[no_mangle]
// pub extern fn resize(ptr: *mut Tty, w: i16, h: i16) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).resize(w, h);
//     }
// }

// #[no_mangle]
// pub extern fn switch(ptr: *mut Tty) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).switch();
//     }
// }

// #[no_mangle]
// pub extern fn to_main(ptr: *mut Tty) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).to_main();
//     };
// }

// #[no_mangle]
// pub extern fn switch_to(ptr: *mut Tty, index: usize) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).switch_to(index);
//     }
// }


// #[no_mangle]
// pub extern fn goto(ptr: *mut Tty, col: i16, row: i16) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).goto(col, row);
//     }
// }

// #[no_mangle]
// pub extern fn up(ptr: *mut Tty) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).up();
//     }
// }

// #[no_mangle]
// pub extern fn dn(ptr: *mut Tty) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).dn();
//     }
// }

// #[no_mangle]
// pub extern fn left(ptr: *mut Tty) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).left();
//     }
// }

// #[no_mangle]
// pub extern fn right(ptr: *mut Tty) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).right();
//     }
// }

// #[no_mangle]
// pub extern fn dpad(ptr: *mut Tty, c_str: *const c_char, n: i16) {
//     unsafe {
//         assert!(!c_str.is_null());
//         assert!(!ptr.is_null());
//         (&mut *ptr).dpad(
//             CStr::from_ptr(c_str).to_str().unwrap(), n);
//     }
// }

// #[no_mangle]
// pub extern fn pos(ptr: *mut Tty) -> u32 {
//     // NOTE: instead of a Tuple, we are sending a u32
//     // that has the first 16 bits containing `col: i16`
//     // and the second 16 bits containing `row: i16`.
//     unsafe {
//         assert!(!ptr.is_null());
//         let (col, row) = (&mut *ptr).pos();
//         ((col as u32) << 16) | row as u32
//     }
// }

// #[no_mangle]
// pub extern fn mark(ptr: *mut Tty) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).mark();
//     }
// }

// #[no_mangle]
// pub extern fn load(ptr: *mut Tty) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).load();
//     }
// }

// #[no_mangle]
// pub extern fn hide_cursor(ptr: *mut Tty) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).hide_cursor();
//     }
// }

// #[no_mangle]
// pub extern fn show_cursor(ptr: *mut Tty) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).show_cursor();
//     }
// }

// // #[no_mangle]
// // pub extern fn set_fgcol(ptr: *mut Tty, c_str: *const c_char) {
// //     unsafe {
// //         assert!(!c_str.is_null());
// //         assert!(!ptr.is_null());
// //         (&mut *ptr).set_fgcol(
// //             CStr::from_ptr(c_str).to_str().unwrap());
// //     }
// // }

// // #[no_mangle]
// // pub extern fn set_bgcol(ptr: *mut Tty, c_str: *const c_char) {
// //     unsafe {
// //         assert!(!ptr.is_null());
// //         (&mut *ptr).set_bgcol(
// //             CStr::from_ptr(c_str).to_str().unwrap());
// //     };
// // }

// // #[no_mangle]
// // pub extern fn set_txfmt(ptr: *mut Tty, c_str: *const c_char) {
// //     unsafe {
// //         assert!(!c_str.is_null());
// //         assert!(!ptr.is_null());
// //         (&mut *ptr).set_txfmt(
// //             CStr::from_ptr(c_str).to_str().unwrap());
// //     }
// // }

// // #[no_mangle]
// // pub extern fn set_fg_rgb(ptr: *mut Tty, r: u8, g: u8, b: u8) {
// //     unsafe {
// //         assert!(!ptr.is_null());
// //         (&mut *ptr).set_fg_rgb(r, g, b);
// //     }
// // }

// // #[no_mangle]
// // pub extern fn set_bg_rgb(ptr: *mut Tty, r: u8, g: u8, b: u8) {
// //     unsafe {
// //         assert!(!ptr.is_null());
// //         (&mut *ptr).set_bg_rgb(r, g, b);
// //     }
// // }

// // #[no_mangle]
// // pub extern fn set_fg_ansi(ptr: *mut Tty, value: u8) {
// //     unsafe {
// //         assert!(!ptr.is_null());
// //         (&mut *ptr).set_fg_ansi(value);
// //     }
// // }

// // #[no_mangle]
// // pub extern fn set_bg_ansi(ptr: *mut Tty, value: u8) {
// //     unsafe {
// //         assert!(!ptr.is_null());
// //         (&mut *ptr).set_bg_ansi(value);
// //     }
// // }

// // #[no_mangle]
// // pub extern fn set_style(
// //     ptr: *mut Tty,
// //     fg: *const c_char,
// //     bg: *const c_char,
// //     fmts: *const c_char) {
// //     unsafe {
// //         assert!(!fg.is_null());
// //         assert!(!bg.is_null());
// //         assert!(!fmts.is_null());
// //         assert!(!ptr.is_null());
// //         (&mut *ptr).set_style(
// //             CStr::from_ptr(fg).to_str().unwrap(),
// //             CStr::from_ptr(bg).to_str().unwrap(),
// //             CStr::from_ptr(fmts).to_str().unwrap());
// //     }
// // }

// #[no_mangle]
// pub extern fn reset(ptr: *mut Tty) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).reset();
//     }
// }

// #[no_mangle]
// pub extern fn prints(ptr: *mut Tty, c_str: *const c_char) {
//     unsafe {
//         assert!(!c_str.is_null());
//         assert!(!ptr.is_null());
//         (&mut *ptr).prints(
//             CStr::from_ptr(c_str).to_str().unwrap());
//     }
// }

// #[no_mangle]
// pub extern fn flush(ptr: *mut Tty) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).flush();
//     }
// }


// // Struct to facilitate FFI for InputEvents.

// #[repr(C)]
// pub struct Event {
//     _kind: u8,
//     _data: u32,
// }


// fn c_event(input: InputEvent, event: &mut Event) {
//     match input {
//         InputEvent::Keyboard(kb) => {
//             match kb {
//                 KeyEvent::Null => event._kind = 0,
//                 KeyEvent::Backspace => event._kind = 1,
//                 KeyEvent::Left => event._kind = 2,
//                 KeyEvent::Right => event._kind = 3,
//                 KeyEvent::Up => event._kind = 4,
//                 KeyEvent::Dn => event._kind = 5,
//                 KeyEvent::Home => event._kind = 6,
//                 KeyEvent::End => event._kind = 7,
//                 KeyEvent::PageUp => event._kind = 8,
//                 KeyEvent::PageDn => event._kind = 9,
//                 KeyEvent::BackTab => event._kind = 10,
//                 KeyEvent::Delete => event._kind = 11,
//                 KeyEvent::Insert => event._kind = 12,
//                 KeyEvent::F(n) => {
//                     event._kind = 13;
//                     event._data = n as u32;
//                 },
//                 KeyEvent::Char(c) => {
//                     event._kind = 14;
//                     event._data = c as u32;
//                 },
//                 KeyEvent::Alt(c) => {
//                     event._kind = 15;
//                     event._data = c as u32;
//                 },
//                 KeyEvent::Ctrl(c) => {
//                     event._kind = 16;
//                     event._data = c as u32;
//                 },
//                 KeyEvent::Esc => event._kind = 17,
//                 KeyEvent::CtrlUp => event._kind = 18,
//                 KeyEvent::CtrlDn => event._kind = 19,
//                 KeyEvent::CtrlRight => event._kind = 20,
//                 KeyEvent::CtrlLeft => event._kind = 21,
//                 KeyEvent::ShiftUp => event._kind = 22,
//                 KeyEvent::ShiftDn => event._kind = 23,
//                 KeyEvent::ShiftRight => event._kind = 24,
//                 KeyEvent::ShiftLeft => event._kind = 25,
//             }
//         },
//         InputEvent::Mouse(ms) => {
//             match ms {
//                 MouseEvent::Press(btn, col, row) => {
//                     match btn {
//                         MouseButton::Left => event._kind = 26,
//                         MouseButton::Right => event._kind = 27,
//                         MouseButton::Middle => event._kind = 28,
//                         MouseButton::WheelUp => event._kind = 29,
//                         MouseButton::WheelDn => event._kind = 30,
//                     }
//                     event._data = ((col as u32) << 16) | row as u32;
//                 },
//                 MouseEvent::Hold(col, row) => {
//                     event._kind = 31;
//                     event._data = ((col as u32) << 16) | row as u32;
//                 }
//                 MouseEvent::Release(col, row) => {
//                     event._kind = 32;
//                     event._data = ((col as u32) << 16) | row as u32;
//                 },
//                 MouseEvent::Unknown => event._kind = 0,
//             }
//         },
//         InputEvent::Unknown => event._kind = 0,
//         _ => event._kind = 0,
//     }
// }
