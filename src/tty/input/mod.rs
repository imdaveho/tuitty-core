//! # Input
//!
//! The `input` module contains functions that abstracts the ways to capture and
//! parse keyboard and mouse input. It also provides multiple functions to read
//! a single character, handle synchronous input, handle asynchronous input, or
//! handle reading until a specific character is passed.
//!
//! There are no notable differences between what is implemented in the ANSI and
//! WinCon sub-modules.

#[cfg(unix)]
pub mod ansi;

#[cfg(windows)]
pub mod wincon;

#[cfg(windows)]
use crate::tty::Handle;

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
    Press(MouseButton, i16, i16),
    Release(i16, i16),
    Hold(i16, i16),
    Unknown,
}

// #[derive(Debug, PartialOrd, PartialEq, Hash, Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    WheelUp,
    WheelDn,
}

// #[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Clone)]
pub enum KeyEvent {
    Backspace,
    Left,
    Right,
    Up,
    Dn,
    Home,
    End,
    PageUp,
    PageDn,
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
    CtrlDn,
    CtrlRight,
    CtrlLeft,
    ShiftUp,
    ShiftDn,
    ShiftRight,
    ShiftLeft,
}