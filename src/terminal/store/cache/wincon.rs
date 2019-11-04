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
        let (w, h) = crate::terminal::actions::wincon::windows::size();
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
