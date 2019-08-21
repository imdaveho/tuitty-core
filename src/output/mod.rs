//! Handles the printing to the visible part
//! of the TTY. It also handles styling and
//! enabling/disabling raw modes.
use std::str::FromStr;
use std::io::Result;

#[cfg(unix)]
use crate::{Termios, TtyResult};

#[cfg(windows)]
use crate::shared::{Handle, ConsoleInfo};

#[cfg(windows)]
use crate::Termios;

#[cfg(unix)]
mod linux;
#[cfg(unix)]
pub use linux::{
    TextStyle,
    _set_fg as fg,
    _set_bg as bg,
    _set_tx as txsty,
    _set_all as set_style,
    _reset as reset,
    _enable_raw as enable_raw,
    _get_terminal_attr as get_mode,
    _write as writeout,
    _set_terminal_attr as set_mode,
};

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::{
    TextStyle,
    _set_fg as fg,
    _set_bg as bg,
    _set_tx as txsty,
    _set_all as set_style,
    _reset as reset,
    _enable_raw as enable_raw,
    _get_terminal_attr as get_mode,
    _write as writeout,
    _disable_raw as disable_raw,
};


enum Style {
    Fg(Color),
    Bg(Color),
    Tx(TextStyle),
}

// Enum with the different colors to color your test and terminal.
// #[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[derive(PartialEq)]
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

    Rgb {
        r: u8,
        g: u8,
        b: u8,
    },
    AnsiValue(u8),
}

// In order to do something like this: `Color::from("blue")`
impl From<&str> for Color {
    fn from(src: &str) -> Self {
        src.parse().unwrap_or(Color::White)
    }
}

// The FromStr Trait provides the .parse() method
impl FromStr for Color {
    type Err = ();
    fn from_str(src: &str) -> ::std::result::Result<Self, Self::Err> {
        match src.as_ref() {
            "black" => Ok(Color::Black),
            "dark_grey" => Ok(Color::DarkGrey),
            "red" => Ok(Color::Red),
            "dark_red" => Ok(Color::DarkRed),
            "green" => Ok(Color::Green),
            "dark_green" => Ok(Color::DarkGreen),
            "yellow" => Ok(Color::Yellow),
            "dark_yellow" => Ok(Color::DarkYellow),
            "blue" => Ok(Color::Blue),
            "dark_blue" => Ok(Color::DarkBlue),
            "magenta" => Ok(Color::Magenta),
            "dark_magenta" => Ok(Color::DarkMagenta),
            "cyan" => Ok(Color::Cyan),
            "dark_cyan" => Ok(Color::DarkCyan),
            "white" => Ok(Color::White),
            "grey" => Ok(Color::Grey),
            "reset" => Ok(Color::Reset),
            _ => Ok(Color::Reset),
        }
    }
}
