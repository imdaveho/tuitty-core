// Windows Console API specific functions.

pub mod cursor;
pub mod screen;
pub mod output;
mod style;
mod mouse;

mod handle;
pub use handle::Handle;

use crate::common::traits::{ 
    CommonCursor, CommonModifier, 
    CommonFormatter, CommonWriter
};
use crate::common::enums::{ Clear, Style, Color };


pub struct Win32Console(style::ConsoleOutput);

impl Win32Console {
    pub fn new() -> Win32Console {
        Win32Console(style::ConsoleOutput::new())
    }
}

impl CommonCursor for Win32Console {
    fn goto(&self, col: i16, row: i16) {
        if col < 0 || row < 0 { return }
        cursor::goto(col, row)
            .expect("Error setting the cursor position");
    }

    fn up(&self, n: i16) {
        if n < 0 { return }
        cursor::move_up(n)
            .expect(&format!("Error moving the cursor up by {}", n));
    }

    fn down(&self, n: i16) {
        if n < 0 { return }
        cursor::move_down(n)
            .expect(&format!("Error moving the cursor down by {}", n));
    }

    fn left(&self, n: i16) {
        if n < 0 { return }
        cursor::move_left(n)
            .expect(&format!("Error moving the cursor left by {}", n));
    }

    fn right(&self, n: i16) {
        if n < 0 { return }
        cursor::move_right(n)
            .expect(&format!("Error moving the cursor right by {}", n));
    }

    fn pos(&self) -> (i16, i16) {
        cursor::pos()
            .expect("Error reading cursor position (Handle related)")
    }
}

impl CommonFormatter for Win32Console {
    fn clear(&self, method: Clear) {
        screen::clear(method);
        // match method {
        //     Clear::All => cursor::goto(0, 0)
        //         .expect("Error setting the cursor position"),
        //     Clear::CurrentLn => {
        //         let (_, row) = self.pos();
        //         cursor::goto(0, row)
        //             .expect("Error setting the cursor position");
        //     },
        //     _ => (),
        // }
    }

    fn resize(&self, w: i16, h: i16) {
        screen::resize(w, h)
            .expect("Error resizing the screen")
    }

    fn set_style(&self, style: Style) {
        self.0.set_style(style)
            .expect("Error setting console style");
    }

    fn set_styles(&self, fg: Color, bg: Color, fx: u32) {
        self.0.set_styles(fg, bg, fx)
            .expect("Error setting console styles");
    }

    fn reset_styles(&self) {
        self.0.reset()
            .expect("Error unsetting console styles");
    }
}

impl CommonModifier for Win32Console {
    fn hide_cursor(&self) {
        cursor::hide_cursor()
            .expect("Error setting cursor visibility to 0");
    }

    fn show_cursor(&self) {
        cursor::show_cursor()
            .expect("Error setting cursor visibility to 100");
    }

    fn enable_mouse(&self) {
        mouse::enable_mouse_mode()
            .expect("Error enabling mouse mode");
    }

    fn disable_mouse(&self) {
        mouse::disable_mouse_mode()
            .expect("Error disabling mouse mode");
    }

    fn enable_alt(&self) {
        // (imdaveho) NOTE: This cannot be implemented here because the stored
        // `alternate` field and screen belong in the `WindowsConsole` struct.
        ()
    }

    fn disable_alt(&self) {
        screen::disable_alt().expect("Error switching back to $STDOUT");
    }
}

impl CommonWriter for Win32Console {
    fn prints(&self, content: &str) {
        output::prints(content)
            .expect("Error writing to console");
    }
    // (imdaveho) NOTE: Win32 printf identical to prints.
    fn printf(&self, content: &str) {
        output::prints(content)
            .expect("Error writing to console");
    }
    // (imdaveho) NOTE: Win32 flush is no-op.
    fn flush(&self) {
        ()
    }
}
