// TODO:

use crate::common::enums::{ Clear, Direction, Style, Color };

#[cfg(unix)]
use crate::terminal::unix::{ SyncReader, AsyncReader };

#[cfg(windows)]
use crate::terminal::windows::{ SyncReader, AsyncReader };


pub trait CommonCursor {
    fn goto(&self, col: i16, row: i16);
    fn up(&self, n: i16);
    fn down(&self, n: i16);
    fn left(&self, n: i16);
    fn right(&self, n: i16);
    fn pos(&self) -> (i16, i16);
}

pub trait TerminalCursor {
    fn goto(&mut self, col: i16, row: i16);
    fn up(&mut self);
    fn down(&mut self);
    fn left(&mut self);
    fn right(&mut self);
    fn pos(&mut self) -> (i16, i16);
    fn mark_pos(&mut self);
    fn load_pos(&mut self);
    fn moves(&mut self, direction: Direction);
}

pub trait CommonModifier {
    fn hide_cursor(&self);
    fn show_cursor(&self);
    fn enable_mouse(&self);
    fn disable_mouse(&self);
    fn enable_alt(&self);
    fn disable_alt(&self);
}

pub trait TerminalModifier {
    fn raw(&mut self);
    fn cook(&mut self);
    fn hide_cursor(&mut self);
    fn show_cursor(&mut self);
    fn enable_mouse(&mut self);
    fn disable_mouse(&mut self);
}

pub trait CommonFormatter {
    fn clear(&self, method: Clear);
    fn resize(&self, w: i16, h: i16);
    fn set_style(&self, style: Style);
    fn set_styles(&self, fg: Color, bg: Color, fx: u32);
    fn reset_styles(&self);
}

pub trait TerminalFormatter {
    fn clear(&mut self, method: Clear);
    fn resize(&mut self, w: i16, h: i16);
    fn set_fg(&mut self, color: Color);
    fn set_bg(&mut self, color: Color);
    fn set_fx(&mut self, effects: u32);
    fn set_styles(&mut self, fg: Color, bg: Color, fx: u32);
    fn reset_styles(&mut self);
    fn screen_pos(&self) -> (i16, i16);
    fn screen_size(&self) -> (i16, i16);
}

pub trait CommonWriter {
    fn prints(&self, content: &str);
    fn flush(&self);
    fn printf(&self, content: &str);
}

pub trait TerminalWriter {
    fn prints(&mut self, content: &str);
    fn flush(&mut self);
    fn printf(&mut self, content: &str);
}

pub trait TerminalInput {
    fn read_char(&self) -> char;
    fn read_sync(&self) -> SyncReader;
    fn read_async(&self) -> AsyncReader;
    fn read_until_async(&self, delimiter: u8) -> AsyncReader;
}

pub trait TerminalSwitcher {
    fn switch(&mut self);
    fn to_main(&mut self);
    fn switch_to(&mut self, index: usize);
}
