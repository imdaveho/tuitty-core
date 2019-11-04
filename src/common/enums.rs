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

// (imdaveho) NOTE: Clone for moving parsed events over channels.
// See: crate::terminal::dispatch::input_handle
#[derive(Clone)]
pub enum InputEvent {
    Keyboard(KeyEvent),
    Mouse(MouseEvent),
    Dispatch(StoreEvent),
    Unsupported,
}


#[derive(Clone)]
pub enum StoreEvent {
    Size(i16, i16),
    Coord(i16, i16),
    SysPos(i16, i16),
    GetCh(String),
}

#[derive(Clone)]
pub enum MouseEvent {
    Press(MouseButton, i16, i16),
    Release(i16, i16),
    Hold(i16, i16),
}


#[derive(Clone)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    WheelUp,
    WheelDown,
}


#[derive(Clone)]
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
    Suspend(usize),
    Transmit(usize),
    Stop(usize),
    Lock(usize),
    Unlock,
    Signal(Action),
    Request(State)
}


pub enum Action {
    // CURSOR
    Goto(i16, i16),
    Up(i16),
    Down(i16),
    Left(i16),
    Right(i16),
    // SCREEN/OUTPUT
    Clear(Clear),
    Prints(String),
    Printf(String),
    Flush,
    // STYLE
    SetFx(u32),
    SetFg(Color),
    SetBg(Color),
    SetStyles(Color, Color, u32),
    ResetStyles,
    // STATEFUL/MODES
    HideCursor,
    ShowCursor,
    EnableMouse,
    DisableMouse,
    EnableAlt,
    DisableAlt,
    Raw,
    Cook,
    // STORE OPS
    Refresh,
    Switch,
    SwitchTo(usize),
    Resize,
    SyncSize(i16, i16),
    SyncTabSize(usize),
    SyncMarker(i16, i16),
    Jump,
}


pub enum State {
    Size(usize),
    Coord(usize),
    SysPos(usize),
    GetCh(usize),
    // ScreenKey(usize),
}
