// TODO:

use crate::common::enums::{ Clear, Color };


pub trait CommandCursor {
    fn goto(&self, col: i16, row: i16);
    fn up(&self, n: i16);
    fn down(&self, n: i16);
    fn left(&self, n: i16);
    fn right(&self, n: i16);
    // (imdaveho) TODO: Convert into an InputEvent.
    // fn pos() -> (i16, i16);
}

pub trait CommandModifier {
    fn hide_cursor(&self);
    fn show_cursor(&self);
    fn enable_mouse(&self);
    fn disable_mouse(&self);
    fn enable_alt(&self);
    fn disable_alt(&self);
    fn raw(&self);
    fn cook(&self);
}

pub trait CommandFormatter {
    fn clear(&self, method: Clear);
    fn size(&self) -> (i16, i16);
    fn resize(&self, w: i16, h: i16);
    fn set_fg(&self, color: Color);
    fn set_bg(&self, color: Color);
    fn set_fx(&self, effects: u32);
    fn set_styles(&self, fg: Color, bg: Color, fx: u32);
    fn reset_styles(&self);
    // fn set_style(&self, style: Style);
}

pub trait CommandWriter {
    fn prints(&self, content: &str);
    fn flush(&self);
    fn printf(&self, content: &str);
}



// pub trait TerminalCursor {
//     fn goto(&mut self, col: i16, row: i16);
//     fn up(&mut self);
//     fn down(&mut self);
//     fn left(&mut self);
//     fn right(&mut self);
//     fn pos(&mut self) -> (i16, i16);
//     fn mark_pos(&mut self);
//     fn load_pos(&mut self);
//     fn moves(&mut self, direction: Direction);
// }

// pub trait TerminalModifier {
//     fn raw(&mut self);
//     fn cook(&mut self);
//     fn hide_cursor(&mut self);
//     fn show_cursor(&mut self);
//     fn enable_mouse(&mut self);
//     fn disable_mouse(&mut self);
// }

// pub trait TerminalFormatter {
//     fn clear(&mut self, method: Clear);
//     fn resize(&mut self, w: i16, h: i16);
//     fn set_fg(&mut self, color: Color);
//     fn set_bg(&mut self, color: Color);
//     fn set_fx(&mut self, effects: u32);
//     fn set_styles(&mut self, fg: Color, bg: Color, fx: u32);
//     fn reset_styles(&mut self);
//     fn screen_pos(&self) -> (i16, i16);
//     fn screen_size(&self) -> (i16, i16);
// }

// pub trait TerminalWriter {
//     // TODO: sync_pos after writing the content internally
//     fn prints(&mut self, content: &str);
//     fn flush(&mut self);
//     fn printf(&mut self, content: &str);
// }

// pub trait TerminalInput {
//     fn read_char() -> char;
//     fn read_sync() -> SyncReader;
//     fn read_async() -> AsyncReader;
//     fn read_until_async(delimiter: u8) -> AsyncReader;
// }

// pub trait TerminalSwitcher {
//     fn switch(&mut self);
//     fn to_main(&mut self);
//     fn switch_to(&mut self, index: usize);
// }
