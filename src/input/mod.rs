use std::io::{Result, stdin};
use crate::shared::{TtyResult, Handle};

#[cfg(unix)]
mod linux;

#[cfg(unix)]
use linux::{
    _read_char as read_char,
    _read_sync as read_sync,
    _read_async as read_async,
    _read_until_async as read_until_async,
    _enable_mouse_mode as enable_mouse_input,
    _disable_mouse_mode as disable_mouse_input,
    AsyncReader,
    SyncReader, 
};

#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use windows::{
    _read_char as read_char,
    _read_sync as read_sync,
    _read_async as read_async,
    _read_until_async as read_until_async,
    _enable_mouse_mode as enable_mouse_input,
    AsyncReader,
    SyncReader, 
};


// pub fn read_line() -> Result<String> {
//     let mut rv = String::new();
//     stdin().read_line(&mut rv)?;
//     let len = rv.trim_end_matches(&['\r', '\n'][..]).len();
//     rv.truncate(len);
//     Ok(rv)
// }


#[derive(Debug, PartialOrd, PartialEq, Hash, Clone)]
pub enum InputEvent {
    Keyboard(KeyEvent),
    Mouse(MouseEvent),
    Unsupported(Vec<u8>),
    Unknown,
}

#[derive(Debug, PartialOrd, PartialEq, Hash, Clone, Copy)]
pub enum MouseEvent {
    Press(MouseButton, u16, u16),
    Release(u16, u16),
    Hold(u16, u16),
    Unknown,
}

#[derive(Debug, PartialOrd, PartialEq, Hash, Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    WheelUp,
    WheelDown,
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Hash, Clone)]
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


// pub fn read_char() -> Result<char> {
//     #[cfg(unix)] {
//         linux::_read_char()
//     }

//     #[cfg(windows)] {
//         windows::_read_char()
//     }
// }

// pub fn read_async() -> AsyncReader {
//     #[cfg(unix)] {
//         linux::_read_async()
//     }

//     #[cfg(windows)] {
//         windows::_read_async()
//     }
// }

// pub fn read_sync() -> SyncReader {
//     #[cfg(unix)] {
//         linux::_read_sync()
//     }

//     #[cfg(windows)] {
//         windows::_read_sync()
//     }
// }

// pub fn read_until_async(delimiter: u8) -> AsyncReader {
//     #[cfg(unix)] {
//         linux::_read_until_async(delimiter)
//     }

//     #[cfg(windows)] {
//         windows::_read_until_async(delimiter)
//     }

// }

// pub fn enable_mouse_input() -> TtyResult<()> {
//     #[cfg(unix)] {
//         linux::_enable_mouse_mode()
//     }

//     #[cfg(windows)] {
//         windows::_enable_mouse_mode()
//     }
// }

// #[cfg(unix)]
// pub fn disable_mouse_input() -> TtyResult<()> {
//     linux::_disable_mouse_mode()
// }
