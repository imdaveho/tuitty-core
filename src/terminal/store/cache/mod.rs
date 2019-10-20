// This module store details about the current screen. It allows for the 
// fetching of cursor position, screen size, characters under the cursor, and
// the restoration of content on each terminal screen that gets created. 

mod ansi;

#[cfg(windows)]
mod wincon;

// use crate::common::enums::{ Style, Color, Clear };
// use crate::common::enums::Clear;
use crate::terminal::actions::is_ansi_enabled;


// (imdaveho) TODO: `\t` handling for ANSI
// A tabstop is the column till which the terminal "inserts spaces"
// termbox-go implementation (notice the modulo):
// func rune_advance_len(r rune, pos int) int {
// 	if r == '\t' {
// 		return tabstop_length - pos%tabstop_length
// 	}
// 	return runewidth.RuneWidth(r)
// }

#[derive(Clone)]
pub enum ScreenCache {
    Ansi(ansi::CellInfoCache),
    #[cfg(windows)]
    Win32(wincon::CharInfoCache),
}

impl ScreenCache {
    pub fn new() -> ScreenCache {
        #[cfg(windows)] {
            if !is_ansi_enabled() {
                return ScreenCache::Win32(wincon::CharInfoCache::new());
            }
        }
        return ScreenCache::Ansi(ansi::CellInfoCache::new());
    }

    // // Win32 Only - function to cache the entire screen into `CharInfo` buffer.
    // #[cfg(windows)]
    // pub fn sync_buffer(&mut self) {
    //     match self {
    //         ScreenCache::Ansi(_) => (),
    //         ScreenCache::Win32(b) => b.sync_buffer(),
    //     }
    // }

    // // ANSI Only - requires updates to internal `CellInfo` buffer on clear.
    // pub fn clear_buffer(&mut self, method: Clear) {
    //     match self {
    //         ScreenCache::Ansi(a) => a.clear_buffer(method),
    //         #[cfg(windows)]
    //         ScreenCache::Win32(_) => (),
    //     }
    // }

    // // ANSI Only - used for each write to terminal.
    // pub fn sync_content(&mut self, content: &str) {
    //     match self {
    //         ScreenCache::Ansi(a) => a.sync_content(content),
    //         #[cfg(windows)]
    //         ScreenCache::Win32(_) => (),
    //     }
    // }
}

// pub trait CacheUpdater {
//     fn tab_width(&self) -> u8;
//     fn screen_size(&self) -> (i16, i16);
//     fn screen_pos(&self) -> (i16, i16);
    
//     fn sync_tab(&mut self, w: u8);
//     fn sync_size(&mut self, w: i16, h: i16);
//     fn sync_pos(&mut self, col: i16, row: i16);
//     fn sync_up(&mut self, n: i16);
//     fn sync_down(&mut self, n: i16);
//     fn sync_left(&mut self, n: i16);
//     fn sync_right(&mut self, n: i16);
//     fn sync_style(&mut self, style: Style);
//     fn sync_styles(&mut self, fg: Color, bg: Color, fx: u32);
    
//     fn reset_styles(&mut self);
//     fn flush_buffer(&self);
// }

// impl CacheUpdater for ScreenCache {
//     fn tab_width(&self) -> u8 {
//         match self {
//             ScreenCache::Ansi(a) => a.tab_width(),
//             #[cfg(windows)]
//             ScreenCache::Win32(b) => b.tab_width(),
//         }
//     }

//     fn screen_size(&self) -> (i16, i16) {
//         match self {
//             ScreenCache::Ansi(a) => a.screen_size(),
//             #[cfg(windows)]
//             ScreenCache::Win32(b) => b.screen_size(),
//         }
//     }

//     fn screen_pos(&self) -> (i16, i16) {
//         match self {
//             ScreenCache::Ansi(a) => a.screen_pos(),
//             #[cfg(windows)]
//             ScreenCache::Win32(b) => b.screen_pos(),
//         }
//     }

//     fn sync_tab(&mut self, w: u8) {
//         match self {
//             ScreenCache::Ansi(a) => a.sync_tab(w),
//             #[cfg(windows)]
//             ScreenCache::Win32(b) => b.sync_tab(w),
//         }
//     }

//     fn sync_size(&mut self, w: i16, h: i16) {
//         match self {
//             ScreenCache::Ansi(a) => a.sync_size(w, h),
//             #[cfg(windows)]
//             ScreenCache::Win32(b) => b.sync_size(w, h),
//         }
//     }

//     fn sync_pos(&mut self, col: i16, row: i16) {
//         match self {
//             ScreenCache::Ansi(a) => a.sync_pos(col, row),
//             #[cfg(windows)]
//             ScreenCache::Win32(b) => b.sync_pos(col, row),
//         }
//     }

//     fn sync_up(&mut self, n: i16) {
//         match self {
//             ScreenCache::Ansi(a) => a.sync_up(n),
//             #[cfg(windows)]
//             ScreenCache::Win32(b) => b.sync_up(n),
//         }
//     }

//     fn sync_down(&mut self, n: i16) {
//         match self {
//             ScreenCache::Ansi(a) => a.sync_down(n),
//             #[cfg(windows)]
//             ScreenCache::Win32(b) => b.sync_down(n),
//         }
//     }

//     fn sync_left(&mut self, n: i16) {
//         match self {
//             ScreenCache::Ansi(a) => a.sync_left(n),
//             #[cfg(windows)]
//             ScreenCache::Win32(b) => b.sync_left(n),
//         }
//     }

//     fn sync_right(&mut self, n: i16) {
//         match self {
//             ScreenCache::Ansi(a) => a.sync_right(n),
//             #[cfg(windows)]
//             ScreenCache::Win32(b) => b.sync_right(n),
//         }
//     }

//     fn sync_style(&mut self, style: Style) {
//         match self {
//             ScreenCache::Ansi(a) => a.sync_style(style),
//             #[cfg(windows)]
//             ScreenCache::Win32(b) => b.sync_style(style),
//         }
//     }

//     fn sync_styles(&mut self, fg: Color, bg: Color, fx: u32) {
//         match self {
//             ScreenCache::Ansi(a) => a.sync_styles(fg, bg, fx),
//             #[cfg(windows)]
//             ScreenCache::Win32(b) => b.sync_styles(fg, bg, fx),
//         }
//     }

//     fn reset_styles(&mut self) {
//         match self {
//             ScreenCache::Ansi(a) => a.reset_styles(),
//             #[cfg(windows)]
//             ScreenCache::Win32(b) => b.reset_styles(),
//         }
//     }

//     fn flush_buffer(&self) {
//         match self {
//             ScreenCache::Ansi(a) => a.flush_buffer(),
//             #[cfg(windows)]
//             ScreenCache::Win32(b) => b.flush_buffer(),
//         }
//     }
// }