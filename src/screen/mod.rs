//! # Screen
//!
//! The `screen` module represents the visible portion of the terminal and
//! contains standardized functions that control the viewport. This features
//! consistent functions that deal with sizing, clearing, and switching to an
//! temporary alternate screen.
//!
//! Note that the difference between the ANSI vs the WinCon functionalities is
//! that we moved the creation and activation of the alternate screen on Windows
//! to the top `wincon` module. This has to do with saving console settings to
//! restore back to normal once the application exits and cleans up.

use std::io::{Error, Result};

pub mod ansi;

#[cfg(windows)]
pub mod wincon;


/// Various styles of clearing the screen
pub enum Clear {
    /// clear all cells in terminal
    All,
    /// clear all cells from the cursor downwards
    CursorDn,
    /// clear all cells from the cursor upwards
    CursorUp,
    /// clear the current line
    CurrentLn,
    /// clear all cells from the cursor until a new line
    NewLn
}


// /// Unit tests
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test_sizing() {
//         use std::{thread, time};
//         use crate::screen::{size, resize};

//         let (w, h) = size();
//         let (new_w, new_h) = (50, 10);
//         resize(new_w, new_h).unwrap();
//         thread::sleep(time::Duration::from_millis(30));
//         let (test_w, test_h) = size();
//         assert_eq!(test_w, new_w);
//         assert_eq!(test_h, new_h);
//         resize(w, h).unwrap();
//     }
// }
