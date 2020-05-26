extern crate tuitty;

fn main() {
    
}

// #[cfg(windows)]
// extern crate tuitty;

// #[cfg(windows)]
// use tuitty::common::unicode::{grapheme::*, wcwidth::*};

// #[cfg(windows)]
// extern crate winapi;

// // #[cfg(windows)]
// // use std::mem::zeroed;
// #[cfg(windows)]
// use std::thread;
// #[cfg(windows)]
// use std::time::Duration;
// #[cfg(windows)]
// use std::ffi::OsStr;
// #[cfg(windows)]
// use std::os::windows::ffi::OsStrExt;

// // #[cfg(windows)]
// // use winapi::um::wincon::{
// //     WriteConsoleOutputW,
// //     CHAR_INFO, COORD, SMALL_RECT
// // };
// #[cfg(windows)]
// use winapi::um::consoleapi::WriteConsoleW;
// #[cfg(windows)]
// use winapi::shared::ntdef::{ NULL, VOID };


// #[cfg(windows)]
// fn main() {
//     let handle = tuitty::terminal::actions::win32::Handle::stdout().unwrap();
//     let mode = handle.get_mode().unwrap();
//     let info = tuitty::terminal::actions::win32::ConsoleInfo::of(&handle).unwrap();
//     let attrs = info.attributes();

//     let altern = tuitty::terminal::actions::win32::Handle::buffer().unwrap();
//     tuitty::terminal::actions::win32::enable_alt(&altern, &mode, false);

//     let example_output = "qwertyuiopasd⚠️fghjkl😀園v明nmQWE👪RTY👨‍👩‍👧UIOPASDFGHJKLZXCVBNM";

//     tuitty::terminal::actions::win32::goto(0, 0, &altern, false);
//     // tuitty::terminal::actions::win32::printf(example_output, false);
//     // Size within sub-terminal (eg. windows terminal or cmder) is incorrect...
//     let size = info.terminal_size();
//     println!("X: {}, Y: {}", size.0, size.1);
//     let size = (size.0 + 1, size.1 + 1);

//     let mut screen_str = String::new();
//     let alpha = ["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z","0","1","2","3","4","5","6","7","8","9","0"];
//     for i in 0..size.1 {
//         screen_str.push_str(&format!("{}", alpha[i as usize]).repeat(size.0 as usize));
//     }
//     tuitty::terminal::actions::win32::printf(&screen_str, false);

//     thread::sleep(Duration::from_millis(2000));

//     // let size = (85, 29);

//     // let buf_size = COORD {X: size.0 , Y: size.1};
//     // let buf_coord = COORD {X: 0, Y: 0};
//     // let mut dest_rect = SMALL_RECT {
//     //     Top: 0,
//     //     Left: 0,
//     //     Bottom: size.1,
//     //     Right: size.0,
//     // };

//     // let mut winbuf: Vec<CHAR_INFO> = unsafe {
//     //     let mut defch: CHAR_INFO = zeroed();
//     //     defch.Attributes = attrs;
//     //     vec![defch; (size.0 * size.1) as usize * 4]
//     // };

//     let mut innerbuf: Vec<Option<Vec<u16>>> = vec![None; (size.0 * size.1) as usize];

//     let first_row = 0;
//     let first_col = size.0 as usize;
//     for i in first_row..first_col {
//         innerbuf[i] = Some(vec!['a' as u16]);
//     }

//     let last_row = (size.0 * (size.1 - 1)) as usize;
//     let last_col = last_row + size.0 as usize;
//     for i in last_row..last_col {
//         innerbuf[i] = Some(vec!['z' as u16]);
//     }

//     let index = size.0 as usize / 2;
//     innerbuf[index] = Some(OsStr::new("👪").encode_wide().collect());
//     innerbuf[index + 1] = Some(vec![]);

//     let mut output: Vec<u16> = vec![];
//     for v in &innerbuf {
//         match v {
//             Some(u) => { for i in u { output.push(*i) }},
//             None => output.push('-' as u16),
//         }
//     }

//     tuitty::terminal::actions::win32::goto(0, 0, &altern, false);
//     let str_output = String::from_utf16(&output).unwrap();
//     tuitty::terminal::actions::win32::printf(&str_output, false);

//     thread::sleep(Duration::from_millis(2000));

//     tuitty::terminal::actions::win32::goto(0, 0, &altern, false);
//     tuitty::terminal::actions::win32::printf(&format!("{}", "🔣".width()), false);
//     tuitty::terminal::actions::win32::printf("🔣", false);

//     thread::sleep(Duration::from_millis(2000));

//     let mut innerbuf: Vec<Option<Vec<u16>>> = vec![None; (size.0 * size.1) as usize];
    
//     // let original: Vec<u16> = OsStr::new(example_output).encode_wide().collect();
//     // let mut index = 0;
//     // for ch in original {
//     //     unsafe { *winbuf[index].Char.UnicodeChar_mut() = ch }
//     //     index += 1;
//     // }
//     let example_output = "qwertyuiopasd⚠️fghjkl😀園v明nmQWE👪RTY👨‍👩‍👧UIOPASDFGHJKLZXCVBNM";


//     let segments: Vec<&str> = UnicodeGraphemes::graphemes(example_output, true).collect();
//     let mut index = 0;
//     for s in segments {
//         match s.width() {
//             1 => {
//                 if s.contains("\u{fe0f}") {
//                     innerbuf[index] = Some(OsStr::new(s).encode_wide().collect());
//                     innerbuf[index  + 1] = Some(vec![]);
//                     index += 2;
//                 } else {
//                     innerbuf[index] = Some(OsStr::new(s).encode_wide().collect());
//                     index += 1;
//                 }
//             },
//             2 => {
//                 innerbuf[index] = Some(OsStr::new(s).encode_wide().collect());
//                 innerbuf[index  + 1] = Some(vec![]);
//                 index += 2;
//             },
//             _ => {
//                 innerbuf[index] = Some(OsStr::new("🔣").encode_wide().collect());
//                 innerbuf[index  + 1] = Some(vec![]);
//                 index += 2;
//             }
//         }
//     }

//     // let patch: Vec<u16> = OsStr::new("👪").encode_wide().collect();
//     // let row = 9 * size.0;
//     // let col = 12;
//     // let mut index = (row + col) as usize;
//     // for ch in patch {
//     //     unsafe { *winbuf[index].Char.UnicodeChar_mut() = ch }
//     //     index += 1;
//     // }
//     let patch: Vec<&str> = UnicodeGraphemes::graphemes("👪", true).collect();
//     let row = 9 * size.0; // 10th row
//     let col = 0;
//     let mut index = (row + col) as usize;
//     // let mut index = 0;
//     for s in patch {
//         match s.width() {
//             1 => {
//                 if s.contains("\u{fe0f}") {
//                     innerbuf[index] = Some(OsStr::new(s).encode_wide().collect());
//                     innerbuf[index  + 1] = Some(vec![]);
//                     index += 2;
//                 } else {
//                     innerbuf[index] = Some(OsStr::new(s).encode_wide().collect());
//                     index += 1;
//                 }
//             },
//             2 => {
//                 innerbuf[index] = Some(OsStr::new(s).encode_wide().collect());
//                 innerbuf[index  + 1] = Some(vec![]);
//                 index += 2;
//             },
//             _ => {
//                 innerbuf[index] = Some(OsStr::new("🔣").encode_wide().collect());
//                 innerbuf[index  + 1] = Some(vec![]);
//                 index += 2;
//             }
//         }
//     }

//     // let patch2: Vec<u16> = OsStr::new("--").encode_wide().collect();
//     // let row = 9 * size.0;
//     // let col = 15;
//     // let mut index = (row + col) as usize;
//     // for ch in patch2 {
//     //     unsafe { *winbuf[index].Char.UnicodeChar_mut() = ch }
//     //     index += 1;
//     // }
//     let patch: Vec<&str> = UnicodeGraphemes::graphemes("xx", true).collect();
//     let row = 9 * size.0;
//     let col = 3;
//     let mut index = (row + col) as usize;
//     for s in patch {
//         match s.width() {
//             1 => {
//                 if s.contains("\u{fe0f}") {
//                     innerbuf[index] = Some(OsStr::new(s).encode_wide().collect());
//                     innerbuf[index  + 1] = Some(vec![]);
//                     index += 2;
//                 } else {
//                     innerbuf[index] = Some(OsStr::new(s).encode_wide().collect());
//                     index += 1;
//                 }
//             },
//             2 => {
//                 innerbuf[index] = Some(OsStr::new(s).encode_wide().collect());
//                 innerbuf[index  + 1] = Some(vec![]);
//                 index += 2;
//             },
//             _ => {
//                 innerbuf[index] = Some(OsStr::new("🔣").encode_wide().collect());
//                 innerbuf[index  + 1] = Some(vec![]);
//                 index += 2;
//             }
//         }
//     }

//     // let copy: Vec<u16> = OsStr::new(example_output).encode_wide().collect();
//     // let mut index = 85 + 41;
//     // for ch in copy {
//     //     unsafe { *winbuf[index].Char.UnicodeChar_mut() = ch }
//     //     index += 1;
//     // }
//     let copy: Vec<&str> = UnicodeGraphemes::graphemes(example_output, true).collect();
//     let mut index = (size.0 as usize * 1) + 20; // second row
//     for s in copy {
//         match s.width() {
//             1 => {
//                 if s.contains("\u{fe0f}") {
//                     innerbuf[index] = Some(OsStr::new(s).encode_wide().collect());
//                     innerbuf[index  + 1] = Some(vec![]);
//                     index += 2;
//                 } else {
//                     innerbuf[index] = Some(OsStr::new(s).encode_wide().collect());
//                     index += 1;
//                 }
//             },
//             2 => {
//                 innerbuf[index] = Some(OsStr::new(s).encode_wide().collect());
//                 innerbuf[index  + 1] = Some(vec![]);
//                 index += 2;
//             },
//             _ => {
//                 innerbuf[index] = Some(OsStr::new("🔣").encode_wide().collect());
//                 innerbuf[index  + 1] = Some(vec![]);
//                 index += 2;
//             }
//         }
//     }

//     // let mut index = 0;
//     // for c in &innerbuf {
//     //     match c {
//     //         Some(s) => for ch in s {
//     //             unsafe { *winbuf[index].Char.UnicodeChar_mut() = *ch }
//     //             index += 1;
//     //         },
//     //         None => {
//     //             unsafe { *winbuf[index].Char.UnicodeChar_mut() = ' ' as u16 }
//     //             index += 1;
//     //         }
//     //     }
//     // }

//     // tuitty::terminal::actions::win32::goto(0, 0, false);
//     // tuitty::terminal::actions::win32::printf(example_output, false);

//     // //let index = (size.0 as usize * 5);
//     // let index = (size.0 as usize) * 2;
//     // let h: Vec<u16> = OsStr::new("h").encode_wide().collect();
//     // innerbuf[index] = Some(h);
//     // let e: Vec<u16> = OsStr::new("e").encode_wide().collect();
//     // innerbuf[index + 4] = Some(e);


//     // let write_err = unsafe {
//     //     WriteConsoleOutputW(
//     //         altern.0,
//     //         winbuf.as_ptr(), 
//     //         COORD {X: size.0, Y: size.1},
//     //         COORD {X: 0, Y: 0},
//     //         &mut SMALL_RECT {
//     //             Top: 0,
//     //             Left: 0,
//     //             Bottom: size.1,
//     //             Right: size.0,
//     //         })
//     // };

    

//     // use winapi::um::consoleapi::WriteConsoleW;
//     // use winapi::shared::ntdef::{ NULL, VOID };

//     let mut output: Vec<u16> = vec![];
//     for v in &innerbuf {
//         match v {
//             Some(u) => { for i in u { output.push(*i) }},
//             None => output.push(' ' as u16),
//         }
//     }

//     tuitty::terminal::actions::win32::goto(0, 0, &altern, false);

//     // let mut written_chars = 0;
//     // let write_err = unsafe {
//     //     WriteConsoleW(
//     //         altern.0,
//     //         output.as_ptr() as *const VOID,
//     //         (size.0 * size.1) as u32,
//     //         &mut written_chars, NULL
//     //     )
//     // };

//     // if write_err == 0 {
//     //     tuitty::terminal::actions::win32::cook();
//     //     tuitty::terminal::actions::win32::disable_alt(false);
//     //     panic!("Something went wrong copying buffer");
//     // }

//     let str_output = String::from_utf16(&output).unwrap();
//     tuitty::terminal::actions::win32::printf(&str_output, false);

//     thread::sleep(Duration::from_millis(2000));

//     tuitty::terminal::actions::win32::goto(size.0 - 1, size.1 - 1, &altern, false);
//     tuitty::terminal::actions::win32::printf(" ", false);

//     thread::sleep(Duration::from_millis(5000));

//     tuitty::terminal::actions::win32::cook();
//     tuitty::terminal::actions::win32::disable_alt(false);
// }