// Windows Console API specific functions.

mod cursor;
mod screen;
mod output;
mod mouse;
mod style;
mod handle;

pub mod windows {
    pub use super::{
        handle::*,
        screen::size,
        output::{ enable_raw, disable_raw, get_mode, prints },
    };
}

use crate::common::enums::{ Clear, Style, Color };

// #[cfg(test)]
// mod tests;


pub struct Win32Console;

trait Win32Action {
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
    fn enable_alt(alternate: &windows::Handle, original_mode: &u32);
    fn disable_alt();
    // OUTPUT
    fn prints(content: &str);
    fn printf(content: &str);
    fn flush();
    fn raw();
    fn cook();
    fn enable_mouse();
    fn disable_mouse();
    // STYLE
    fn set_fg(default: u16, color: Color);
    fn set_bg(default: u16, color: Color);
    fn set_fx(effects: u32);
    fn set_styles(default: u16, fg: Color, bg: Color, fx: u32);
    fn reset_styles(default: u16);
}

impl Win32Action for Win32Console {
    // CURSOR
    fn goto(col: i16, row: i16) {
        let (mut col, mut row) = (col, row);
        if col < 0 { col = col.abs() }
        if row < 0 { row = row.abs() }
        cursor::goto(col, row)
            .expect("Error setting the cursor position");
    }

    fn up(n: i16) {
        let mut n = n;
        if n < 0 { n = n.abs() }
        cursor::move_up(n)
            .expect(&format!("Error moving the cursor up by {}", n));
    }

    fn down(n: i16) {
        let mut n = n;
        if n < 0 { n = n.abs() }
        cursor::move_down(n)
            .expect(&format!("Error moving the cursor down by {}", n));
    }

    fn left(n: i16) {
        let mut n = n;
        if n < 0 { n = n.abs() }
        cursor::move_left(n)
            .expect(&format!("Error moving the cursor left by {}", n));
    }

    fn right(n: i16) {
        let mut n = n;
        if n < 0 { n = n.abs() }
        cursor::move_right(n)
            .expect(&format!("Error moving the cursor right by {}", n));
    }

    fn hide_cursor() {
        cursor::hide_cursor()
            .expect("Error setting cursor visibility to 0");
    }

    fn show_cursor() {
        cursor::show_cursor()
            .expect("Error setting cursor visibility to 100");
    }

    // SCREEN
    fn clear(method: Clear) {
        screen::clear(method)
            .expect("Error clearing the screen");
    }

    fn size() -> (i16, i16) {
        screen::size()
    }

    fn resize(w: i16, h: i16) {
        screen::resize(w, h)
            .expect("Error resizing the screen")
    }

    fn enable_alt(alternate: &windows::Handle, original_mode: &u32) {
        alternate.set_mode(original_mode)
            .expect("Error setting alternate screen back to default");
        alternate.show().expect("Error switching to the alternate screen");
    }

    fn disable_alt() {
        screen::disable_alt().expect("Error switching back to $STDOUT");
    }

    // OUTPUT
    fn prints(content: &str) {
        output::prints(content)
            .expect("Error writing to console");
    }

    fn printf(content: &str) {
        // (imdaveho) NOTE: Win32 `printf` identical to `prints`
        output::prints(content)
            .expect("Error writing to console");
    }
    
    fn flush() {
        // (imdaveho) NOTE: Win32 flush is simply a no-op.
        () 
    }

    fn raw() {
        output::enable_raw().expect("Error enabling raw mode");
    }

    fn cook() {
        output::disable_raw().expect("Error disabling raw mode");
    }

    // MOUSE
    fn enable_mouse() {
        mouse::enable_mouse_mode()
            .expect("Error enabling mouse mode");
    }

    fn disable_mouse() {
        mouse::disable_mouse_mode()
            .expect("Error disabling mouse mode");
    }

    // STYLE
    fn set_fg(default: u16, color: Color) {
        style::set_style(default, Style::Fg(color))
            .expect("Error setting console foreground");
    }

    fn set_bg(default: u16, color: Color) {
        style::set_style(default, Style::Bg(color))
            .expect("Error setting console background");
    }

    fn set_fx(effects: u32) {
        style::set_style(0, Style::Fx(effects))
            .expect("Error setting console text attributes");
    }

    
    fn set_styles(default: u16, fg: Color, bg: Color, fx: u32) {
        style::set_styles(default, fg, bg, fx)
            .expect("Error setting console styles");
    }

    fn reset_styles(default: u16) {
        style::reset(default)
            .expect("Error unsetting console styles");
    }
}