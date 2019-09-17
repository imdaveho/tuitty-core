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