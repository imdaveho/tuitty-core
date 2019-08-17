//! This module represents the visible portion
//! of the TTY. Think of it like the application
//! shell or window -- basically like a viewport.
#[cfg(unix)]
mod linux;

#[cfg(unix)]
pub use linux::{
    _clear as clear,
    _size as size,
    _resize as resize,
    _disable_alt as disable_alt,
    _enable_alt as enable_alt,
};

#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use windows::{
    _clear as clear,
    _size as size,
    _resize as resize,
    _disable_alt as disable_alt,
};


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


// /// Clear the current screen by specifying a `ClearStyle`.
// ///
// /// # Doc Test / Example Usage
// /// ```rust
// /// use tuitty::screen::{self, ClearStyle};
// ///
// /// assert_eq!(screen::clear(ClearStyle::All).unwrap(), ());
// /// assert_eq!(screen::clear(ClearStyle::CursorUp).unwrap(), ());
// /// assert_eq!(screen::clear(ClearStyle::CursorDown).unwrap(), ());
// /// assert_eq!(screen::clear(ClearStyle::CurrentLine).unwrap(), ());
// /// assert_eq!(screen::clear(ClearStyle::NewLine).unwrap(), ());
// /// ```
// pub fn clear(clr: Clear) -> TtyResult<()> {
//     #[cfg(unix)] {
//         linux::_clear(clr)
//     }

//     #[cfg(windows)] {
//         windows::_clear(clr)
//     }
// }

//     /// Get the size of the terminal screen.
// pub fn size() -> (i16, i16) {
//     #[cfg(unix)] {
//         linux::_size()
//     }

//     #[cfg(windows)] {
//         windows::_size()
//     }
// }

// /// Resize the terminal screen.
// pub fn resize(w: i16, h: i16) -> TtyResult<()> {
//     #[cfg(unix)] {
//         linux::_resize(w, h)
//     }

//     #[cfg(windows)] {
//         windows::_resize(w, h)
//     }
// }

// // /// Scroll `n` lines up the current terminal screen.
// // pub fn scroll_up(n: i16) -> TtyResult<()> {
// //     #[cfg(unix)]
// //     linux::_scroll_up(n)
// // }

// // /// Scroll `n` lines down the current terminal screen.
// // pub fn scroll_dn(n: i16) -> TtyResult<()> {
// //     #[cfg(unix)]
// //     linux::_scroll_dn(n)
// // }

// /// Switch to the Alternative terminal screen.
// #[cfg(unix)]
// pub fn enable_alt() -> TtyResult<()> {
//     linux::_enable_alt()
// }

// /// Switch back to the Main terminal screen.
// pub fn disable_alt() -> TtyResult<()> {
//     #[cfg(unix)] {
//         linux::_disable_alt()
//     }

//     #[cfg(windows)] {
//         windows::_disable_alt()
//     }
// }

/// Unit tests
#[cfg(test)]
mod tests {
    #[test]
    fn test_sizing() {
        use std::{thread, time};
        use crate::screen::{size, resize};

        let (w, h) = size();
        let (new_w, new_h) = (50, 10);
        resize(new_w, new_h).unwrap();
        thread::sleep(time::Duration::from_millis(30));
        let (test_w, test_h) = size();
        assert_eq!(test_w, new_w);
        assert_eq!(test_h, new_h);
        resize(w, h).unwrap();
    }
}
