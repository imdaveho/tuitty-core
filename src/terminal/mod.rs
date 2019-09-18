mod ansi;

#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod wincon;

#[cfg(windows)]
mod windows;

use crate::common::enums::{ Clear, Direction, Style, Color };


trait PartialTerminalApi {
    fn clear(&self, method: Clear);
    fn resize(&self, w: i16, h: i16);
    fn goto(&self, col: i16, row: i16);
    fn up(&self);
    fn down(&self);
    fn left(&self);
    fn right(&self);
    fn moves(&self, direction: Direction);
    fn hide_cursor(&self);
    fn show_cursor(&self);
    fn set_style(&self, style: Style);
    fn set_styles(&self, fg: Color, bg: Color, fx: u32);
    fn unset_styles(&self);
    fn enable_mouse(&self);
    fn disable_mouse(&self);
    fn prints(&self, content: &str);
    // (imdaveho) NOTE: Just a bit of OS specific logic for pos.
    fn pos(&self) -> (i16, i16);
}