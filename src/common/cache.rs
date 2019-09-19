use super::enums::{ Style, Color, Clear };
use super::runtime::is_ansi_enabled;

use crate::terminal::ansi::CellInfoCache;
#[cfg(windows)]
use crate::terminal::windows::CharInfoCache;


#[derive(Clone)]
pub enum ScreenCache {
    Ansi(CellInfoCache),
    #[cfg(windows)]
    Win32(CharInfoCache),
}

impl ScreenCache {
    pub fn new() -> ScreenCache {
        if is_ansi_enabled() {
            ScreenCache::Ansi(CellInfoCache::new())
        } else {
            #[cfg(windows)] 
            ScreenCache::Win32(CharInfoCache::new())
        }
    }

    // Win32 Only - function to cache the entire screen into `CharInfo` buffer.
    // See: TerminalSwitcher
    #[cfg(windows)]
    pub fn _cache_buffer(&mut self) {
        match self {
            ScreenCache::Ansi(_) => (),
            ScreenCache::Win32(b) => b._cache_buffer(),
        }
    }

    // ANSI Only - requires updates to internal `CellInfo` buffer on clear.
    pub fn _clear_buffer(&mut self, method: Clear) {
        match self {
            ScreenCache::Ansi(a) => a._clear_buffer(method),
            #[cfg(windows)]
            ScreenCache::Win32(_) => (),
        }
    }

    // ANSI Only - used for each write to terminal.
    // See TerminalWriter
    pub fn _cache_content(&mut self, content: &str) {
        match self {
            ScreenCache::Ansi(a) => a._cache_content(content),
            #[cfg(windows)]
            ScreenCache::Win32(b) => (),
        }
    }
}

pub trait CacheUpdater {
    fn _screen_size(&self) -> (i16, i16);
    fn _screen_pos(&self) -> (i16, i16);
    fn _sync_size(&mut self, w: i16, h: i16);
    fn _sync_pos(&mut self, col: i16, row: i16);
    fn _sync_up(&mut self, n: i16);
    fn _sync_down(&mut self, n: i16);
    fn _sync_left(&mut self, n: i16);
    fn _sync_right(&mut self, n: i16);
    fn _sync_style(&mut self, style: Style);
    fn _sync_styles(&mut self, fg: Color, bg: Color, fx: u32);
    fn _reset_styles(&mut self);
    fn _flush_buffer(&self);
    // NOTE: Wincon doesn't need to sync contents of each
    // write to an internal buffer.
    // fn _cache(&mut self)
    // NOTE: Wincon doesn't need to manage Clear.
    // fn _clear(&mut self, method: Clear);
}

impl CacheUpdater for ScreenCache {
    fn _screen_size(&self) -> (i16, i16) {
        match self {
            ScreenCache::Ansi(a) => a._screen_size(),
            #[cfg(windows)]
            ScreenCache::Win32(b) => b._screen_size(),
        }
    }

    fn _screen_pos(&self) -> (i16, i16) {
        match self {
            ScreenCache::Ansi(a) => a._screen_pos(),
            #[cfg(windows)]
            ScreenCache::Win32(b) => b._screen_pos(),
        }
    }

    fn _sync_size(&mut self, w: i16, h: i16) {
        match self {
            ScreenCache::Ansi(a) => a._sync_size(w, h),
            #[cfg(windows)]
            ScreenCache::Win32(b) => b._sync_size(w, h),
        }
    }
    
    fn _sync_pos(&mut self, col: i16, row: i16) {
        match self {
            ScreenCache::Ansi(a) => a._sync_pos(col, row),
            #[cfg(windows)]
            ScreenCache::Win32(b) => b._sync_pos(col, row),
        }
    }
    
    fn _sync_up(&mut self, n: i16) {
        match self {
            ScreenCache::Ansi(a) => a._sync_up(n),
            #[cfg(windows)]
            ScreenCache::Win32(b) => b._sync_up(n),
        }
    }
    
    fn _sync_down(&mut self, n: i16) {
        match self {
            ScreenCache::Ansi(a) => a._sync_down(n),
            #[cfg(windows)]
            ScreenCache::Win32(b) => b._sync_down(n),
        }
    }
    
    fn _sync_left(&mut self, n: i16) {
        match self {
            ScreenCache::Ansi(a) => a._sync_left(n),
            #[cfg(windows)]
            ScreenCache::Win32(b) => b._sync_left(n),
        }
    }
    
    fn _sync_right(&mut self, n: i16) {
        match self {
            ScreenCache::Ansi(a) => a._sync_right(n),
            #[cfg(windows)]
            ScreenCache::Win32(b) => b._sync_right(n),
        }
    }
    
    fn _sync_style(&mut self, style: Style) {
        match self {
            ScreenCache::Ansi(a) => a._sync_style(style),
            #[cfg(windows)]
            ScreenCache::Win32(b) => b._sync_style(style),
        }
    }

    fn _sync_styles(&mut self, fg: Color, bg: Color, fx: u32) {
        match self {
            ScreenCache::Ansi(a) => a._sync_styles(fg, bg, fx),
            #[cfg(windows)]
            ScreenCache::Win32(b) => b._sync_styles(fg, bg, fx),
        }
    }

    fn _reset_styles(&mut self) {
        match self {
            ScreenCache::Ansi(a) => a._reset_styles(),
            #[cfg(windows)]
            ScreenCache::Win32(b) => b._reset_styles(),
        }
    }
    
    fn _flush_buffer(&self) {
        match self {
            ScreenCache::Ansi(a) => a._flush_buffer(),
            #[cfg(windows)]
            ScreenCache::Win32(b) => b._flush_buffer(),
        }
    }
}


