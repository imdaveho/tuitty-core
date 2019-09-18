// Windows Console API specific functions.

mod cursor;
mod screen;
mod output;
mod style;
mod mouse;
mod handle;

use crate::terminal::PartialTerminalApi;
use crate::common::enums::{ Clear, Direction, Style, Color };


pub struct Win32Console(style::ConsoleOutput)

impl PartialTerminalApi for Win32Console {
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

    pub fn up(&self) {
        cursor::move_up(1)
            .expect("Error moving the cursor up by 1");
    }

    pub fn dn(&self) {
        cursor::move_dn(1)
            .expect("Error moving the cursor down by 1");
    }

    pub fn left(&self) {
        cursor::move_left(1)
            .expect("Error moving the cursor left by 1");
    }

    pub fn right(&self) {
        cursor::move_right(1)
            .expect("Error moving the cursor right by 1");
    }

    pub fn moves(&self, direction: Direction) {
        let err_msg = "Error moving the cursor ";
        match direction {
            Direction::Up(n) => {
                if n < 0 { return }
                else { 
                    cursor::move_up(n)
                        .expect(format!("{} up by {}", err_msg, n));
                }
            }
            Direction::Dn(n) => {
                if n < 0 { return }
                else { 
                    cursor::move_dn(n)
                        .expect(format!("{} up down {}", err_msg, n));
                }
            }
            Direction::Left(n) => {
                if n < 0 { return }
                else { 
                    cursor::move_left(n)
                        .expect(format!("{} up left {}", err_msg, n));
                }
            }
            Direction::Right(n) => {
                if n < 0 { return }
                else {
                    cursor::move_right(n)
                        .expect(format!("{} up right {}", err_msg, n));
                }
            }
        }
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

    pub fn unset_styles(&self) {
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

    pub fn prints(&self, content: &str) {
        output::prints(content)
            .expect("Error writing to console");
    }
    
    pub fn pos(&self) -> (i16, i16) {
        crate::terminal::wincon::cursor::pos()
            .expect("Error reading cursor position (Handle related)")
    }
}