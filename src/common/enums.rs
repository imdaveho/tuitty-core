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

// (imdaveho) NOTE: Copy/Clone needed to send over channels.
// See: crate::terminal::dispatch
#[derive(Copy, Clone)]
pub enum InputEvent {
    Keyboard(KeyEvent),
    Mouse(MouseEvent),
    Unknown,
    // Unsupported(Vec<u8>),
}


#[derive(Copy, Clone)]
pub enum MouseEvent {
    Press(MouseButton, i16, i16),
    Release(i16, i16),
    Hold(i16, i16),
    Unknown,
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

pub enum Cmd {
    Continue,
    Emit,
    Execute(Action),
}

pub enum Action {
    // CURSOR
    Goto = 1,
    Up = 2,
    Down = 3,
    Left = 4,
    Right = 5,
    HideCursor = 6,
    ShowCursor = 7,
    // SCREEN
    Clear = 8,
    Size = 9,
    Resize = 10,
    EnableAlt = 11,
    DisableAlt = 12,
    // OUTPUT
    Prints = 13,
    Printf = 14,
    Flush = 15,
    Raw = 16,
    Cook = 17,
    EnableMouse = 18,
    DisableMouse = 19,
    // STYLE
    SetFg = 20,
    SetBg = 21,
    SetFx = 22,
    SetStyles = 23,
    ResetStyles = 24,
}