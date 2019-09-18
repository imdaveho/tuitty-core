// TODO: 

use crate::common::enums::{ Clear, Direction, Style, Color };

mod ansi;
#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod wincon;
#[cfg(windows)]
mod windows;

#[cfg(unix)]
pub use unix::{ 
    SyncReader, AsyncReader,
    UnixTerminal as Terminal
};
#[cfg(windows)]
pub use windows::{
    SyncReader, AsyncReader,
    WindowsConsole as Terminal
};


enum CommonTerminal {
    Ansi(ansi::AnsiTerminal),
    #[cfg(windows)]
    Win32(wincon::Win32Console)
}

trait CommonTerminalApi {
    fn resize(&self, w: i16, h: i16);
    fn goto(&self, col: i16, row: i16);
    fn up(&self, n: i16);
    fn down(&self, n: i16);
    fn left(&self, n: i16);
    fn right(&self, n: i16);
    fn set_style(&self, style: Style);
    fn set_styles(&self, fg: Color, bg: Color, fx: u32);
    fn reset_styles(&self);
    fn clear(&self, method: Clear);
    fn enable_mouse(&self);
    fn disable_mouse(&self);
    fn hide_cursor(&self);
    fn show_cursor(&self);
    fn pos(&self) -> (i16, i16);
}


trait SystemTerminalApi {
    fn init() -> Terminal; //
    fn terminate(&mut self);//
    fn raw(&mut self);
    fn cook(&mut self);
    fn read_char(&self) -> char;
    fn read_sync(&self) -> SyncReader;
    fn read_async(&self) -> AsyncReader;
    fn read_until_async(&self) -> AsyncReader;
    fn mark_pos(&mut self);
    fn load_pos(&mut self);
    fn screen_pos(&self) -> (i16, i16);
    fn screen_size(&self) -> (i16, i16);
    fn moves(&mut self, direction: Direction);    
    fn prints(&mut self, content: &str);
    // (imdaveho) NOTE: Wincon printf identical to prints.
    fn printf(&mut self, content: &str);
    // (imdaveho) NOTE: Wincon flush is no-op.
    fn flush(&mut self);
    fn switch(&mut self);
    fn to_main(&mut self);
    fn switch_to(&mut self, index: usize);
}