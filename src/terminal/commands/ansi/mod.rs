// ANSI specific functions.

mod cursor;
mod output;
mod screen;
mod style;
mod mouse;

// #[cfg(test)]
// mod tests;

use crate::common::traits::{
    CommandCursor, CommandWriter,
    CommandFormatter, CommandModifier
};
use crate::common::enums::{ Clear, Style, Color };

#[cfg(unix)]
use libc::termios as Termios;


pub struct AnsiTerminal {
    #[cfg(unix)]
    original_mode: Termios,
    #[cfg(windows)]
    original_mode: u32,
}

impl AnsiTerminal {
    pub fn new() -> AnsiTerminal {
        AnsiTerminal {
            #[cfg(unix)]
            original_mode: output::get_mode()
                .expect("Error fetching Termios"),
            #[cfg(windows)]
            original_mode: super::wincon::output::get_mode()
                .expect("Error fetching mode from $STDOUT"),
        }
    }
}

impl CommandCursor for AnsiTerminal {
    fn goto(&self, col: i16, row: i16) {
        let (mut col, mut row) = (col, row);
        if col < 0 { col = col.abs() }
        if row < 0 { row = row.abs() }
        output::prints(&cursor::goto(col, row));
    }

    fn up(&self, n: i16) {
        let mut n = n;
        if n < 0 { n = n.abs() }
        output::prints(&cursor::move_up(n));
    }

    fn down(&self, n: i16) {
        let mut n = n;
        if n < 0 { n = n.abs() }
        output::prints(&cursor::move_down(n));
    }

    fn left(&self, n: i16) {
        let mut n = n;
        if n < 0 { n = n.abs() }
        output::prints(&cursor::move_left(n));
    }

    fn right(&self, n: i16) {
        let mut n = n;
        if n < 0 { n = n.abs() }
        output::prints(&cursor::move_right(n));
    }
}

impl CommandFormatter for AnsiTerminal {
    fn clear(&self, method: Clear) {
        output::prints(&screen::clear(method));
    }

    fn size(&self) -> (i16, i16) {
        #[cfg(unix)] { screen::size() }

        #[cfg(windows)] {
            super::wincon::screen::size()
        }
    }

    fn resize(&self, w: i16, h: i16) {
        output::printf(&screen::resize(w, h));
    }

    fn set_fg(&self, color: Color) {
        output::prints(&style::set_style(Style::Fg(color)));
    }

    fn set_bg(&self, color: Color) {
        output::prints(&style::set_style(Style::Bg(color)));
    }

    fn set_fx(&self, effects: u32) {
        output::prints(&style::set_style(Style::Fx(effects)));
    }

    fn set_styles(&self, fg: Color, bg: Color, fx: u32) {
        output::prints(&style::set_styles(fg, bg, fx));
    }

    fn reset_styles(&self) {
        output::prints(&style::reset());
    }
}

impl CommandModifier for AnsiTerminal {
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

    #[cfg(unix)]
    fn raw(&self) {
        output::enable_raw().expect("Error enabling raw mode");
    }

    #[cfg(unix)]
    fn cook(&self) {
        output::set_mode(&self.original_mode)
            .expect("Error disabling raw mode");
    }
}

impl CommandWriter for AnsiTerminal {
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
