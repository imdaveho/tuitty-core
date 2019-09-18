use super::enums::Style;
use super::runtime::is_ansi_enabled;

use crate::ansi::cell::CellInfoCache;
#[cfg(windows)]
use crate::wincon::cell::CharInfoCache;


pub enum ScreenCache {
    Ansi(CellInfoCache),
    #[cfg(windows)]
    Win32(CharInfoCache),
}

impl ScreenCache {
    pub fn new() -> ScreenCache {
        if is_ansi_enabled {
            ScreenCache::Ansi(CellInfoCache::new())
        }
        #[cfg(windows)]
        else { 
            ScreenCache::Win32(CharInfoCache::new())
        }
    }

    pub fn ansi(&self) -> &mut CellInfoCache {
        match self {
            Ansi(b) => b,
            #[cfg(windows)]
            Win32(_) => panic!("ScreenCache is not Ansi"),
        }
    }

    #[cfg(windows)]
    pub fn win32(&self) -> &mut CharInfoCache {
        match self {
            Ansi(_) => panic!("ScreenCache is not Win32"),
            Win32(b) => b,
        }
    }
}


trait CacheHandler {
    fn new() -> ScreenCache;
    fn _screen_size(&self) -> (i16, i16);
    fn _screen_pos(&self) -> (i16, i16);
    // NOTE: Wincon doesn't need to manage Clear.
    // fn _clear(&mut self, method: Clear);
    fn _clear_style(&mut self);
    fn _sync_size(&mut self, w: i16, h: i16);
    fn _sync_pos(&mut self, col: i16, row: i16);
    fn _sync_up(&mut self, n: i16);
    fn _sync_dn(&mut self, n: i16);
    fn _sync_left(&mut self, n: i16);
    fn _sync_right(&mut self, n: i16);
    fn _sync_style(&mut self, style: Style);
    fn _flush(&self);
    // NOTE: Wincon doesn't need to sync contents of each
    // write to an internal buffer.
    // fn _cache(&mut self)
}