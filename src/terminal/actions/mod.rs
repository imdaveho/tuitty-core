// This module unifies essential terminal commands
// across Ansi and Wincon APIs through a single interface.
//
// TODO: What is an `TerminalAction`?

use crate::common::{
    enums::{ Clear, Color },
    traits::{ CursorActor, ModeActor, ViewActor, OutputActor },
};

pub mod ansi;

#[cfg(windows)]
pub mod wincon;


pub enum TerminalAction {
    Ansi(ansi::AnsiTerminal),
    #[cfg(windows)]
    Win32(wincon::Win32Console)
}


impl TerminalAction {
    pub fn new() -> TerminalAction {
        #[cfg(windows)] { if !wincon::is_ansi_enabled() {
            return TerminalAction::Win32(wincon::Win32Console::new());
        }}
        return TerminalAction::Ansi(ansi::AnsiTerminal::new());
    }
}

impl CursorActor for TerminalAction {
    fn goto(&self, col: i16, row: i16) {
        match self {
            TerminalAction::Ansi(a) => a.goto(col, row),
            #[cfg(windows)]
            TerminalAction::Win32(b) => b.goto(col, row),
        }
    }

    fn up(&self, n: i16) {
        match self {
            TerminalAction::Ansi(a) => a.up(n),
            #[cfg(windows)]
            TerminalAction::Win32(b) => b.up(n),
        }
    }

    fn down(&self, n: i16) {
        match self {
            TerminalAction::Ansi(a) => a.down(n),
            #[cfg(windows)]
            TerminalAction::Win32(b) => b.down(n),
        }
    }

    fn left(&self, n: i16) {
        match self {
            TerminalAction::Ansi(a) => a.left(n),
            #[cfg(windows)]
            TerminalAction::Win32(b) => b.left(n),
        }
    }

    fn right(&self, n: i16) {
        match self {
            TerminalAction::Ansi(a) => a.right(n),
            #[cfg(windows)]
            TerminalAction::Win32(b) => b.right(n),
        }
    }
}

impl ViewActor for TerminalAction {
    fn clear(&self, method: Clear) {
        match self {
            TerminalAction::Ansi(a) => a.clear(method),
            #[cfg(windows)]
            TerminalAction::Win32(b) => b.clear(method),
        }
    }

    fn size(&self) -> (i16, i16) {
        match self {
            TerminalAction::Ansi(a) => a.size(),
            #[cfg(windows)]
            TerminalAction::Win32(b) => b.size(),
        }
    }

    fn resize(&self, w: i16, h: i16) {
        match self {
            TerminalAction::Ansi(a) => a.resize(w, h),
            #[cfg(windows)]
            TerminalAction::Win32(b) => b.resize(w, h),
        }
    }

    fn set_fg(&self, color: Color) {
        match self {
            TerminalAction::Ansi(a) => a.set_fg(color),
            #[cfg(windows)]
            TerminalAction::Win32(b) => b.set_fg(color),
        }
    }

    fn set_bg(&self, color: Color) {
        match self {
            TerminalAction::Ansi(a) => a.set_bg(color),
            #[cfg(windows)]
            TerminalAction::Win32(b) => b.set_bg(color),
        }
    }

    fn set_fx(&self, effects: u32) {
        match self {
            TerminalAction::Ansi(a) => a.set_fx(effects),
            #[cfg(windows)]
            TerminalAction::Win32(b) => b.set_fx(effects),
        }
    }

    fn set_styles(&self, fg: Color, bg: Color, fx: u32) {
        match self {
            TerminalAction::Ansi(a) => a.set_styles(fg, bg, fx),
            #[cfg(windows)]
            TerminalAction::Win32(b) => b.set_styles(fg, bg, fx),
        }
    }

    fn reset_styles(&self) {
        match self {
            TerminalAction::Ansi(a) => a.reset_styles(),
            #[cfg(windows)]
            TerminalAction::Win32(b) => b.reset_styles(),
        }
    }
}

impl ModeActor for TerminalAction {
    fn hide_cursor(&self) {
        match self {
            TerminalAction::Ansi(a) => a.hide_cursor(),
            #[cfg(windows)]
            TerminalAction::Win32(b) => b.hide_cursor(),
        }
    }

    fn show_cursor(&self) {
        match self {
            TerminalAction::Ansi(a) => a.show_cursor(),
            #[cfg(windows)]
            TerminalAction::Win32(b) => b.show_cursor(),
        }
    }

    fn enable_mouse(&self) {
        match self {
            TerminalAction::Ansi(a) => a.enable_mouse(),
            #[cfg(windows)]
            TerminalAction::Win32(b) => b.enable_mouse(),
        }
    }

    fn disable_mouse(&self) {
        match self {
            TerminalAction::Ansi(a) => a.disable_mouse(),
            #[cfg(windows)]
            TerminalAction::Win32(b) => b.disable_mouse(),
        }
    }

    fn enable_alt(&self) {
        match self {
            TerminalAction::Ansi(a) => a.enable_alt(),
            #[cfg(windows)]
            TerminalAction::Win32(b) => b.enable_alt(),
        }
    }

    fn disable_alt(&self) {
        match self {
            TerminalAction::Ansi(a) => a.disable_alt(),
            #[cfg(windows)]
            TerminalAction::Win32(b) => b.disable_alt(),
        }
    }

    fn raw(&self) {
        match self {
            TerminalAction::Ansi(a) => a.raw(),
            #[cfg(windows)]
            TerminalAction::Win32(b) => b.raw(),
        }
    }

    fn cook(&self) {
        match self {
            TerminalAction::Ansi(a) => a.cook(),
            #[cfg(windows)]
            TerminalAction::Win32(b) => b.cook(),
        }
    }
}

impl OutputActor for TerminalAction {
    fn prints(&self, content: &str) {
        match self {
            TerminalAction::Ansi(a) => a.prints(content),
            #[cfg(windows)]
            TerminalAction::Win32(b) => b.prints(content),
        }
    }

    fn flush(&self) {
        match self {
            TerminalAction::Ansi(a) => a.flush(),
            #[cfg(windows)]
            TerminalAction::Win32(b) => b.flush(),
        }
    }

    fn printf(&self, content: &str) {
        match self {
            TerminalAction::Ansi(a) => a.printf(content),
            #[cfg(windows)]
            TerminalAction::Win32(b) => b.printf(content),
        }
    }
}
