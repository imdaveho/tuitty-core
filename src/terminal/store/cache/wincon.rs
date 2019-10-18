// Windows Console API specific screen cache buffer implmentation.

use std::mem::zeroed;
use winapi::um::wincon::{
    // WriteConsoleOutputW, ReadConsoleOutputW,
    // COORD, CHAR_INFO, SMALL_RECT,
    CHAR_INFO
};

// use super::CacheUpdater;
// use crate::terminal::actions::wincon::Handle;
use crate::common::enums::{ Color, Effect};//, Style };


#[derive(Clone)]
pub struct CharInfoCache {
    pub screen_pos: (i16, i16),
    pub screen_size: (i16, i16),
    pub style: (Color, Color, u32),
    pub buffer: Vec<CHAR_INFO>,
}

impl CharInfoCache {
    pub fn new() -> CharInfoCache {
        let (w, h) = crate::terminal::actions::wincon::screen::size();
        let capacity = (w * h) as usize;
        CharInfoCache {
            screen_pos: (0, 0),
            screen_size: (w, h),
            style: (Color::Reset, Color::Reset, Effect::Reset as u32),
            buffer: unsafe {vec![zeroed(); capacity]},
        }
    }
}

    // pub fn sync_buffer(&mut self) {
    //     let (w, h) = self.screen_size;
    //     let conout = Handle::conout().unwrap();
    //     let dimens = COORD {X: w, Y: h};
    //     let origin = COORD {X: 0, Y: 0};
    //     let mut dest_rect = SMALL_RECT {
    //         Top: 0,
    //         Left: 0,
    //         Bottom: h,
    //         Right: w,
    //     };
    //     unsafe {
    //         // https://docs.microsoft.com/en-us/windows/console/writeconsole
    //         if ReadConsoleOutputW(
    //             conout.0,
    //             self.buffer.as_mut_ptr(),
    //             dimens, origin,
    //             &mut dest_rect
    //         ) == 0 {
    //             panic!("Error caching screen into buffer cache")
    //         }
    //     }
    // }
// }

// impl CacheUpdater for CharInfoCache {
//     fn tab_width(&self) -> u8 {
//         self.tab_width
//     }

//     fn screen_size(&self) -> (i16, i16) {
//         self.screen_size
//     }

//     fn screen_pos(&self) -> (i16, i16) {
//         self.screen_pos
//     }

//     fn sync_tab(&mut self, w: u8) {
//         self.tab_width = w;
//     }

//     fn sync_size(&mut self, w: i16, h: i16) {
//         self.screen_size = (w, h);
//         self.buffer.resize((w * h) as usize, unsafe { zeroed() });
//         // TODO: re-calc cursor position
//     }

//     fn sync_pos(&mut self, col: i16, row: i16) {
//         self.screen_pos = (col, row)
//     }

//     fn sync_up(&mut self, n: i16) {
//         let mut n = n;
//         if n < 0 { n = n.abs() }
//         let current_row = self.screen_pos.1;
//         if current_row - n > 0 {
//             self.screen_pos.1 -= n
//         } else { self.screen_pos.1 = 0 }
//     }

//     fn sync_down(&mut self, n: i16) {
//         let mut n = n;
//         if n < 0 { n = n.abs() }
//         let h = self.screen_size.1;
//         let current_row = self.screen_pos.1;
//         if current_row + n < h {
//             self.screen_pos.1 += n
//         } else { self.screen_pos.1 = h }
//     }

//     fn sync_left(&mut self, n: i16) {
//         let mut n = n;
//         if n < 0 { n = n.abs() }
//         let current_col = self.screen_pos.0;
//         if current_col - n > 0 {
//             self.screen_pos.0 -= n
//         } else {
//             // self.screen_pos.0 = 0
//             // (imdaveho) NOTE: Cursor wrapping draft.
//             // TODO: confirm behavior on Windows.
//             // TODO: n > capacity handling
//             let w = self.screen_size.0;
//             let rows = n / w;
//             let rest = n % w;
//             self.sync_up(rows);
//             if current_col - rest > 0 {
//                 self.screen_pos.0 -= rest
//             } else {
//                 self.screen_pos.0 = 0
//             }
//         }
//     }

//     fn sync_right(&mut self, n: i16) {
//         let mut n = n;
//         if n < 0 { n = n.abs() }
//         let w = self.screen_size.0;
//         let current_col = self.screen_pos.0;
//         if current_col + n < w {
//             self.screen_pos.0 += n
//         } else {
//             // self.screen_pos.0 = w;
//             // (imdaveho) NOTE: Cursor wrapping draft.
//             // TODO: confirm behavior on Windows.
//             // TODO: n > capacity handling
//             let rows = n / w;
//             let rest = n % w;
//             self.sync_down(rows);
//             if current_col + rest < w {
//                 self.screen_pos.0 += rest
//             } else {
//                 self.screen_pos.0 = w
//             }
//         }
//     }

//     fn sync_style(&mut self, style: Style) {
//         match style {
//             Style::Fg(c) => self.style.0 = c,
//             Style::Bg(c) => self.style.1 = c,
//             Style::Fx(f) => self.style.2 = f,
//         }
//     }

//     fn sync_styles(&mut self, fg: Color, bg: Color, fx: u32) {
//         self.style = (fg, bg, fx)
//     }

//     fn reset_styles(&mut self) {
//         self.style = (Color::Reset, Color::Reset, Effect::Reset as u32);
//     }

//     fn flush_buffer(&self) {
//         let (w, h) = self.screen_size;
//         let conout = Handle::conout().unwrap();
//         let dimens = COORD {X: w, Y: h};
//         let origin = COORD {X: 0, Y: 0};
//         let mut dest_rect = SMALL_RECT {
//             Top: 0,
//             Left: 0,
//             Bottom: h,
//             Right: w,
//         };
//         unsafe {
//             // https://docs.microsoft.com/en-us/windows/console/writeconsole
//             if WriteConsoleOutputW(
//                 conout.0,
//                 self.buffer.as_ptr(),
//                 dimens, origin,
//                 &mut dest_rect
//             ) == 0 {
//                 panic!("Error restoring screen from buffer cache")
//             }
//         }
//     }
// }
