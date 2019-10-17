// Windows Console API specific functions.

pub mod cursor;
pub mod output;
pub mod screen;
mod style;
mod mouse;

// #[cfg(test)]
// mod tests;

mod handle;
pub use handle::Handle;

use crate::common::traits::{
    CommandCursor, CommandModifier,
    CommandFormatter, CommandWriter
};
use crate::common::enums::{ Clear, Style, Color };


pub struct Win32Console {
    style: style::ConsoleOutput,
    alternate: Handle,
};

impl Win32Console {
    pub fn new() -> Win32Console {
        Win32Console {
            style: style::ConsoleOutput::new(),
            original_mode: output::get_mode()
                .expect("Error fetching mode from $STDOUT"),
            alternate: Handle::buffer()
                .expect("Error creating alternate Console buffer"),
    }
}

impl CommandCursor for Win32Console {
    fn goto(&self, col: i16, row: i16) {
        let (mut col, mut row) = (col, row);
        if col < 0 { col = col.abs() }
        if row < 0 { row = row.abs() }
        cursor::goto(col, row)
            .expect("Error setting the cursor position");
    }

    fn up(&self, n: i16) {
        let mut n = n;
        if n < 0 { n = n.abs() }
        cursor::move_up(n)
            .expect(&format!("Error moving the cursor up by {}", n));
    }

    fn down(&self, n: i16) {
        let mut n = n;
        if n < 0 { n = n.abs() }
        cursor::move_down(n)
            .expect(&format!("Error moving the cursor down by {}", n));
    }

    fn left(&self, n: i16) {
        let mut n = n;
        if n < 0 { n = n.abs() }
        cursor::move_left(n)
            .expect(&format!("Error moving the cursor left by {}", n));
    }

    fn right(&self, n: i16) {
        let mut n = n;
        if n < 0 { n = n.abs() }
        cursor::move_right(n)
            .expect(&format!("Error moving the cursor right by {}", n));
    }
}

impl CommandFormatter for Win32Console {
    fn clear(&self, method: Clear) {
        screen::clear(method);
    }

    fn size(&self) -> (i16, i16) {
        screen::size()
    }

    fn resize(&self, w: i16, h: i16) {
        screen::resize(w, h)
            .expect("Error resizing the screen")
    }

    fn set_fg(&self, color: Color) {
        self.style.set_style(Style::Fg(color))
            .expect("Error setting console foreground");
    }

    fn set_bg(&self, color: Color) {
        self.style.set_style(Style::Bg(color))
            .expect("Error setting console background");
    }

    fn set_fx(&self, effects: u32) {
        self.style.set_style(Style::Fx(effects))
            .expect("Error setting console text attributes");
    }

    fn set_styles(&self, fg: Color, bg: Color, fx: u32) {
        self.style.set_styles(fg, bg, fx)
            .expect("Error setting console styles");
    }

    fn reset_styles(&self) {
        self.style.reset()
            .expect("Error unsetting console styles");
    }
}

impl CommandModifier for Win32Console {
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
        self.alternate.set_mode(&self.original_mode)
            .expect("Error setting alternate screen back to defaults");
        self.alternate.show().expect("Error switching to the alternate screen");
    }

    fn disable_alt(&self) {
        screen::disable_alt().expect("Error switching back to $STDOUT");
    }

    fn raw(&mut self) {
        output::enable_raw().expect("Error enabling raw mode");
    }

    fn cook(&mut self) {
        output::disable_raw().expect("Error disabling raw mode");
    }
}

impl CommandWriter for Win32Console {
    fn prints(&self, content: &str) {
        output::prints(content)
            .expect("Error writing to console");
    }
    // (imdaveho) NOTE: Win32 `printf` identical to `prints`
    fn printf(&self, content: &str) {
        output::prints(content)
            .expect("Error writing to console");
    }
    // (imdaveho) NOTE: Win32 flush is simply a no-op.
    fn flush(&self) { () }
}


pub fn is_ansi_enabled() -> bool {
    const TERMS: [&'static str; 15] = [
        "xterm",  // xterm, PuTTY, Mintty
        "rxvt",   // RXVT
        "eterm",  // Eterm
        "screen", // GNU screen, tmux
        "tmux",   // tmux
        "vt100", "vt102", "vt220", "vt320",   // DEC VT series
        "ansi",    // ANSI
        "scoansi", // SCO ANSI
        "cygwin",  // Cygwin, MinGW
        "linux",   // Linux console
        "konsole", // Konsole
        "bvterm",  // Bitvise SSH Client
    ];

    let matched_terms = match std::env::var("TERM") {
        Ok(val) => val != "dumb" || TERMS.contains(&val.as_str()),
        Err(_) => false,
    };

    if matched_terms {
        return true
    } else {
        #[cfg(windows)] {
        let enable_vt = 0x0004;
        let handle = match Handle::stdout() {
            Ok(h) => h,
            Err(_) => return false,
        };
        let mode = match handle.get_mode() {
            Ok(m) => m,
            Err(_) => return false,
        };
        match handle.set_mode(&(mode | enable_vt)) {
            Ok(_) => return true,
            Err(_) => return false,
        }}
        #[cfg(not(windows))]
        return false
    }
}
