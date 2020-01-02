//! `tuitty` is a cross platform, interoperable, simplfied terminal library
//! that is meant to be wrapped by multiple languages.

use std::ffi::{ CStr, CString };
use std::os::raw::c_char;

pub mod terminal;
pub mod common;

use terminal::dispatch::{ Dispatcher, EventHandle };
use common::enums::{ Clear::*, Color, Action::*, InputEvent };


// Initialization of Dispatcher and Event Handles
// #[no_mangle]
// pub extern fn dispatcher() -> *mut Dispatcher {
//     Box::into_raw(Box::new(Dispatcher::init()))
// }
//
// #[no_mangle]
// pub extern fn dispatcher_free(ptr: *mut Dispatcher) {
//     if ptr.is_null() { return }
//     unsafe { Box::from_raw(ptr); }
// }
//
// #[no_mangle]
// pub extern fn dispatcher_listen(ptr: *mut Dispatcher) -> *const EventHandle {
//     unsafe {
//         assert!(!ptr.is_null());
//         Box::into_raw(Box::new((&mut *ptr).listen()))
//     }
// }
//
// #[no_mangle]
// pub extern fn dispatcher_spawn(ptr: *mut Dispatcher) -> *const EventHandle {
//     unsafe {
//         assert!(!ptr.is_null());
//         Box::into_raw(Box::new((&mut *ptr).spawn()))
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_free(ptr: *mut EventHandle) {
//     if ptr.is_null() { return }
//     unsafe { Box::from_raw(ptr); }
// }
//
// // Cursor Signals
// #[no_mangle]
// pub extern fn dispatcher_goto(ptr: *mut Dispatcher, col: i16, row: i16) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).signal(Goto(col, row));
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_goto(ptr: *const EventHandle, col: i16, row: i16) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).signal(Goto(col, row));
//     }
// }
//
// #[no_mangle]
// pub extern fn dispatcher_up(ptr: *mut Dispatcher, n: i16) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).signal(Up(n));
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_up(ptr: *const EventHandle, n: i16) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).signal(Up(n));
//     }
// }
//
// #[no_mangle]
// pub extern fn dispatcher_down(ptr: *mut Dispatcher, n: i16) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).signal(Down(n));
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_down(ptr: *const EventHandle, n: i16) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).signal(Down(n));
//     }
// }
//
// #[no_mangle]
// pub extern fn dispatcher_left(ptr: *mut Dispatcher, n: i16) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).signal(Left(n));
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_left(ptr: *const EventHandle, n: i16) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).signal(Left(n));
//     }
// }
//
// #[no_mangle]
// pub extern fn dispatcher_right(ptr: *mut Dispatcher, n: i16) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).signal(Right(n));
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_right(ptr: *const EventHandle, n: i16) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).signal(Right(n));
//     }
// }
//
// // Screen/Output Signals
// #[no_mangle]
// pub extern fn dispatcher_clear(ptr: *mut Dispatcher, clr: u8) {
//     unsafe {
//         assert!(!ptr.is_null());
//         match clr {
//             0 => (&mut *ptr).signal(Clear(All)),
//             1 => (&mut *ptr).signal(Clear(CursorDn)),
//             2 => (&mut *ptr).signal(Clear(CursorUp)),
//             3 => (&mut *ptr).signal(Clear(CurrentLn)),
//             4 => (&mut *ptr).signal(Clear(NewLn)),
//             _ => ()
//         }
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_clear(ptr: *const EventHandle, clr: u8) {
//     unsafe {
//         assert!(!ptr.is_null());
//         match clr {
//             0 => (&*ptr).signal(Clear(All)),
//             1 => (&*ptr).signal(Clear(CursorDn)),
//             2 => (&*ptr).signal(Clear(CursorUp)),
//             3 => (&*ptr).signal(Clear(CurrentLn)),
//             4 => (&*ptr).signal(Clear(NewLn)),
//             _ => ()
//         }
//     }
// }
//
// #[no_mangle]
// pub extern fn dispatcher_resize(ptr: *mut Dispatcher, w: i16, h: i16) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).signal(Resize(w, h));
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_resize(ptr: *const EventHandle, w: i16, h: i16) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).signal(Resize(w, h));
//     }
// }
//
// #[no_mangle]
// pub extern fn dispatcher_prints(ptr: *mut Dispatcher, c_str: *mut c_char) {
//     unsafe {
//         assert!(!ptr.is_null());
//         assert!(!c_str.is_null());
//         let r_str = CStr::from_ptr(c_str).to_str();
//         if let Ok(s) = r_str { (&mut *ptr).signal(Prints(s.to_string())) }
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_prints(ptr: *const EventHandle, c_str: *mut c_char) {
//     unsafe {
//         assert!(!ptr.is_null());
//         assert!(!c_str.is_null());
//         let r_str = CStr::from_ptr(c_str).to_str();
//         if let Ok(s) = r_str { (&*ptr).signal(Prints(s.to_string())) }
//     }
// }
//
// #[no_mangle]
// pub extern fn dispatcher_printf(ptr: *mut Dispatcher, c_str: *mut c_char) {
//     unsafe {
//         assert!(!ptr.is_null());
//         assert!(!c_str.is_null());
//         let r_str = CStr::from_ptr(c_str).to_str();
//         if let Ok(s) = r_str { (&mut *ptr).signal(Printf(s.to_string())) }
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_printf(ptr: *const EventHandle, c_str: *mut c_char) {
//     unsafe {
//         assert!(!ptr.is_null());
//         assert!(!c_str.is_null());
//         let r_str = CStr::from_ptr(c_str).to_str();
//         if let Ok(s) = r_str { (&*ptr).signal(Printf(s.to_string())) }
//     }
// }
//
// #[no_mangle]
// pub extern fn dispatcher_flush(ptr: *mut Dispatcher) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).signal(Flush);
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_flush(ptr: *const EventHandle) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).signal(Flush);
//     }
// }
//
// // Style Signals
// #[no_mangle]
// pub extern fn dispatcher_set_basic_fg(ptr: *mut Dispatcher, fg: u8) {
//     unsafe {
//         assert!(!ptr.is_null());
//         if let Some(c) = colorize(fg) {
//             (&mut *ptr).signal(SetFg(c))
//         }
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_set_basic_fg(ptr: *const EventHandle, fg: u8) {
//     unsafe {
//         assert!(!ptr.is_null());
//         if let Some(c) = colorize(fg) {
//             (&*ptr).signal(SetFg(c))
//         }
//     }
// }
//
// #[no_mangle]
// pub extern fn dispatcher_set_ansi_fg(ptr: *mut Dispatcher, fg: u8) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).signal(SetFg(Color::AnsiValue(fg)));
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_set_ansi_fg(ptr: *const EventHandle, fg: u8) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).signal(SetFg(Color::AnsiValue(fg)));
//     }
// }
//
// #[no_mangle]
// pub extern fn dispatcher_set_rgb_fg(ptr: *mut Dispatcher, r: u8, g: u8, b: u8) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).signal(SetFg(Color::Rgb{r: r, g: g, b: b}));
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_set_rgb_fg(ptr: *const EventHandle, r: u8, g: u8, b: u8) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).signal(SetFg(Color::Rgb{r: r, g: g, b: b}));
//     }
// }
//
// #[no_mangle]
// pub extern fn dispatcher_set_basic_bg(ptr: *mut Dispatcher, bg: u8) {
//     unsafe {
//         assert!(!ptr.is_null());
//         if let Some(c) = colorize(bg) {
//             (&mut *ptr).signal(SetBg(c))
//         }
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_set_basic_bg(ptr: *const EventHandle, bg: u8) {
//     unsafe {
//         assert!(!ptr.is_null());
//         if let Some(c) = colorize(bg) {
//             (&*ptr).signal(SetBg(c))
//         }
//     }
// }
//
// #[no_mangle]
// pub extern fn dispatcher_set_ansi_bg(ptr: *mut Dispatcher, bg: u8) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).signal(SetBg(Color::AnsiValue(bg)));
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_set_ansi_bg(ptr: *const EventHandle, bg: u8) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).signal(SetBg(Color::AnsiValue(bg)));
//     }
// }
//
// #[no_mangle]
// pub extern fn dispatcher_set_rgb_bg(ptr: *mut Dispatcher, r: u8, g: u8, b: u8) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).signal(SetBg(Color::Rgb{r: r, g: g, b: b}));
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_set_rgb_bg(ptr: *const EventHandle, r: u8, g: u8, b: u8) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).signal(SetBg(Color::Rgb{r: r, g: g, b: b}));
//     }
// }
//
// #[no_mangle]
// pub extern fn dispatcher_set_fx(ptr: *mut Dispatcher, fx: u32) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).signal(SetFx(fx));
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_set_fx(ptr: *const EventHandle, fx: u32) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).signal(SetFx(fx));
//     }
// }
//
// #[no_mangle]
// pub extern fn dispatcher_set_styles(
//     ptr: *mut Dispatcher, fg: u8, bg: u8, fx: u32) {
//     match (colorize(fg), colorize(bg)) {
//         (Some(fg), Some(bg)) => {
//             unsafe {
//                 assert!(!ptr.is_null());
//                 (&mut *ptr).signal(SetStyles(fg, bg, fx));
//             }
//         },
//         (None, None) => {
//             unsafe {
//                 assert!(!ptr.is_null());
//                 (&mut *ptr).signal(SetFx(fx));
//             }
//         },
//         (Some(fg), None) => {
//             unsafe {
//                 assert!(!ptr.is_null());
//                 (&mut *ptr).signal(SetFg(fg));
//                 (&mut *ptr).signal(SetFx(fx));
//             }
//         },
//         (None, Some(bg)) => {
//             unsafe {
//                 assert!(!ptr.is_null());
//                 (&mut *ptr).signal(SetBg(bg));
//                 (&mut *ptr).signal(SetFx(fx));
//             }
//         }
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_set_styles(
//     ptr: *const EventHandle, fg: u8, bg: u8, fx: u32) {
//     match (colorize(fg), colorize(bg)) {
//         (Some(fg), Some(bg)) => {
//             unsafe {
//                 assert!(!ptr.is_null());
//                 (&*ptr).signal(SetStyles(fg, bg, fx));
//             }
//         },
//         (None, None) => {
//             unsafe {
//                 assert!(!ptr.is_null());
//                 (&*ptr).signal(SetFx(fx));
//             }
//         },
//         (Some(fg), None) => {
//             unsafe {
//                 assert!(!ptr.is_null());
//                 (&*ptr).signal(SetFg(fg));
//                 (&*ptr).signal(SetFx(fx));
//             }
//         },
//         (None, Some(bg)) => {
//             unsafe {
//                 assert!(!ptr.is_null());
//                 (&*ptr).signal(SetBg(bg));
//                 (&*ptr).signal(SetFx(fx));
//             }
//         }
//     }
// }
//
// #[no_mangle]
// pub extern fn dispatcher_reset_styles(ptr: *mut Dispatcher) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).signal(ResetStyles);
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_reset_styles(ptr: *const EventHandle) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).signal(ResetStyles);
//     }
// }
//
// // Toggle Mode Signals
// #[no_mangle]
// pub extern fn dispatcher_show_cursor(ptr: *mut Dispatcher) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).signal(ShowCursor);
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_show_cursor(ptr: *const EventHandle) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).signal(ShowCursor);
//     }
// }
//
// #[no_mangle]
// pub extern fn dispatcher_hide_cursor(ptr: *mut Dispatcher) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).signal(HideCursor);
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_hide_cursor(ptr: *const EventHandle) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).signal(HideCursor);
//     }
// }
//
// #[no_mangle]
// pub extern fn dispatcher_enable_mouse(ptr: *mut Dispatcher) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).signal(EnableMouse);
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_enable_mouse(ptr: *const EventHandle) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).signal(EnableMouse);
//     }
// }
//
//
// #[no_mangle]
// pub extern fn dispatcher_disable_mouse(ptr: *mut Dispatcher) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).signal(DisableMouse);
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_disable_mouse(ptr: *const EventHandle) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).signal(DisableMouse);
//     }
// }
//
// #[no_mangle]
// pub extern fn dispatcher_enable_alt(ptr: *mut Dispatcher) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).signal(EnableAlt);
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_enable_alt(ptr: *const EventHandle) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).signal(EnableAlt);
//     }
// }
//
//
// #[no_mangle]
// pub extern fn dispatcher_disable_alt(ptr: *mut Dispatcher) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).signal(DisableAlt);
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_disable_alt(ptr: *const EventHandle) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).signal(DisableAlt);
//     }
// }
//
// #[no_mangle]
// pub extern fn dispatcher_raw(ptr: *mut Dispatcher) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).signal(Raw);
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_raw(ptr: *const EventHandle) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).signal(Raw);
//     }
// }
//
//
// #[no_mangle]
// pub extern fn dispatcher_cook(ptr: *mut Dispatcher) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).signal(Cook);
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_cook(ptr: *const EventHandle) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).signal(Cook);
//     }
// }
//
// // Store Operation Signals
// #[no_mangle]
// pub extern fn dispatcher_switch(ptr: *mut Dispatcher) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).signal(Switch);
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_switch(ptr: *const EventHandle) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).signal(Switch);
//     }
// }
//
// #[no_mangle]
// pub extern fn dispatcher_switch_to(ptr: *mut Dispatcher, id: usize) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).signal(SwitchTo(id));
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_switch_to(ptr: *const EventHandle, id: usize) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).signal(SwitchTo(id));
//     }
// }
//
// #[no_mangle]
// pub extern fn dispatcher_resized(ptr: *mut Dispatcher) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).signal(Resized);
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_resized(ptr: *const EventHandle) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).signal(Resized);
//     }
// }
//
// #[no_mangle]
// pub extern fn dispatcher_mark(ptr: *mut Dispatcher, col: i16, row: i16) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).signal(SyncMarker(col, row));
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_mark(ptr: *const EventHandle, col: i16, row: i16) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).signal(SyncMarker(col, row));
//     }
// }
//
// #[no_mangle]
// pub extern fn dispatcher_jump(ptr: *mut Dispatcher) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).signal(Jump);
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_jump(ptr: *const EventHandle) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).signal(Jump);
//     }
// }
//
// // (imdaveho) NOTE: Subject to change.
// #[no_mangle]
// pub extern fn dispatcher_sync_tab_size(ptr: *mut Dispatcher, n: usize) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&mut *ptr).signal(SyncTabSize(n));
//     }
// }
//
// // (imdaveho) NOTE: Subject to change.
// #[no_mangle]
// pub extern fn event_handle_sync_tab_size(ptr: *const EventHandle, n: usize) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).signal(SyncTabSize(n));
//     }
// }
//
//
// // Store Requests (EventHandle Only)
// #[no_mangle]
// pub extern fn event_handle_size(ptr: *const EventHandle) -> u32 {
//     // NOTE: instead of a Tuple, we are sending a u32
//     // that has the first 16 bits containing `w: i16`
//     // and the second 16 bits containing `h: i16`.
//     unsafe {
//         assert!(!ptr.is_null());
//         let (w, h) = (&*ptr).size();
//         ((w as u32) << 16) | h as u32
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_coord(ptr: *const EventHandle) -> u32 {
//     // NOTE: instead of a Tuple, we are sending a u32
//     // that has the first 16 bits containing `col: i16`
//     // and the second 16 bits containing `row: i16`.
//     unsafe {
//         assert!(!ptr.is_null());
//         let (col, row) = (&*ptr).coord();
//         ((col as u32) << 16) | row as u32
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_syspos(ptr: *const EventHandle) -> u32 {
//     // NOTE: instead of a Tuple, we are sending a u32
//     // that has the first 16 bits containing `col: i16`
//     // and the second 16 bits containing `row: i16`.
//     unsafe {
//         assert!(!ptr.is_null());
//         let (col, row) = (&*ptr).syspos();
//         ((col as u32) << 16) | row as u32
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_getch(ptr: *const EventHandle) -> *mut c_char {
//     unsafe {
//         assert!(!ptr.is_null());
//         let r_str = (&*ptr).getch();
//         let c_str = CString::new(r_str).unwrap_or(
//             // NOTE: If a NUL byte is found, handle
//             // by creating an empty CString from "".
//             CString::new("").unwrap());
//         c_str.into_raw()
//     }
// }
//
// #[no_mangle]
// pub extern fn gotch_free(ptr: *mut c_char) {
//     unsafe {
//         if ptr.is_null() { return }
//         CString::from_raw(ptr);
//     }
// }
//
// // (imdaveho) NOTE: Subject to change.
// #[no_mangle]
// pub extern fn event_handle_poll_async(
//     ptr: *const EventHandle, meta: &mut Eventmeta) -> bool {
//     unsafe {
//         assert!(!ptr.is_null());
//         if let Some(evt) = (&*ptr).poll_async() {
//             meta.metastasize(evt);
//             true
//         } else { false }
//     }
// }
//
// // (imdaveho) NOTE: Subject to change.
// #[no_mangle]
// pub extern fn event_handle_poll_latest_async(
//     ptr: *const EventHandle, meta: &mut Eventmeta) -> bool {
//     unsafe {
//         assert!(!ptr.is_null());
//         if let Some(evt) = (&*ptr).poll_latest_async() {
//             meta.metastasize(evt);
//             true
//         } else { false }
//     }
// }
//
// // (imdaveho) NOTE: Subject to change.
// #[no_mangle]
// pub extern fn event_handle_poll_sync(
//     ptr: *const EventHandle, meta: &mut Eventmeta) {
//     unsafe {
//         assert!(!ptr.is_null());
//         if let Some(evt) = (&*ptr).poll_sync() {
//             meta.metastasize(evt);
//         }
//     }
// }
//
//
// // Event Handle Commands
// #[no_mangle]
// pub extern fn event_handle_suspend(ptr: *const EventHandle) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).suspend();
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_transmit(ptr: *const EventHandle) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).transmit();
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_stop(ptr: *const EventHandle) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).stop();
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_lock(ptr: *const EventHandle) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).lock();
//     }
// }
//
// #[no_mangle]
// pub extern fn event_handle_unlock(ptr: *const EventHandle) {
//     unsafe {
//         assert!(!ptr.is_null());
//         (&*ptr).unlock();
//     }
// }
//
//
// // Previous Implementation References:
// // #[no_mangle]
// // pub extern fn sync_next(ptr: *mut SyncReader, event: &mut Event) {
// //     let stdin = unsafe {
// //         assert!(!ptr.is_null());
// //         &mut *ptr
// //     };
//
// //     if let Some(evt) = stdin.next() {
// //         c_event(evt, event);
// //     }
// // }
//
// // #[no_mangle]
// // pub extern fn async_next(ptr: *mut AsyncReader, event: &mut Event) -> bool {
// //     let stdin = unsafe {
// //         assert!(!ptr.is_null());
// //         &mut *ptr
// //     };
//
// //     if let Some(evt) = stdin.next() {
// //         c_event(evt, event);
// //         return true
// //     } else { return false }
// // }
//
// // Helper function to convert ffi style
// // arguments into the corresponding Color.
// fn colorize(num: u8) -> Option<Color> {
//     match num {
//         0 => Some(Color::Reset),
//         1 => Some(Color::Black),
//         2 => Some(Color::DarkGrey),
//         3 => Some(Color::Red),
//         4 => Some(Color::DarkRed),
//         5 => Some(Color::Green),
//         6 => Some(Color::DarkGreen),
//         7 => Some(Color::Yellow),
//         8 => Some(Color::DarkYellow),
//         9 => Some(Color::Blue),
//         10 => Some(Color::DarkBlue),
//         11 => Some(Color::Magenta),
//         12 => Some(Color::DarkMagenta),
//         13 => Some(Color::Cyan),
//         14 => Some(Color::DarkCyan),
//         15 => Some(Color::White),
//         16 => Some(Color::Grey),
//         _ => None,
//     }
// }
//
// // Helper struct to pass along data regarding
// // InputEvents parsed by the Dispatcher.
// #[repr(C)]
// pub struct Eventmeta { _kind: u8, _data: u32 }
//
// impl Eventmeta {
//     fn metastasize(&mut self, input: InputEvent) {
//         match input {
//             InputEvent::Keyboard(kv) => {
//                 self._kind = kv.enumerate();
//                 self._data = kv.values();
//             },
//             InputEvent::Mouse(mv) => {
//                 self._kind = mv.enumerate();
//                 self._data = mv.values();
//             },
//             InputEvent::Unsupported => self._kind = 0,
//             _ => self._kind = 0,
//         }
//     }
// }
