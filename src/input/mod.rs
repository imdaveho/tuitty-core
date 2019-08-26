//! # Input
//!
//! The `input` module contains functions that abstracts the ways to capture and
//! parse keyboard and mouse input. It also provides multiple functions to read
//! a single character, handle synchronous input, handle asynchronous input, or
//! handle reading until a specific character is passed.
//!
//! There are no notable differences between what is implemented in the ANSI and
//! WinCon sub-modules.

pub mod ansi;

// #[cfg(unix)]
// pub use linux::{
//     _read_char as read_char,
//     _read_sync as read_sync,
//     _read_async as read_async,
//     _read_until_async as read_until_async,
//     _enable_mouse_mode as enable_mouse_input,
//     _disable_mouse_mode as disable_mouse_input,
//     AsyncReader,
//     SyncReader,
// };

#[cfg(windows)]
pub mod wincon;

// #[cfg(windows)]
// pub use windows::{
//     _read_char as read_char,
//     _read_sync as read_sync,
//     _read_async as read_async,
//     _read_until_async as read_until_async,
//     _enable_mouse_mode as enable_mouse_input,
//     AsyncReader,
//     SyncReader,
// };

// (imdaveho) TODO: Check to see if it works in legacy cmd.exe.
// use std::io::{Result, stdin};
// pub fn read_line() -> Result<String> {
//     let mut rv = String::new();
//     stdin().read_line(&mut rv)?;
//     let len = rv.trim_end_matches(&['\r', '\n'][..]).len();
//     rv.truncate(len);
//     Ok(rv)
// }


// #[derive(Debug, PartialOrd, PartialEq, Hash, Clone)]
pub enum InputEvent {
    Keyboard(KeyEvent),
    Mouse(MouseEvent),
    Unsupported(Vec<u8>),
    Unknown,
}

// #[derive(Debug, PartialOrd, PartialEq, Hash, Clone, Copy)]
pub enum MouseEvent {
    Press(MouseButton, u16, u16),
    Release(u16, u16),
    Hold(u16, u16),
    Unknown,
}

// #[derive(Debug, PartialOrd, PartialEq, Hash, Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    WheelUp,
    WheelDown,
}

// #[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Clone)]
pub enum KeyEvent {
    Backspace,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    BackTab,
    Delete,
    Insert,
    F(u8),
    Char(char),
    Alt(char),
    Ctrl(char),
    Null,
    Esc,
    CtrlUp,
    CtrlDown,
    CtrlRight,
    CtrlLeft,
    ShiftUp,
    ShiftDown,
    ShiftRight,
    ShiftLeft,
}
