//! This module contains structs, functions, and traits that support CFFI.

use super::{
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
    kind: u8,
    // 0..n for types of Keyboard or Mouse events.
    input: u8,
    // 0=MouseLeft, 1=MouseRight, 2=MouseMiddle, 3=MouseWheelUp, 4=MouseWheelDn
    btn: u8,
    // (row: i16, col: i16); default (-1, -1)
    coord: Coord,
    // char or u8 for Char(char), Ctrl(char), Alt(char), and F(u8 as u32)
    ch: u32,
}

impl Default for Event {
    fn default() -> Self {
        Event {
            kind: 2,
            input: 0,
            btn: 0,
            coord: (-1, -1).into(),
            ch: 0,
        }
    }
}

pub fn match_event(input: InputEvent, evt: &mut Event) {
    match input {
        InputEvent::Keyboard(ke) => {
            evt.kind = 0;
            match ke {
                KeyEvent::Backspace  => evt.input = 0,
                KeyEvent::Left       => evt.input = 1,
                KeyEvent::Right      => evt.input = 2,
                KeyEvent::Up         => evt.input = 3,
                KeyEvent::Down       => evt.input = 4,
                KeyEvent::Home       => evt.input = 5,
                KeyEvent::End        => evt.input = 6,
                KeyEvent::PageUp     => evt.input = 7,
                KeyEvent::PageDown   => evt.input = 8,
                KeyEvent::BackTab    => evt.input = 9,
                KeyEvent::Delete     => evt.input = 10,
                KeyEvent::Insert     => evt.input = 11,
                KeyEvent::F(n) => {
                    evt.input = 12;
                    evt.ch = n as u32;
                },
                KeyEvent::Char(c) => {
                    evt.input = 13;
                    evt.ch = c as u32;
                },
                KeyEvent::Alt(c) => {
                    evt.input = 14;
                    evt.ch = c as u32;
                },
                KeyEvent::Ctrl(c) => {
                    evt.input = 15;
                    evt.ch = c as u32;
                },
                KeyEvent::Null       => evt.kind = 2,
                KeyEvent::Esc        => evt.input = 16,
                KeyEvent::CtrlUp     => evt.input = 17,
                KeyEvent::CtrlDn     => evt.input = 18,
                KeyEvent::CtrlRight  => evt.input = 19,
                KeyEvent::CtrlLeft   => evt.input = 20,
                KeyEvent::ShiftUp    => evt.input = 21,
                KeyEvent::ShiftDn    => evt.input = 22,
                KeyEvent::ShiftRight => evt.input = 23,
                KeyEvent::ShiftLeft  => evt.input = 24,
            }
        },
        InputEvent::Mouse(me) => {
            evt.kind = 1;
            match me {
                MouseEvent::Press(b, r, c) => {
                    evt.input = 0;
                    evt.coord = (r, c).into();
                    match b {
                        MouseButton::Left    => evt.btn = 0,
                        MouseButton::Right   => evt.btn = 1,
                        MouseButton::Middle  => evt.btn = 2,
                        MouseButton::WheelUp => evt.btn = 3,
                        MouseButton::WheelDn => evt.btn = 4,
                    }
                },
                MouseEvent::Release(r, c) => {
                    evt.input = 1;
                    evt.coord = (r, c).into();
                }
                MouseEvent::Hold(r, c) => {
                    evt.input = 2;
                    evt.coord = (r, c).into();
                },
                MouseEvent::Unknown => evt.kind = 3,
            }
        },
        InputEvent::Unknown => evt.kind = 2,
        _ => evt.kind = 2,
    }
}
