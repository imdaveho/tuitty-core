// This module unifies basic terminal commands across Ansi and Wincon APIs
// through a single interface.

use crate::common::{
    traits::{
        CommandCursor, CommandModifier,
        CommandFormatter, CommandWriter
    },
    enums::{ Clear, Color },
};

pub mod ansi;

#[cfg(windows)]
pub mod wincon;


pub enum Commands {
    Ansi(ansi::AnsiTerminal),
    #[cfg(windows)]
    Win32(wincon::Win32Console)
}


impl Commands {
    pub fn new() -> Commands {
        #[cfg(windows)] { if !wincon::is_ansi_enabled() {
            return Commands::Win32(wincon::Win32Console::new());
        }}
        return Commands::Ansi(ansi::AnsiTerminal::new());
    }
}

impl CommandCursor for Commands {
    fn goto(&self, col: i16, row: i16) {
        match self {
            Commands::Ansi(a) => a.goto(col, row),
            #[cfg(windows)]
            Commands::Win32(b) => b.goto(col, row),
        }
    }

    fn up(&self, n: i16) {
        match self {
            Commands::Ansi(a) => a.up(n),
            #[cfg(windows)]
            Commands::Win32(b) => b.up(n),
        }
    }

    fn down(&self, n: i16) {
        match self {
            Commands::Ansi(a) => a.down(n),
            #[cfg(windows)]
            Commands::Win32(b) => b.down(n),
        }
    }

    fn left(&self, n: i16) {
        match self {
            Commands::Ansi(a) => a.left(n),
            #[cfg(windows)]
            Commands::Win32(b) => b.left(n),
        }
    }

    fn right(&self, n: i16) {
        match self {
            Commands::Ansi(a) => a.right(n),
            #[cfg(windows)]
            Commands::Win32(b) => b.right(n),
        }
    }
}

impl CommandModifier for Commands {
    fn hide_cursor(&self) {
        match self {
            Commands::Ansi(a) => a.hide_cursor(),
            #[cfg(windows)]
            Commands::Win32(b) => b.hide_cursor(),
        }
    }

    fn show_cursor(&self) {
        match self {
            Commands::Ansi(a) => a.show_cursor(),
            #[cfg(windows)]
            Commands::Win32(b) => b.show_cursor(),
        }
    }

    fn enable_mouse(&self) {
        match self {
            Commands::Ansi(a) => a.enable_mouse(),
            #[cfg(windows)]
            Commands::Win32(b) => b.enable_mouse(),
        }
    }

    fn disable_mouse(&self) {
        match self {
            Commands::Ansi(a) => a.disable_mouse(),
            #[cfg(windows)]
            Commands::Win32(b) => b.disable_mouse(),
        }
    }

    fn enable_alt(&self) {
        match self {
            Commands::Ansi(a) => a.enable_alt(),
            #[cfg(windows)]
            Commands::Win32(b) => b.enable_alt(),
        }
    }

    fn disable_alt(&self) {
        match self {
            Commands::Ansi(a) => a.disable_alt(),
            #[cfg(windows)]
            Commands::Win32(b) => b.disable_alt(),
        }
    }

    fn raw(&self) {
        match self {
            Commands::Ansi(a) => a.raw(),
            #[cfg(windows)]
            Commands::Win32(b) => b.raw(),
        }
    }

    fn cook(&self) {
        match self {
            Commands::Ansi(a) => a.cook(),
            #[cfg(windows)]
            Commands::Win32(b) => b.cook(),
        }
    }
}

impl CommandFormatter for Commands {
    fn clear(&self, method: Clear) {
        match self {
            Commands::Ansi(a) => a.clear(method),
            #[cfg(windows)]
            Commands::Win32(b) => b.clear(method),
        }
    }

    fn size(&self) -> (i16, i16) {
        match self {
            Commands::Ansi(a) => a.size(),
            #[cfg(windows)]
            Commands::Win32(b) => b.size(),
        }
    }

    fn resize(&self, w: i16, h: i16) {
        match self {
            Commands::Ansi(a) => a.resize(w, h),
            #[cfg(windows)]
            Commands::Win32(b) => b.resize(w, h),
        }
    }

    fn set_fg(&self, color: Color) {
        match self {
            Commands::Ansi(a) => a.set_fg(color),
            #[cfg(windows)]
            Commands::Win32(b) => b.set_fg(color),
        }
    }

    fn set_bg(&self, color: Color) {
        match self {
            Commands::Ansi(a) => a.set_bg(color),
            #[cfg(windows)]
            Commands::Win32(b) => b.set_bg(color),
        }
    }

    fn set_fx(&self, effects: u32) {
        match self {
            Commands::Ansi(a) => a.set_fx(effects),
            #[cfg(windows)]
            Commands::Win32(b) => b.set_fx(effects),
        }
    }

    fn set_styles(&self, fg: Color, bg: Color, fx: u32) {
        match self {
            Commands::Ansi(a) => a.set_styles(fg, bg, fx),
            #[cfg(windows)]
            Commands::Win32(b) => b.set_styles(fg, bg, fx),
        }
    }

    fn reset_styles(&self) {
        match self {
            Commands::Ansi(a) => a.reset_styles(),
            #[cfg(windows)]
            Commands::Win32(b) => b.reset_styles(),
        }
    }
}

impl CommandWriter for Commands {
    fn prints(&self, content: &str) {
        match self {
            Commands::Ansi(a) => a.prints(content),
            #[cfg(windows)]
            Commands::Win32(b) => b.prints(content),
        }
    }

    fn flush(&self) {
        match self {
            Commands::Ansi(a) => a.flush(),
            #[cfg(windows)]
            Commands::Win32(b) => b.flush(),
        }
    }

    fn printf(&self, content: &str) {
        match self {
            Commands::Ansi(a) => a.printf(content),
            #[cfg(windows)]
            Commands::Win32(b) => b.printf(content),
        }
    }
}
