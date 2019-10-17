// ANSI functions for configuring the terminal size and clearing the screen.

use crate::common::enums::Clear;


pub fn clear(clr: Clear) -> String {
    match clr {
        Clear::All => {
            "\x1B[2J".to_string()
        }
        Clear::CursorDn => {
            "\x1B[J".to_string()
        }
        Clear::CursorUp => {
            "\x1B[1J".to_string()
        }
        Clear::CurrentLn => {
            "\x1B[2K".to_string()
        }
        Clear::NewLn => {
            "\x1B[K".to_string()
        }
    }
}

pub fn resize(w: i16, h: i16) -> String {
    format!("\x1B[8;{};{}t", h, w)
}


pub fn enable_alt() -> String {
    "\x1B[?1049h".to_string()
}


pub fn disable_alt() -> String {
    "\x1B[?1049l".to_string()
}

#[cfg(unix)]
use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};

#[cfg(unix)]
pub fn size() -> (i16, i16) {
    // Reference source:
    // http://rosettacode.org/wiki/Terminal_control/Dimensions#Library:_BSD_libc
    let mut size = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ.into(), &mut size) };

    if r == 0 {
        (size.ws_col as i16, size.ws_row as i16)
    } else {
        (0, 0)
    }
}
