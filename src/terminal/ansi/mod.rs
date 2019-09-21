// ANSI specific functions.

mod cursor;
mod output;
pub mod screen;
mod style;
mod mouse;

#[cfg(test)]
mod tests;

mod cell;
pub use cell::CellInfoCache;

use crate::common::traits::{
    CommonCursor, CommonWriter,
    CommonFormatter, CommonModifier
};
use crate::common::enums::{ Clear, Style, Color };


pub struct AnsiTerminal();

impl AnsiTerminal {
    pub fn new() -> AnsiTerminal {
        AnsiTerminal()
    }
}

impl CommonCursor for AnsiTerminal {
    fn goto(&self, col: i16, row: i16) {
        if col < 0 || row < 0 { return }
        output::prints(&cursor::goto(col, row));
    }

    fn up(&self, n: i16) {
        if n < 0 { return }
        output::prints(&cursor::move_up(n));
    }

    fn down(&self, n: i16) {
        if n < 0 { return }
        output::prints(&cursor::move_down(n));
    }

    fn left(&self, n: i16) {
        if n < 0 { return }
        output::prints(&cursor::move_left(n));
    }

    fn right(&self, n: i16) {
        if n < 0 { return }
        output::prints(&cursor::move_right(n));
    }

    // (imdaveho) NOTE: Just a bit of OS specific logic.
    fn pos(&self) -> (i16, i16) {
        #[cfg(unix)] {
            crate::terminal::unix::pos()
                .expect("Error reading cursor position (I/O related)")
        }

        #[cfg(windows)] {
            crate::terminal::wincon::cursor::pos()
                .expect("Error reading cursor position (Handle related)")
        }
    }
}

impl CommonFormatter for AnsiTerminal {
    fn clear(&self, method: Clear) {
        output::prints(&screen::clear(method));
        // match method {
        //     Clear::All => output::prints(&cursor::goto(0, 0)),
        //     Clear::CurrentLn => {
        //         let (_, row) = self.pos();
        //         output::prints(&cursor::goto(0, row));
        //     },
        //     _ => (),
        // }
    }

    fn resize(&self, w: i16, h: i16) {
        output::printf(&screen::resize(w, h));
    }

    fn set_style(&self, style: Style) {
        output::prints(&style::set_style(style));
    }

    fn set_styles(&self, fg: Color, bg: Color, fx: u32) {
        output::prints(&style::set_styles(fg, bg, fx));
    }

    fn reset_styles(&self) {
        output::prints(&style::reset());
    }
}

impl CommonModifier for AnsiTerminal {
    fn hide_cursor(&self) {
        output::prints(&cursor::hide_cursor());
    }

    fn show_cursor(&self) {
        output::prints(&cursor::show_cursor());
    }

    fn enable_mouse(&self) {
        output::prints(&mouse::enable_mouse_mode());
    }

    fn disable_mouse(&self) {
        output::prints(&mouse::disable_mouse_mode());
    }

    fn enable_alt(&self) {
        output::printf(&screen::enable_alt());
    }

    fn disable_alt(&self) {
        output::printf(&screen::disable_alt());
    }
}

impl CommonWriter for AnsiTerminal {
    fn prints(&self, content: &str) {
        output::prints(content);
    }

    fn printf(&self, content: &str) {
        output::printf(content);
    }

    fn flush(&self) {
        output::flush();
    }
}
