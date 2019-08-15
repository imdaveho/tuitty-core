//! Platform specific functions for the library.
use super::{Clear, TtyResult};
use winapi::{
    um::{
        wincon::{
            SetConsoleScreenBufferSize, FillConsoleOutputCharacterA, FillConsoleOutputAttribute,
            SetConsoleWindowInfo, GetLargestConsoleWindowSize, COORD, SMALL_RECT,
        },
    },
};

use crate::shared::{Handle, ConsoleInfo, TtyErrorKind};
use std::io::{Error, Result};

mod alternate;
pub use alternate::*;


pub fn _clear(clr: Clear) -> TtyResult<()> {
    let handle = Handle::conout()?;
    let info = ConsoleInfo::of(&handle)?;

    let pos = info.cursor_pos();
    let bsize = info.buffer_size();
    let attrs = info.attributes();

    return match clr {
        Clear::All => {
            // get sum cells before cursor
            let cells_to_write = bsize.0 as u32 * bsize.1 as u32;
            // location where to start clearing
            let start_location = (0, 0);

            // clear the entire screen
            let conout = Handle::conout()?;
            _fill_chars(&conout, start_location, cells_to_write, ' ')?;
            _fill_attrs(&conout, start_location, cells_to_write, attrs)?;            

            Ok(())
        }
        Clear::CursorDn => {
            let (mut x, mut y) = (pos.0, pos.1);
            // if cursor position is at the outer right position
            if x as i16 > bsize.0 {
                y += 1;
                x = 0;
            }
            // get sum cells before cursor
            // let cells_to_write = bsize.0 as u32 * bsize.1 as u32;
            let cells_to_write = (bsize.0 as u32 - x as u32) + (
                bsize.0 as u32 * (bsize.1 as u32 - y as u32)
            );
            // location where to start clearing
            let start_location = (x, y);

            let conout = Handle::conout()?;
            _fill_chars(&conout, start_location, cells_to_write, ' ')?;
            _fill_attrs(&conout, start_location, cells_to_write, attrs)?;
            
            Ok(())
        }
        Clear::CursorUp => {
            let (x, y) = (pos.0, pos.1);
            let cells_to_write = (bsize.0 as u32 * y as u32) + (x as u32 + 1);
            let start_location = (0, 0);

            let conout = Handle::conout()?;
            _fill_chars(&conout, start_location, cells_to_write, ' ')?;
            _fill_attrs(&conout, start_location, cells_to_write, attrs)?;
            
            Ok(())
        }
        Clear::CurrentLn => {
            let cells_to_write = bsize.0 as u32;
            let start_location = (0, pos.1);

            let conout = Handle::conout()?;
            _fill_chars(&conout, start_location, cells_to_write, ' ')?;
            _fill_attrs(&conout, start_location, cells_to_write, attrs)?;

            Ok(())
        }
        Clear::NewLn => {
            let cells_to_write = (bsize.0 - pos.1 as i16) as u32;
            let start_location = (pos.0, pos.1);

            let conout = Handle::conout()?;
            _fill_chars(&conout, start_location, cells_to_write, ' ')?;
            _fill_attrs(&conout, start_location, cells_to_write, attrs)?;

            Ok(())
        }
    };
}

fn _fill_chars(h: &Handle, s: (i16, i16), n: u32, c: char) -> Result<u32> {
    let mut written = 0;
    let coord = COORD { X: s.0, Y: s.1, };
    unsafe {
        if !(FillConsoleOutputCharacterA(
            h.0, c as i8, n, coord, &mut written) == 0) {
            return Err(Error::last_os_error());
        }
    }
    Ok(written)
}

fn _fill_attrs(h: &Handle, s: (i16, i16), n: u32, a: u16) -> Result<u32> {
    let mut written = 0;
    let coord = COORD { X: s.0, Y: s.1, };
    unsafe {
        if !(FillConsoleOutputAttribute(
            h.0, a, n, coord, &mut written) == 0) {
            return Err(Error::last_os_error());
        }
    }
    Ok(written)
}

pub fn _size() -> (u16, u16) {
    if let Ok(handle) = Handle::conout() {
        if let Ok(info) = ConsoleInfo::of(&handle) {
            let size = info.terminal_size();
            ((size.0 + 1) as u16, (size.1 + 1) as u16)
        } else {
            (0, 0)
        }
    } else {
        (0, 0)
    }
}

pub fn _resize(w: u16, h: u16) -> TtyResult<()> {
    if w <= 0 {
        return Err(TtyErrorKind::ResizingError(String::from(
            "Cannot set the terminal width lower than 1",
        )));
    }

    if h <= 0 {
        return Err(TtyErrorKind::ResizingError(String::from(
            "Cannot set the terminal height lower then 1",
        )));
    }

    let handle = Handle::conout()?;
    let info = ConsoleInfo::of(&handle)?;

    let (buf_w, buf_h) = info.buffer_size();
    let (left, _, _, top) = info.window_pos();

    let (mut new_w, mut new_h) = (buf_w, buf_h);

    // If the buffer is smaller than this new window size, resize the
    // buffer to be large enough.  Include window position.
    let mut resize_buffer = false;

    if buf_w < left + w as i16 {
        if left >= i16::max_value() - w as i16 {
            return Err(TtyErrorKind::ResizingError(String::from(
                "Argument out of range when setting terminal width.",
            )));
        }

        new_w = left + w as i16;
        resize_buffer = true;
    }
    if buf_h < top + h as i16 {
        if top >= i16::max_value() - h as i16 {
            return Err(TtyErrorKind::ResizingError(String::from(
                "Argument out of range when setting terminal height.",
            )));
        }

        new_h = top + h as i16;
        resize_buffer = true;
    }

    let resize_error = TtyErrorKind::ResizingError(String::from(
        "Something went wrong when setting screen buffer size.",
    ));
    
    if resize_buffer {
        if let Err(_) = _set_size(&handle, new_w, new_h) {
            return Err(resize_error);
        }
    }

    unsafe {
        if !(SetConsoleWindowInfo(handle.0, 1, &SMALL_RECT {
            Left: left,
            Right: left + w as i16,  
            Bottom: top + h as i16, 
            Top: top
        }) == 0) {
            return Err(resize_error);
        }
    }

    // If we resized the buffer, un-resize it.
    if resize_buffer {
        if let Err(_) = _set_size(&handle, buf_w, buf_h) {
            return Err(resize_error);
        }
    }

    let (bound_w, bound_h) = unsafe {
        let bounds = GetLargestConsoleWindowSize(handle.0);
        (bounds.X, bounds.Y)
    };

    if w as i16 > bound_w {
        return Err(TtyErrorKind::ResizingError(format!(
            "Argument width: {} out of range when setting terminal width.", w
        )));
    }

    if h as i16 > bound_h {
        return Err(TtyErrorKind::ResizingError(format!(
            "Argument height: {} out of range when setting terminal height.", h
        )));
    }

    Ok(())
}

fn _set_size(h: &Handle, x: i16, y: i16) -> Result<()> {
    unsafe {
        if !(SetConsoleScreenBufferSize(h.0, COORD {X: x, Y: y}, ) == 0) {
            return Err(Error::last_os_error());
        }
    }
    Ok(())
}

// pub fn _scroll_up(n: i16) -> TtyResult<()> {
//     let handle = Handle::conout()?;
//     let info = ConsoleInfo::of(&handle)?;

//     let (left, right, bottom, top) = info.window_pos();

//     if top >= n {
//         let stdout = Handle::stdout()?;
//         unsafe {
//             if !(SetConsoleWindowInfo(stdout.0, 0, &SMALL_RECT {
//                 Left: left,
//                 Right: right,  
//                 Bottom: n, 
//                 Top: top - n,
//             }) == 0) {
//                 return Err(TtyErrorKind::ResizingError(String::from(
//                     "Something went wrong when scrolling up the console."
//                 )));
//             }
//         }
//     }
//     Ok(())
// }

// pub fn _scroll_dn(n: i16) -> TtyResult<()> {
//     let handle = Handle::conout()?;
//     let info = ConsoleInfo::of(&handle)?;

//     let (left, right, bottom, top) = info.window_pos();
//     let (buf_w, buf_h) = info.buffer_size();

//     if bottom < buf_h - n {
//         let stdout = Handle::stdout()?;
//         unsafe {
//             if !(SetConsoleWindowInfo(stdout.0, 0, &SMALL_RECT {
//                 Left: left,
//                 Right: right,  
//                 Bottom: bottom + n,
//                 Top: top + n,
//             }) == 0) {
//                 return Err(TtyErrorKind::ResizingError(String::from(
//                     "Something went wrong when scrolling down the console."
//                 )));
//             }
//         }
//     }
//     Ok(())
// }