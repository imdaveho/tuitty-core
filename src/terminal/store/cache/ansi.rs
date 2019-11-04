// ANSI specific screen cache buffer implmentation.

use std::{
    // iter, mem,
    collections::VecDeque,
};

// use crate::terminal::actions::ansi::output;
// use crate::terminal::actions::ansi::style;
use crate::common::{
    // unicode::{grapheme::*, wcwidth::*},
    // enums::{ Clear, Color, Effect, Style },
    enums::{ Color, Effect }
};


#[derive(Clone)]
enum Rune {
    Single(char),
    Double(char),
    Compound(Vec<char>),
    Null,
}


#[derive(Clone)]
pub struct CellInfo {
    rune: Rune,
    width: usize,
    style: (Color, Color, u32),
}


#[derive(Clone)]
pub struct CellInfoCache {
    pub style: (Color, Color, u32),
    pub screen_pos: (i16, i16),
    pub screen_size: (i16, i16),
    pub buffer: VecDeque<Option<CellInfo>>,
}

impl CellInfoCache {
    pub fn new() -> CellInfoCache {
        let (w, h) = {
            #[cfg(unix)] {
                crate::terminal::actions::ansi::unix::size()
            }

            #[cfg(windows)] {
                crate::terminal::actions::wincon::windows::size()
            }
        };
        let capacity = (w * h) as usize;
        CellInfoCache {
            screen_pos: (0, 0),
            screen_size: (w, h),
            style: (Color::Reset, Color::Reset, Effect::Reset as u32),
            buffer: vec![None; capacity].into(),
        }
    }
}

//     fn flush_buffer(&self) {
//         let (w, h) = (
//             self.screen_size.0 as usize,
//             self.screen_size.1 as usize);
//         let capacity = w * h;
//         let default = (Color::Reset, Color::Reset, Effect::Reset as u32);
//         // TODO: stress test the content.len capacity here.
//         let mut contents = String::with_capacity(capacity * 3);
//         let mut previous = (Color::Reset, Color::Reset, Effect::Reset as u32);
//         // Reset everything from the previous screens once at the start.
//         contents.push_str(&style::reset());

//         for cell in &self.buffer {
//             match cell {
//                 Some(cel) => {
//                     // Restore styles.
//                     if cel.style != previous && cel.style == default {
//                         // Reset not just when the current style differs a bit
//                         // from the previous, but every field is different and
//                         // is a {Color|Effect}::Reset value.
//                         contents.push_str(&style::reset())
//                     } else {
//                         // Else, go through each and update them.
//                         if cel.style.0 != previous.0 {
//                             contents.push_str(
//                                 &style::set_style(Style::Fg(cel.style.0)))
//                         }

//                         if cel.style.1 != previous.1 {
//                             contents.push_str(
//                                 &style::set_style(Style::Bg(cel.style.1)))
//                         }

//                         if cel.style.2 != previous.2 {
//                             contents.push_str(
//                                 &style::set_style(Style::Fx(cel.style.2)))
//                         }
//                     }
//                     previous = cel.style;
//                     // Insert contents.
//                     match &cel.rune {
//                         Rune::Single(c) => match c {
//                             '\t' => for _ in 0..self.tab_width {
//                                 contents.push(' ')
//                             },
//                             // '\n' => (),
//                             _ => contents.push(*c),
//                         },
//                         Rune::Double(c) => contents.push(*c),
//                         Rune::Null => (),
//                         Rune::Compound(v) => {
//                             for c in v {
//                                 contents.push(*c)
//                             }
//                         }
//                     }
//                 },
//                 None => {
//                     if previous == default { contents.push(' '); }
//                     else {
//                         contents.push_str(&style::reset());
//                         contents.push(' ');
//                         previous = default;
//                     }
//                 }
//             }
//         }
//         output::printf(&contents);
//     }
// }


// trait FirstLastIterator: Iterator + Sized {
//     fn first_last(self) -> FirstLast<Self>;
// }

// impl<I> FirstLastIterator for I where I: Iterator {
//     fn first_last(self) -> FirstLast<Self> {
//         FirstLast(true, self.peekable())
//     }
// }

// pub struct FirstLast<I>(bool, iter::Peekable<I>) where I: Iterator;

// impl<I> Iterator for FirstLast<I> where I: Iterator {
//     type Item = (bool, bool, I::Item);

//     fn next(&mut self) -> Option<Self::Item> {
//         let first = mem::replace(&mut self.0, false);
//         self.1.next().map(|item| (first, self.1.peek().is_none(), item))
//     }
// }

#[cfg(test)]
mod tests {
    // #[test]
    // fn test_sync_content_ansii() {
    //     use super::CellInfoCache;

    //     let ascii = "AA";
    //     let mut cache = CellInfoCache::new();
    //     cache.sync_content(ascii);
    //     let mut cache_copy = vec![String::from("."); 2];
    //     for i in 0..2 {
    //         if let Some(info) = &cache.buffer[i] {
    //             match info.rune {
    //                 super::Rune::Single(c) => cache_copy[i] = c.to_string(),
    //                 super::Rune::Null => cache_copy[i] = String::from("_"),
    //                 _ => cache_copy[i] = String::from("Other"),
    //             }
    //         } else {
    //             cache_copy[i] = String::from("None");
    //         }
    //     }
    //     println!("{:?}", cache_copy);
    //     std::thread::sleep(std::time::Duration::from_millis(10000));
    // }

    // #[test]
    // fn test_sync_content_cjk() {
    //     use super::CellInfoCache;

    //     let cjk = "色A";
    //     let mut cache = CellInfoCache::new();
    //     cache.sync_content(cjk);
    //     let mut cache_copy = vec![String::from("."); 3];
    //     for i in 0..3 {
    //         if let Some(info) = &cache.buffer[i] {
    //             match info.rune {
    //                 super::Rune::Single(c) => cache_copy[i] = c.to_string(),
    //                 super::Rune::Double(c) => cache_copy[i] = c.to_string(),
    //                 super::Rune::Null => cache_copy[i] = String::from("_"),
    //                 _ => cache_copy[i] = String::from("Other"),
    //             }
    //         } else {
    //             cache_copy[i] = String::from("None");
    //         }
    //     }
    //     println!("{:?}", cache_copy);
    // }

    // #[test]
    // fn test_sync_content_compound() {
    //     use super::CellInfoCache;

    //     let compound = "👨‍👩‍👧A👨‍🚀A🤦‍♀️A";
    //     // let compound = "🤦‍♀️A";
    //     let mut cache = CellInfoCache::new();
    //     cache.sync_content(compound);
    //     let mut cache_copy = vec![String::from("."); 9];
    //     for i in 0..9 {
    //         if let Some(info) = &cache.buffer[i] {
    //             match &info.rune {
    //                 super::Rune::Single(c) => cache_copy[i] = c.to_string(),
    //                 super::Rune::Double(c) => cache_copy[i] = c.to_string(),
    //                 super::Rune::Compound(v) => {
    //                     let mut s = String::new();
    //                     for c in v {
    //                         s.push(*c);
    //                     }
    //                     cache_copy[i] = s;
    //                 },
    //                 super::Rune::Null => cache_copy[i] = String::from("_"),
    //             }
    //         } else {
    //             cache_copy[i] = String::from("None");
    //         }
    //     }
    //     println!("{:?}", cache_copy);
    // }
}
