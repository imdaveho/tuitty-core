//! This module contains structs, functions, and traits that support CFFI.

use super::tty::{
    AsyncReader, SyncReader,
    InputEvent, KeyEvent, MouseEvent, MouseButton
};


pub fn match_method(m: u8) -> &'static str {
    match m {
        0 => "all",
        1 => "newln",
        2 => "currentln",
        3 => "cursorup",
        4 => "cursordn",
        _ => "",
    }
}

pub fn match_direction(d: u8) -> &'static str {
    match d {
        0 => "up",
        1 => "dn",
        2 => "left",
        3 => "right",
        _ => "",
    }
}

pub fn match_color(c: u8) -> &'static str {
    match c {
        0 => "reset",
        1 => "black",
        2 => "red",
        3 => "green",
        4 => "yellow",
        5 => "blue",
        6 => "magenta",
        7 => "cyan",
        8 => "white",
        9 => "dark_grey",
        10 => "dark_red",
        11 => "dark_green",
        12 => "dark_yellow",
        13 => "dark_blue",
        14 => "dark_magenta",
        15 => "dark_cyan",
        16 => "grey",
        _ => "reset",
    }
}

pub fn match_style(s: u8) -> &'static str {
    match s {
        0 => "reset",
        1 => "bold",
        2 => "dim",
        4 => "underline",
        7 => "reverse",
        8 => "hide",

        14 => "bold, underline",
        17 => "bold, reverse",
        18 => "bold, hide",

        24 => "dim, underline",
        27 => "dim, reverse",
        28 => "dim, hide",

        47 => "underline, reverse",
        48 => "underline, hide",

        78 => "reverse, hide",

        147 => "bold, underline, reverse",
        247 => "dim, underline, reverse",

        148 => "bold, reverse, hide",
        248 => "dim, reverse, hide",

        254 => "bold, underline, reverse, hide",
        255 => "dim, underline, reverse, hide",

        _ => "reset",
    }
}


#[repr(C)]
pub struct Coord {
    row: i16,
    col: i16,
}

#[repr(C)]
pub struct Size {
    w: i16,
    h: i16,
}

impl From<(i16, i16)> for Coord {
    fn from(coord: (i16, i16)) -> Coord {
        Coord { row: coord.0, col: coord.1 }
    }
}

impl From<Coord> for (i16, i16) {
    fn from(coord: Coord) -> (i16, i16) {
        (coord.row, coord.col)
    }
}

impl From<(i16, i16)> for Size {
    fn from(size: (i16, i16)) -> Size {
        Size { w: size.0, h: size.1 }
    }
}

impl From<Size> for (i16, i16) {
    fn from(size: Size) -> (i16, i16) {
        (size.w, size.h)
    }
}


pub struct SyncInput {
    pub iter: SyncReader,
    pub event: Event,
}

pub struct AsyncInput {
    pub iter: AsyncReader,
    pub event: Event,
}

pub struct Event {
    // 0=Keyboard, 1=Mouse, 2=Null
    pub kind: u8,
    // 0..n for label of Keyboard or Mouse events.
    pub label: u8,
    // 0=MouseLeft, 1=MouseRight, 2=MouseMiddle, 3=MouseWheelUp, 4=MouseWheelDn
    pub btn: u8,
    // (row: i16, col: i16); default (-1, -1)
    pub coord: (i16, i16),
    // char or u8 for Char(char), Ctrl(char), Alt(char), and F(u8 as u32)
    pub ch: u32,
}

impl Default for Event {
    fn default() -> Self {
        Event {
            kind: 2,
            label: 0,
            btn: 0,
            coord: (0, 0),
            ch: 0,
        }
    }
}

pub fn match_event(input: InputEvent, evt: &mut Event) {
    match input {
        InputEvent::Keyboard(ke) => {
            evt.kind = 0;
            match ke {
                KeyEvent::Backspace  => evt.label = 0,
                KeyEvent::Left       => evt.label = 1,
                KeyEvent::Right      => evt.label = 2,
                KeyEvent::Up         => evt.label = 3,
                KeyEvent::Dn         => evt.label = 4,
                KeyEvent::Home       => evt.label = 5,
                KeyEvent::End        => evt.label = 6,
                KeyEvent::PageUp     => evt.label = 7,
                KeyEvent::PageDn     => evt.label = 8,
                KeyEvent::BackTab    => evt.label = 9,
                KeyEvent::Delete     => evt.label = 10,
                KeyEvent::Insert     => evt.label = 11,
                KeyEvent::F(n) => {
                    evt.label = 12;
                    evt.ch = n as u32;
                },
                KeyEvent::Char(c) => {
                    evt.label = 13;
                    evt.ch = c as u32;
                },
                KeyEvent::Alt(c) => {
                    evt.label = 14;
                    evt.ch = c as u32;
                },
                KeyEvent::Ctrl(c) => {
                    evt.label = 15;
                    evt.ch = c as u32;
                },
                KeyEvent::Null       => evt.kind = 2,
                KeyEvent::Esc        => evt.label = 16,
                KeyEvent::CtrlUp     => evt.label = 17,
                KeyEvent::CtrlDn     => evt.label = 18,
                KeyEvent::CtrlRight  => evt.label = 19,
                KeyEvent::CtrlLeft   => evt.label = 20,
                KeyEvent::ShiftUp    => evt.label = 21,
                KeyEvent::ShiftDn    => evt.label = 22,
                KeyEvent::ShiftRight => evt.label = 23,
                KeyEvent::ShiftLeft  => evt.label = 24,
            }
        },
        InputEvent::Mouse(me) => {
            evt.kind = 1;
            match me {
                MouseEvent::Press(b, r, c) => {
                    evt.label = 0;
                    evt.coord = (r, c);
                    match b {
                        MouseButton::Left    => evt.btn = 0,
                        MouseButton::Right   => evt.btn = 1,
                        MouseButton::Middle  => evt.btn = 2,
                        MouseButton::WheelUp => evt.btn = 3,
                        MouseButton::WheelDn => evt.btn = 4,
                    }
                },
                MouseEvent::Release(r, c) => {
                    evt.label = 1;
                    evt.coord = (r, c);
                }
                MouseEvent::Hold(r, c) => {
                    evt.label = 2;
                    evt.coord = (r, c);
                },
                MouseEvent::Unknown => evt.kind = 2,
            }
        },
        InputEvent::Unknown => evt.kind = 2,
        _ => evt.kind = 2,
    }
}
