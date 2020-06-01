// Define enums used across the entire library.
// Contains:
// * Clear
// * Style
// * Color
// * Effect
// * InputEvent
// * MouseEvent
// * MouseButton
// * KeyEvent

use std::ops::{ BitAnd, BitOr };


#[derive(Copy, Clone)]
// TODO: change Dn -> Down and Ln -> Line
pub enum Clear {
    /// clear all cells in terminal
    All,
    /// clear all cells from the cursor downwards
    CursorDn,
    /// clear all cells from the cursor upwards
    CursorUp,
    /// clear the current line
    CurrentLn,
    /// clear all cells from the cursor until a new line
    NewLn
}


#[derive(Copy, Clone)]
pub enum Style {
    Fg(Color),
    Bg(Color),
    Fx(u32),
}


#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Reset,
    Black,
    DarkGrey,
    Red,
    DarkRed,
    Green,
    DarkGreen,
    Yellow,
    DarkYellow,
    Blue,
    DarkBlue,
    Magenta,
    DarkMagenta,
    Cyan,
    DarkCyan,
    White,
    Grey,
    Rgb{r: u8, g: u8, b: u8},
    AnsiValue(u8),
}

#[cfg(windows)]
pub const RESET: u16 = 0xFFFF;
#[cfg(windows)]
pub const IGNORE: u16 = 0xFFF0;

#[cfg(windows)]
use winapi::um::wincon::{
    COMMON_LVB_UNDERSCORE as UNDERLINE,
    COMMON_LVB_REVERSE_VIDEO as REVERSE,
    FOREGROUND_RED as FG_RED,
    FOREGROUND_GREEN as FG_GREEN,
    FOREGROUND_BLUE as FG_BLUE,
    FOREGROUND_INTENSITY as FG_INTENSE,
    BACKGROUND_RED as BG_RED,
    BACKGROUND_GREEN as BG_GREEN,
    BACKGROUND_BLUE as BG_BLUE,
    BACKGROUND_INTENSITY as BG_INTENSE,
};

#[cfg(windows)]
pub fn foreground(color: Color, current: u16, reset: u16) -> u16 {
    let mut attrib = match color {
        Color::Black => 0,
        Color::DarkGrey => FG_INTENSE,
        Color::Red => FG_RED | FG_INTENSE,
        Color::DarkRed => FG_GREEN,
        Color::Green => FG_GREEN | FG_INTENSE,
        Color::DarkGreen => FG_GREEN,
        Color::Yellow => FG_RED | FG_GREEN | FG_INTENSE,
        Color::DarkYellow => FG_RED | FG_GREEN,
        Color::Blue => FG_BLUE | FG_INTENSE,
        Color::DarkBlue => FG_BLUE,
        Color::Magenta => FG_RED | FG_BLUE | FG_INTENSE,
        Color::DarkMagenta => FG_RED | FG_BLUE,
        Color::Cyan => FG_GREEN | FG_BLUE | FG_INTENSE,
        Color::DarkCyan => FG_GREEN | FG_BLUE,
        Color::White => FG_RED | FG_GREEN | FG_BLUE,
        Color::Grey => FG_RED | FG_GREEN | FG_BLUE | FG_INTENSE,
        Color::Reset => RESET,
        Color::Rgb{r: _, g: _, b: _} => IGNORE,
        Color::AnsiValue(_) => IGNORE,
    };
    if attrib == RESET {
        attrib = reset & 0x000f;
    }
    if attrib == IGNORE {
        attrib = current & 0x000f;
    }
    // (imdaveho) NOTE: We need to isolate Colors in Windows
    // because Color attributes mix. So if you previously had
    // Color::Red and you wanted Color::Blue, that would end up
    // as Color::Magenta if you didn't first clear it out
    // attrib | just_bg | just_fx
    attrib | current & !0x000f
}

#[cfg(windows)]
pub fn background(color: Color, current: u16, reset: u16) -> u16 {
    let mut attrib = match color {
        Color::Black => 0,
        Color::DarkGrey => BG_INTENSE,
        Color::Red => BG_RED | BG_INTENSE,
        Color::DarkRed => BG_GREEN,
        Color::Green => BG_GREEN | BG_INTENSE,
        Color::DarkGreen => BG_GREEN,
        Color::Yellow => BG_RED | BG_GREEN | BG_INTENSE,
        Color::DarkYellow => BG_RED | BG_GREEN,
        Color::Blue => BG_BLUE | BG_INTENSE,
        Color::DarkBlue => BG_BLUE,
        Color::Magenta => BG_RED | BG_BLUE | BG_INTENSE,
        Color::DarkMagenta => BG_RED | BG_BLUE,
        Color::Cyan => BG_GREEN | BG_BLUE | BG_INTENSE,
        Color::DarkCyan => BG_GREEN | BG_BLUE,
        Color::White => BG_RED | BG_GREEN | BG_BLUE,
        Color::Grey => BG_RED | BG_GREEN | BG_BLUE | BG_INTENSE,
        Color::Reset => RESET,
        Color::Rgb{r: _, g: _, b: _} => IGNORE,
        Color::AnsiValue(_) => IGNORE,
    };
    if attrib == RESET {
        attrib = reset & 0x00f0;
    }
    if attrib == IGNORE {
        attrib = current & 0x00f0;
    }
    // (imdaveho) NOTE: We need to isolate Colors in Windows
    // because Color attributes mix. So if you previously had
    // Color::Red and you wanted Color::Blue, that would end up
    // as Color::Magenta if you didn't first clear it out
    // just_fg | attrib | just_fx
    attrib | current & !0x00f0
    
}

#[cfg(windows)]
pub fn effects(fx: u32, current: u16) -> u16 {
    let mut attrib = current;
    let available_effects = [
        Effect::Reset, 
        Effect::Bold, 
        Effect::Dim, 
        Effect::Underline, 
        Effect::Reverse, 
        Effect::Hide
    ];
    for effect in &available_effects {
        if (fx & *effect as u32) != 0 {
            match *effect {
                Effect::Bold => attrib |= FG_INTENSE,
                Effect::Dim => attrib &= !FG_INTENSE,
                Effect::Underline => attrib |= UNDERLINE,
                Effect::Reverse => attrib |= REVERSE,
                Effect::Hide => {
                    // FOREGROUND and BACKGROUND color differ by 4
                    // bits; to convert from 0x0020 (BG Green) to
                    // 0x0002 (FG Green), shift right 4 bits. By
                    // making the FOREGROUND color the same as the
                    // BACKGROUND color, effectively you hide the
                    // printed content.
                    let updated_fg = (current & 0x00f0) >> 4;
                    // Since we identified the new FOREGROUND, we
                    // include it and remove it from the current
                    // attributes. The BACKGROUND should remain the
                    // same within the current attrs.
                    attrib = updated_fg | current & !0x000f
                },
                Effect::Reset => attrib = current & !0xdf00,
            }
        }
    }
    attrib
}


#[derive(Clone, Copy, PartialEq)]
pub enum Effect {
    Reset = 1 << (0 + 9),
    Bold = 1 << (1 + 9),
    Dim = 1 << (2 + 9),
    Underline = 1 << (4 + 9),
    Reverse = 1 << (7 + 9),
    Hide = 1 << (8 + 9),
}

impl BitOr<u32> for Effect {
    type Output = u32;

    fn bitor(self, rhs: u32) -> u32 {
        self as u32 | rhs
    }
}

impl BitOr<Effect> for Effect {
    type Output = u32;

    fn bitor(self, rhs: Self) -> u32 {
        self as u32 | rhs as u32
    }
}

impl BitOr<Effect> for u32 {
    type Output = Self;

    fn bitor(self, rhs: Effect) -> Self {
        self | rhs as u32
    }
}

impl BitAnd<u32> for Effect {
    type Output = u32;

    fn bitand(self, rhs: u32) -> u32 {
        self as u32 & rhs
    }
}

impl BitAnd<Effect> for Effect {
    type Output = u32;

    fn bitand(self, rhs: Self) -> u32 {
        self as u32 & rhs as u32
    }
}

impl BitAnd<Effect> for u32 {
    type Output = Self;

    fn bitand(self, rhs: Effect) -> Self {
        self & rhs as u32
    }
}

// (imdaveho) NOTE: Clone for moving parsed events over channels.
// See: crate::terminal::dispatch::input_handle
#[derive(Clone)]
pub enum InputEvent {
    Keyboard(KeyEvent),
    Mouse(MouseEvent),
    CursorPos(i16, i16),
    Unsupported,
}


#[derive(Copy, Clone)]
pub enum MouseEvent {
    Press(MouseButton, i16, i16),
    Release(i16, i16),
    Hold(i16, i16),
}

impl MouseEvent {
    pub fn enumerate(self) -> u8 {
        match self {
            Self::Press(btn, _, _) => match btn {
                MouseButton::Left => 28,
                MouseButton::Right => 29,
                MouseButton::Middle => 30,
                MouseButton::WheelUp => 31,
                MouseButton::WheelDown => 32,
            },
            Self::Release(_, _) => 33,
            Self::Hold(_, _) => 34,
        }
    }

    pub fn values(self) -> u32 {
        match self {
            Self::Press(_, col, row) => ((col as u32) << 16) | row as u32,
            Self::Release(col, row) => ((col as u32) << 16) | row as u32,
            Self::Hold(col, row) => ((col as u32) << 16) | row as u32,
        }
    }
}


#[derive(Copy, Clone)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    WheelUp,
    WheelDown,
}


#[derive(Copy, Clone)]
pub enum KeyEvent {
    Null,
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    F(u8),
    Char(char),
    Alt(char),
    Ctrl(char),
    Esc,
    CtrlLeft,
    CtrlRight,
    CtrlUp,
    CtrlDown,
    ShiftLeft,
    ShiftRight,
    ShiftUp,
    ShiftDown,
}

impl KeyEvent {
    pub fn enumerate(self) -> u8 {
        match self {
            Self::Null => 0,
            Self::Backspace => 1,
            Self::Enter => 2,
            Self::Left => 3,
            Self::Right => 4,
            Self::Up => 5,
            Self::Down => 6,
            Self::Home => 7,
            Self::End => 8,
            Self::PageUp => 9,
            Self::PageDown => 10,
            Self::Tab => 11,
            Self::BackTab => 12,
            Self::Delete => 13,
            Self::Insert => 14,
            Self::F(_) => 15,
            Self::Char(_) => 16,
            Self::Alt(_) => 17,
            Self::Ctrl(_) => 18,
            Self::Esc => 19,
            Self::CtrlLeft => 20,
            Self::CtrlRight => 21,
            Self::CtrlUp => 22,
            Self::CtrlDown => 23,
            Self::ShiftLeft => 24,
            Self::ShiftRight => 25,
            Self::ShiftUp => 26,
            Self::ShiftDown => 27,
        }
    }

    pub fn values(self) -> u32 {
        match self {
            Self::F(n) => n as u32,
            Self::Char(c) => c as u32,
            Self::Alt(c) => c as u32,
            Self::Ctrl(c) => c as u32,
            _ => 0,
        }
    }
}