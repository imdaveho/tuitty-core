// Windows Console API specific functions.

mod cursor;
mod screen;
mod output;
mod style;
mod mouse;
mod handle;

pub use handle::Handle;
use crate::terminal::CommonTerminalApi;
use crate::common::enums::{ Clear, Direction, Style, Color };


pub struct Win32Console(style::ConsoleOutput)

impl CommonTerminalApi for Win32Console {
    pub fn new() -> Win32Console {
        Win32Console(style::ConsoleOutput::new())
    }

    pub fn clear(&self, method: Clear) {
        screen::clear(Clear);
        match method: {
            Clear::All => cursor::goto(0, 0),
            Clear::CurrentLn => {
                let (_ row) = self.pos();
                cursor::goto(0, row);
            },
            _ => (),
        }
        Ok(())
    }

    pub fn resize(&self, w: i16, h: i16) {
        screen::resize(w, h)
            .expect("Error resizing the screen")
    }

    pub fn goto(&self, col: i16, row: i16) {
        if col < 0 || row < 0 { return }
        cursor::goto(col, row)
            .expect("Error setting the cursor position");
    }

    pub fn up(&self, n: i16) {
        if n < 0 { return }
        cursor::move_up(n)
            .expect(format!("Error moving the cursor up by {}", n));
    }

    pub fn dn(&self, n: i16) {
        if n < 0 { return }
        cursor::move_dn(n)
            .expect(format!("Error moving the cursor down by {}", n));
    }

    pub fn left(&self, n: i16) {
        if n < 0 { return }
        cursor::move_left(n)
            .expect(format!("Error moving the cursor left by {}", n));
    }

    pub fn right(&self, n: i16) {
        if n < 0 { return }
        cursor::move_right(n)
            .expect(format!("Error moving the cursor right by {}", n));
    }

    pub fn hide_cursor(&self) {
        cursor::hide_cursor()
            .expect("Error setting cursor visibility to 0");
    }

    pub fn show_cursor(&self) {
        cursor::show_cursor()
            .expect("Error setting cursor visibility to 100");
    }

    pub fn set_style(&self, style: Style) {
        self.0.set_style(style)
            .expect("Error setting console style");
    }

    pub fn set_styles(&self, fg: Color, bg: Color, fx: u32) {
        self.0.set_styles(fg, bg, fx)
            .expect("Error setting console styles");
    }

    pub fn reset_styles(&self) {
        self.0.reset()
            .expect("Error unsetting console styles");
    }

    pub fn enable_mouse(&self) {
        mouse::enable_mouse_mode()
            .expect("Error enabling mouse mode");
    }

    pub fn disable_mouse(&self) {
        mouse::disable_mouse_mode()
            .expect("Error disabling mouse mode");
    }

    pub fn pos(&self) -> (i16, i16) {
        crate::terminal::wincon::cursor::pos()
            .expect("Error reading cursor position (Handle related)")
    }
    
    
    // (imdaveho) NOTE: The below are still common API methods as part of the 
    // struct. But left out of the trait to prevent duplication when combined
    // together into a single `Terminal` struct.
    pub fn prints(&self, content: &str) {
        output::prints(content)
            .expect("Error writing to console");
    }
    // (imdaveho) NOTE: Wincon printf identical to prints.
    pub fn printf(&self, content: &str) {
        output::prints(content)
            .expect("Error writing to console");
    }
    // (imdaveho) NOTE: Wincon flush is no-op.
    pub fn flush(&self) {
        ()
    }
}