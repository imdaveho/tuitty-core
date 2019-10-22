// ANSI specific functions.

mod cursor;
mod screen;
mod style;
mod output;
mod mouse;

#[cfg(unix)]
pub mod unix {
    pub use libc::termios as Termios;
    pub use super::{
        screen::size,
        output::{ enable_raw, get_mode, set_mode },
    };
}

use crate::common::enums::{ Clear, Style, Color };

// #[cfg(test)]
// mod tests;


pub struct AnsiTerminal;

pub trait AnsiAction {
    // CURSOR
    fn goto(col: i16, row: i16);
    fn up(n: i16);
    fn down(n: i16);
    fn left(n: i16);
    fn right(n: i16);
    fn hide_cursor();
    fn show_cursor();
    // SCREEN
    fn clear(method: Clear);
    fn size() -> (i16, i16);
    fn resize(w: i16, h: i16);
    fn enable_alt();
    fn disable_alt();
    // OUTPUT
    fn prints(content: &str);
    fn printf(content: &str);
    fn flush();
    fn raw();
    #[cfg(unix)]
    fn cook(original_mode: &unix::Termios); 
    #[cfg(windows)]
    fn cook();
    fn enable_mouse();
    fn disable_mouse();
    // STYLE
    fn set_fg(color: Color);
    fn set_bg(color: Color);
    fn set_fx(effects: u32);
    fn set_styles(fg: Color, bg: Color, fx: u32);
    fn reset_styles();
}

impl AnsiAction for AnsiTerminal {
    // CURSOR
    fn goto(col: i16, row: i16) {
        let (mut col, mut row) = (col, row);
        if col < 0 { col = col.abs() }
        if row < 0 { row = row.abs() }
        output::prints(&cursor::goto(col, row));
    }

    fn up(n: i16) {
        let mut n = n;
        if n < 0 { n = n.abs() }
        output::prints(&cursor::move_up(n));
    }

    fn down(n: i16) {
        let mut n = n;
        if n < 0 { n = n.abs() }
        output::prints(&cursor::move_down(n));
    }

    fn left(n: i16) {
        let mut n = n;
        if n < 0 { n = n.abs() }
        output::prints(&cursor::move_left(n));
    }

    fn right(n: i16) {
        let mut n = n;
        if n < 0 { n = n.abs() }
        output::prints(&cursor::move_right(n));
    }

    fn hide_cursor() {
        output::prints(&cursor::hide_cursor());
    }

    fn show_cursor() {
        output::prints(&cursor::show_cursor());
    }

    // SCREEN
    fn clear(method: Clear) {
        output::prints(&screen::clear(method));
    }

    fn size() -> (i16, i16) {
        #[cfg(unix)] { 
            self::unix::size() 
        }

        #[cfg(windows)] { 
            super::wincon::windows::size()
        }
    }

    fn resize(w: i16, h: i16) {
        output::printf(&screen::resize(w, h));
    }

    fn enable_alt() {
        output::printf(&screen::enable_alt());
    }

    fn disable_alt() {
        output::printf(&screen::disable_alt());
    }

    // OUTPUT
    fn prints(content: &str) {
        output::prints(content);
    }

    fn printf(content: &str) {
        output::printf(content);
    }

    fn flush() {
        output::flush();
    }

    fn raw() {
        #[cfg(unix)] {
            self::unix::enable_raw()
            .expect("Error enabling raw mode");
        }

        #[cfg(windows)] {
            super::wincon::windows::enable_raw()
                .expect("Error enabling raw mode")
        }
    }

    #[cfg(unix)]
    fn cook(original_mode: &unix::Termios) {
        self::unix::set_mode(original_mode)
            .expect("Error disabling raw mode");
    }

    #[cfg(windows)]
    fn cook() {
        super::wincon::windows::disable_raw()
            .expect("Error disabling raw mode");
    }

    // MOUSE
    fn enable_mouse() {
        output::prints(&mouse::enable_mouse_mode());
    }

    fn disable_mouse() {
        output::prints(&mouse::disable_mouse_mode());
    }

    // STYLE
    fn set_fx(effects: u32) {
        output::prints(&style::set_style(Style::Fx(effects)));
    }

    fn set_fg(color: Color) {
        output::prints(&style::set_style(Style::Fg(color)));
    }

    fn set_bg(color: Color) {
        output::prints(&style::set_style(Style::Bg(color)));
    }

    fn set_styles(fg: Color, bg: Color, fx: u32) {
        output::prints(&style::set_styles(fg, bg, fx));
    }

    fn reset_styles() {
        output::prints(&style::reset());
    }
}