use super::enums::{ Style, Color, Effect };
use super::runtime::is_ansi_enabled;

use crate::ansi::cell::CellInfoCache;
#[cfg(windows)]
use crate::wincon::cell::CharInfoCache;


pub enum ScreenCache {
    Ansi(CellInfoCache),
    #[cfg(windows)]
    Win32(CharInfoCache),
}

impl CacheHandler for ScreenCache {
    pub fn new() -> ScreenCache {
        if is_ansi_enabled {
            ScreenCache::Ansi(CellInfoCache::new())
        }
        #[cfg(windows)]
        else { 
            ScreenCache::Win32(CharInfoCache::new())
        }
    }

    pub fn _screen_size(&self) -> (i16, i16) {
        match self {
            Ansi(a) => a._screen_size(),
            #[cfg(windows)]
            Win32(b) => b._screen_size(),
        }
    }

    pub fn _screen_pos(&self) -> (i16, i16) {
        match self {
            Ansi(a) => a._screen_pos(),
            #[cfg(windows)]
            Win32(b) => b._screen_pos(),
        }
    }

    pub fn _sync_size(&mut self, w: i16, h: i16) {
        match self {
            Ansi(a) => a._sync_size(w, h),
            #[cfg(windows)]
            Win32(b) => b._sync_size(w, h),
        }
    }
    
    pub fn _sync_pos(&mut self, col: i16, row: i16) {
        match self {
            Ansi(a) => a._sync_pos(col, row),
            #[cfg(windows)]
            Win32(b) => b._sync_pos(col, row),
        }
    }
    
    pub fn _sync_up(&mut self, n: i16) {
        match self {
            Ansi(a) => a._sync_up(n),
            #[cfg(windows)]
            Win32(b) => b._sync_up(n),
        }
    }
    
    pub fn _sync_dn(&mut self, n: i16) {
        match self {
            Ansi(a) => a._sync_dn(n),
            #[cfg(windows)]
            Win32(b) => b._sync_dn(n),
        }
    }
    
    pub fn _sync_left(&mut self, n: i16) {
        match self {
            Ansi(a) => a._sync_left(n),
            #[cfg(windows)]
            Win32(b) => b._sync_left(n),
        }
    }
    
    pub fn _sync_right(&mut self, n: i16) {
        match self {
            Ansi(a) => a._sync_right(n),
            #[cfg(windows)]
            Win32(b) => b._sync_right(n),
        }
    }
    
    pub fn _sync_style(&mut self, style: Style) {
        match self {
            Ansi(a) => a._sync_style(style),
            #[cfg(windows)]
            Win32(b) => b._sync_style(style),
        }
    }

    pub fn _sync_styles(&mut self, fg: Color, bg: Color, fx: u32) {
        match self {
            Ansi(a) => a._sync_styles(fg, bg, fx),
            #[cfg(windows)]
            Win32(b) => b._sync_styles(fg, bg, fx),
        }
    }

    pub fn _reset_style(&mut self) {
        match self {
            Ansi(a) => a._reset_style(),
            #[cfg(windows)]
            Win32(b) => b._reset_style(),
        }
    }
    
    pub fn _flush(&self) {
        match self {
            Ansi(a) => a._flush(),
            #[cfg(windows)]
            Win32(b) => b._flush(),
        }
    }

    // API specific helper methods outside scope of CacheHandler:

    // ANSI - requires updates to internal `CellInfo` buffer on clear.
    pub fn _clear(&self, method: Clear) {
        match self {
            Ansi(a) => a._clear(method),
            #[cfg(windows)]
            Win32(_) => (),
        }
    }

    // ANSI - used for each write to terminal.
    pub fn _sync_content(&self, content: &str) {
        match self {
            Ansi(a) => a._sync_content(content),
            #[cfg(windows)]
            Win32(b) => (),
        }
    }

    // Win32 - function to cache the entire screen into `CharInfo` buffer.
    #[cfg(windows)]
    pub fn _cache(&self) {
        match self {
            Ansi(_) => (),
            Win32(b) => b._cache(),
        }
    }
}


trait CacheHandler {
    fn new() -> ScreenCache;
    fn _screen_size(&self) -> (i16, i16);
    fn _screen_pos(&self) -> (i16, i16);
    fn _sync_size(&mut self, w: i16, h: i16);
    fn _sync_pos(&mut self, col: i16, row: i16);
    fn _sync_up(&mut self, n: i16);
    fn _sync_dn(&mut self, n: i16);
    fn _sync_left(&mut self, n: i16);
    fn _sync_right(&mut self, n: i16);
    fn _sync_style(&mut self, style: Style);
    fn _sync_styles(&mut self, fg: Color, bg: Color, fx: u32);
    fn _reset_styles(&mut self);
    fn _flush(&self);
    // NOTE: Wincon doesn't need to sync contents of each
    // write to an internal buffer.
    // fn _cache(&mut self)
    // NOTE: Wincon doesn't need to manage Clear.
    // fn _clear(&mut self, method: Clear);
}