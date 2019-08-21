//! This module represents the visible portion
//! of the TTY. Think of it like the application
//! shell or window -- basically like a viewport.
#[cfg(unix)]
use crate::{TtyResult};

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
