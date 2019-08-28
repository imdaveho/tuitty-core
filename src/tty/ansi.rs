use crate::screen;
use crate::cursor;
use crate::output;
use crate::input;

use crate::{AsyncReader, SyncReader, Termios};
use super::{Tty, Metadata};


// pub fn clear(method: &str) {
//     match method {
//         "all" => {
//             screen::ansi::clear(screen::Clear::All).unwrap();
//             goto(0, 0);
//         }
//         "newln" => {
//             screen::ansi::clear(screen::Clear::NewLn).unwrap();
//         }
//         "currentln" => {
//             screen::ansi::clear(screen::Clear::CurrentLn).unwrap();
//         }
//         "cursorup" => {
//             screen::ansi::clear(screen::Clear::CursorUp).unwrap();
//         }
//         "cursordn" => {
//             screen::ansi::clear(screen::Clear::CursorDn).unwrap();
//         }
//         _ => ()
//     }
// }

// #[cfg(unix)]
// pub fn size() -> (i16, i16) {
//     screen::ansi::size()
// }

// pub fn resize(w: i16, h: i16) {
//     screen::ansi::resize(w, h).unwrap();
// }

// pub fn switch(tty: &mut Tty) {
//     // This function is used primarily to create
//     // a new "screen" by creating some Metadata
//     // that reflects any changes in the mode as
//     // with enabling raw input or mouse events.
//     // On Unix, we will need to reset the screen
//     // to disable raw mode and mouse events.
//     if tty.id == 0 {
//         // There is no point to switch
//         // if you're on another screen
//         // since Unix systems only have
//         // a single "alternate screen".
//         screen::ansi::enable_alt().unwrap();
//     }
//     // Create the new `Metadata` to describe the
//     // new screen.
//     let metas = &mut tty.meta;
//     let rstate = metas[tty.id].is_raw_enabled;
//     let mstate = metas[tty.id].is_mouse_enabled;
//     metas.push(Metadata{
//         is_raw_enabled: rstate,
//         is_mouse_enabled: mstate,
//     });
//     tty.id = tty.meta.len() - 1;
//     // Ensure that raw and mouse modes are disabled.
//     cook(tty);
//     input::ansi::disable_mouse_mode().unwrap();
// }

// pub fn main(tty: &mut Tty) {
//     if tty.id != 0 {
//         // This function only works if the
//         // User is not already on the main
//         // screen buffer.
//         let metas = &tty.meta;
//         let rstate = metas[0].is_raw_enabled;
//         let mstate = metas[0].is_mouse_enabled;
//         tty.id = 0;
//         screen::ansi::disable_alt().unwrap();

//         if rstate {
//             output::ansi::enable_raw().unwrap();
//         } else {
//             cook(tty);
//         }

//         if mstate {
//             input::ansi::enable_mouse_mode().unwrap();
//         } else {
//             input::ansi::disable_mouse_mode().unwrap();
//         }
//     }
// }

// pub fn switch_to(tty: &mut Tty, id: usize) {
//     // If the id and the current id are the same, well,
//     // there is nothing more to do, you're already on
//     // the active screen buffer.
//     if id != tty.id {
//         if id == 0 {
//             // Switch to the main screen.
//             main(tty);
//         } else {
//             // Restore the mode of the alternate
//             // screen that you're switching to.
//             let metas = &tty.meta;
//             let rstate = metas[id].is_raw_enabled;
//             let mstate = metas[id].is_mouse_enabled;
//             tty.id = id;
//             if rstate {
//                 output::ansi::enable_raw().unwrap();
//             } else {
//                 cook(tty);
//             }

//             if mstate {
//                 input::ansi::enable_mouse_mode().unwrap();
//             } else {
//                 input::ansi::disable_mouse_mode().unwrap();
//             }
//         }
//     }
//     // NOTE: this only switches the screen buffer and updates
//     // the settings. Updating the content that will be passed
//     // in and rendered, that is up to the implementation.
// }

// #[cfg(unix)]
// pub fn raw(tty: &mut Tty) {
//     let mut m = &mut tty.meta[tty.id];
//     output::ansi::enable_raw().unwrap();
//     m.is_raw_enabled = true;
// }

// #[cfg(unix)]
// pub fn cook(tty: &mut Tty) {
//     // "cooked" vs "raw" mode terminology from Wikipedia:
//     // https://en.wikipedia.org/wiki/Terminal_mode
//     // A terminal mode is one of a set of possible states of a
//     // terminal or pseudo terminal character device in Unix-like
//     // systems and determines how characters written to the terminal
//     // are interpreted. In cooked mode data is preprocessed before
//     // being given to a program, while raw mode passes the data as-is
//     // to the program without interpreting any of the special characters.
    // let mut m = &mut tty.meta[tty.id];
    // output::ansi::set_mode(&tty.original_mode).unwrap();
    // m.is_raw_enabled = false;
// }

// #[cfg(unix)]
// pub fn enable_mouse(tty: &mut Tty) {
//     let mut m = &mut tty.meta[tty.id];
//     input::ansi::enable_mouse_mode().unwrap();
//     m.is_mouse_enabled = true;
// }

// #[cfg(unix)]
// pub fn disable_mouse(tty: &mut Tty) {
//     let mut m = &mut tty.meta[tty.id];
//     input::ansi::disable_mouse_mode().unwrap();
//     m.is_mouse_enabled = false;
// }

// pub fn goto(col: i16, row: i16) {
//     cursor::ansi::goto(col, row).unwrap();
// }

// pub fn up() {
//     cursor::ansi::move_up(1).unwrap();
// }

// pub fn dn() {
//     cursor::ansi::move_down(1).unwrap();
// }

// pub fn left() {
//     cursor::ansi::move_left(1).unwrap();
// }

// pub fn right() {
//     cursor::ansi::move_right(1).unwrap();
// }

// pub fn dpad(dir: &str, n: i16) {
//     // Case-insensitive.
//     let d = dir.to_lowercase();
//     if n > 0 {
//         match d.as_str() {
//             "up" => cursor::ansi::move_up(n).unwrap(),
//             "dn" => cursor::ansi::move_down(n).unwrap(),
//             "left" => cursor::ansi::move_left(n).unwrap(),
//             "right" => cursor::ansi::move_right(n).unwrap(),
//             _ => ()
//         }
//     }
// }

// pub fn pos(tty: &mut Tty) -> (i16, i16) {
//     if tty.meta[tty.id].is_raw_enabled {
//         cursor::ansi::pos_raw().unwrap()
//     } else {
//         // Unix needs to be raw to use pos().
//         raw(tty);
//         let (col, row) = cursor::ansi::pos_raw().unwrap();
//         // Since the output was not in raw_mode before
//         // we need to revert back to the cooked state.
//         cook(tty);
//         return (col, row);
//     }
// }

// pub fn mark() {
//     cursor::ansi::save_pos().unwrap()
// }

// pub fn load() {
//     cursor::ansi::load_pos().unwrap()
// }

// pub fn hide_cursor() {
//     cursor::ansi::hide().unwrap();
// }

// pub fn show_cursor() {
//     cursor::ansi::show().unwrap();
// }

// pub fn read_char() -> char {
//     input::ansi::read_char().unwrap()
// }

// pub fn read_sync() -> SyncReader {
//     input::ansi::read_sync()
// }

// pub fn read_async() -> AsyncReader {
//     input::ansi::read_async()
// }

// pub fn read_until_async(delimiter: u8) -> AsyncReader {
//     input::ansi::read_until_async(delimiter)
// }

// pub fn set_fg(col: &str) {
//     let fg_col = output::Color::from(col);
//     output::ansi::set_fg(fg_col).unwrap();
// }

// pub fn set_bg(col: &str) {
//     let bg_col = output::Color::from(col);
//     output::ansi::set_bg(bg_col).unwrap();
// }

// pub fn set_tx(tx: &str) {
//     let tx = output::TextStyle::from(tx);
//     output::ansi::set_tx(tx).unwrap();
// }

// pub fn set_fg_rgb(r: u8, g: u8, b: u8) {
//     let fg_col = output::Color::Rgb{
//         r: r,
//         g: g,
//         b: b,
//     };
//     output::ansi::set_fg(fg_col).unwrap();
// }

// pub fn set_bg_rgb(r: u8, g: u8, b: u8) {
//     let bg_col = output::Color::Rgb{
//         r: r,
//         g: g,
//         b: b,
//     };
//     output::ansi::set_bg(bg_col).unwrap();
// }

// pub fn set_fg_ansi(v: u8) {
//     let fg_col = output::Color::AnsiValue(v);
//     output::ansi::set_fg(fg_col).unwrap();
// }

// pub fn set_bg_ansi(v: u8) {
//     let bg_col = output::Color::AnsiValue(v);
//     output::ansi::set_bg(bg_col).unwrap();
// }

// pub fn set_style(fg: &str, bg: &str, tx: &str) {
//     // The params fg is a single word, bg is
//     // also a single word, however the tx
//     // param can be treated as a comma-separated
//     // list of words that match the various text
//     // styles that are supported: "bold", "dim",
//     // "underline", "reverse", "hide", and "reset".
//     output::ansi::set_all(fg, bg, tx).unwrap();
// }

// pub fn reset() {
//     output::ansi::reset().unwrap();
// }

// pub fn writeout(s: &str) {
//     output::ansi::writeout(s).unwrap();
// }
