// ANSI specific functions.

mod cursor;
mod screen;
mod output;
mod style;
mod mouse;
mod cell;

use crate::terminal::PartialTerminalApi;
use crate::common::enums::{ Clear, Direction, Style, Color };


pub struct AnsiTerminal()

impl PartialTerminalApi for AnsiTerminal {
    pub new() -> AnsiTerminal {
        AnsiTerminal()
    }

    pub fn clear(&self, method: Clear) {
        output::prints(&screen::clear(method));
        match method {
            Clear::All => cursor::goto(0, 0),
            Clear::CurrentLn => {
                let (_, row) = self.pos();
                cursor::goto(0, row);
            },
            _ => (),
        }
        Ok(())
    }

    pub fn resize(&self, w: i16, h: i16) {
        output::printf(&screen::resize(w, h));
    }

    pub fn goto(&self, col: i16, row: i16) {
        if col < 0 || row < 0 { return }
        output::prints(&cursor::goto(col, row));
    }

    pub fn up(&self) {
        output::prints(&cursor::move_up(1));
    }
    
    pub fn dn(&self) {
        output::prints(&cursor::move_dn(1));
    }
    
    pub fn left(&self) {
        output::prints(&cursor::move_left(1));
    }
    
    pub fn right(&self) {
        output::prints(&cursor::move_right(1));
    }

    pub fn moves(&self, direction: Direction) {
        match direction {
            Direction::Up(n) => {
                if n < 0 { return } 
                else { output::prints(&cursor::move_up(n)) }
            }
            Direction::Dn(n) => {
                if n < 0 { return } 
                else { output::prints(&cursor::move_dn(n)) }
            }
            Direction::Left(n) => {
                if n < 0 { return } 
                else { output::prints(&cursor::move_left(n)) }
            }
            Direction::Right(n) => {
                if n < 0 { return } 
                else { output::prints(&cursor::move_right(n)) }
            }
        }
    }

    pub fn hide_cursor(&self) {
        output::prints(&cursor::hide_cursor());
    }

    pub fn show_cursor(&self) {
        output::prints(&cursor::show_cursor());
    }

    pub fn set_style(&self, style: Style) {
        output::prints(&style::set_style(style));
    }

    pub fn set_styles(&self, fg: Color, bg: Color, fx: u32) {
        output::prints(&style::set_styles(fg, bg, fx));
    }

    pub fn unset_styles(&self) {
        output::prints(&style::reset());
    }

    pub fn enable_mouse(&self) {
        output::prints(&mouse::enable_mouse_mode());
    }

    pub fn disable_mouse(&self) {
        output.prints(&mouse::disable_mouse_mode());
    }

    pub fn prints(&self, content: &str) {
        output::prints(content);
    }

    pub fn pos(&self) -> (i16, i16) {
        #[cfg(windows)]
        crate::terminal::wincon::cursor::pos()
            .expect("Error reading cursor position (Handle related)")
        
        #[cfg(unix)]
        crate::terminal::unix::pos()
            .expect("Error reading cursor position (I/O related)")
    }
}