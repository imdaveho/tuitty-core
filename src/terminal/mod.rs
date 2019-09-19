// TODO: 

use crate::common::{
    runtime::is_ansi_enabled,
    traits::{ 
        CommonCursor, CommonModifier, 
        CommonFormatter, CommonWriter 
    },
    enums::{ Clear, Style, Color },
};

pub mod ansi;
#[cfg(unix)]
pub mod unix;
#[cfg(windows)]
pub mod wincon;
#[cfg(windows)]
pub mod windows;

// #[cfg(unix)]
// pub use unix::UnixTerminal as Terminal;
// #[cfg(windows)]
// pub use windows::WindowsConsole as Terminal;

enum CommonTerminal {
    Ansi(ansi::AnsiTerminal),
    #[cfg(windows)]
    Win32(wincon::Win32Console)
}

impl CommonTerminal {
    pub fn new() -> CommonTerminal {
        if is_ansi_enabled() {
            CommonTerminal::Ansi(ansi::AnsiTerminal::new())
        } else {
            #[cfg(windows)] 
            CommonTerminal::Win32(wincon::Win32Console::new())
        }
    }
}

impl CommonCursor for CommonTerminal {
    fn goto(&self, col: i16, row: i16) {
        match self {
            CommonTerminal::Ansi(a) => a.goto(col, row),
            #[cfg(windows)]
            CommonTerminal::Win32(b) => b.goto(col, row),
        }
    }

    fn up(&self, n: i16) {
        match self {
            CommonTerminal::Ansi(a) => a.up(n),
            #[cfg(windows)]
            CommonTerminal::Win32(b) => b.up(n),
        }
    }

    fn down(&self, n: i16) {
        match self {
            CommonTerminal::Ansi(a) => a.down(n),
            #[cfg(windows)]
            CommonTerminal::Win32(b) => b.down(n),
        }
    }

    fn left(&self, n: i16) {
        match self {
            CommonTerminal::Ansi(a) => a.left(n),
            #[cfg(windows)]
            CommonTerminal::Win32(b) => b.left(n),
        }
    }

    fn right(&self, n: i16) {
        match self {
            CommonTerminal::Ansi(a) => a.right(n),
            #[cfg(windows)]
            CommonTerminal::Win32(b) => b.right(n),
        }
    }

    fn pos(&self) -> (i16, i16) {
        match self {
            CommonTerminal::Ansi(a) => a.pos(),
            #[cfg(windows)]
            CommonTerminal::Win32(b) => b.pos(),
        }
    }
}

impl CommonModifier for CommonTerminal {
    fn hide_cursor(&self) {
        match self {
            CommonTerminal::Ansi(a) => a.hide_cursor(),
            #[cfg(windows)]
            CommonTerminal::Win32(b) => b.hide_cursor(),
        }
    }

    fn show_cursor(&self) {
        match self {
            CommonTerminal::Ansi(a) => a.show_cursor(),
            #[cfg(windows)]
            CommonTerminal::Win32(b) => b.show_cursor(),
        }
    }

    fn enable_mouse(&self) {
        match self {
            CommonTerminal::Ansi(a) => a.enable_mouse(),
            #[cfg(windows)]
            CommonTerminal::Win32(b) => b.enable_mouse(),
        }
    }

    fn disable_mouse(&self) {
        match self {
            CommonTerminal::Ansi(a) => a.disable_mouse(),
            #[cfg(windows)]
            CommonTerminal::Win32(b) => b.disable_mouse(),
        }
    }

    fn enable_alt(&self) {
        match self {
            CommonTerminal::Ansi(a) => a.enable_alt(),
            #[cfg(windows)]
            CommonTerminal::Win32(b) => b.enable_alt(),
        }
    }

    fn disable_alt(&self) {
        match self {
            CommonTerminal::Ansi(a) => a.disable_alt(),
            #[cfg(windows)]
            CommonTerminal::Win32(b) => b.disable_alt(),
        }
    }
}

impl CommonFormatter for CommonTerminal {
    fn clear(&self, method: Clear) {
        match self {
            CommonTerminal::Ansi(a) => a.clear(method),
            #[cfg(windows)]
            CommonTerminal::Win32(b) => b.clear(method),
        }
    }

    fn resize(&self, w: i16, h: i16) {
        match self {
            CommonTerminal::Ansi(a) => a.resize(w, h),
            #[cfg(windows)]
            CommonTerminal::Win32(b) => b.resize(w, h),
        }
    }

    fn set_style(&self, style: Style) {
        match self {
            CommonTerminal::Ansi(a) => a.set_style(style),
            #[cfg(windows)]
            CommonTerminal::Win32(b) => b.set_style(style),
        }
    }

    fn set_styles(&self, fg: Color, bg: Color, fx: u32) {
        match self {
            CommonTerminal::Ansi(a) => a.set_styles(fg, bg, fx),
            #[cfg(windows)]
            CommonTerminal::Win32(b) => b.set_styles(fg, bg, fx),
        }
    }

    fn reset_styles(&self) {
        match self {
            CommonTerminal::Ansi(a) => a.reset_styles(),
            #[cfg(windows)]
            CommonTerminal::Win32(b) => b.reset_styles(),
        }
    }
}

impl CommonWriter for CommonTerminal {
    fn prints(&self, content: &str) {
        match self {
            CommonTerminal::Ansi(a) => a.prints(content),
            #[cfg(windows)]
            CommonTerminal::Win32(b) => b.prints(content),
        }
    }

    fn flush(&self) {
        match self {
            CommonTerminal::Ansi(a) => a.flush(),
            #[cfg(windows)]
            CommonTerminal::Win32(b) => b.flush(),
        }
    }

    fn printf(&self, content: &str) {
        match self {
            CommonTerminal::Ansi(a) => a.printf(content),
            #[cfg(windows)]
            CommonTerminal::Win32(b) => b.printf(content),
        }
    }
}