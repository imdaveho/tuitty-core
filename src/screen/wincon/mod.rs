// Windows Console API functions for terminal size and clearing the screen.

use std::io::ErrorKind;
use winapi::um::wincon::{
    GetLargestConsoleWindowSize, COORD, SMALL_RECT,
    SetConsoleScreenBufferSize, SetConsoleWindowInfo,
    FillConsoleOutputCharacterA, FillConsoleOutputAttribute,
    COMMON_LVB_LEADING_BYTE as LB, COMMON_LVB_TRAILING_BYTE as TB,
    COMMON_LVB_GRID_HORIZONTAL as TH, COMMON_LVB_GRID_LVERTICAL as LV,
    COMMON_LVB_GRID_RVERTICAL as RV, COMMON_LVB_REVERSE_VIDEO as REV,
    COMMON_LVB_UNDERSCORE as UN,
};
use crate::shared::{Handle, ConsoleInfo};
use super::{Clear, Error, Result};

mod alternate;
pub use alternate::*;


pub fn clear(clr: Clear) -> Result<()> {
    let handle = Handle::conout()?;
    let info = ConsoleInfo::of(&handle)?;

    let pos = info.cursor_pos();
    let bsize = info.buffer_size();
    // Because the current Handle could have attributes that modify the
    // surrounding console cell (eg. underscore or left vertical), we
    // strip them out of the resulting attribute before clearing the
    // screen. If you do not do this, artifacts may result instead of
    // the intended blank screen. This maintains fg and bg colors.
    let curr_at = info.attributes() & !(LB | TB | TH | LV | RV | REV | UN);

    return match clr {
        Clear::All => {
            // get sum cells before cursor
            let cells_to_write = bsize.0 as u32 * bsize.1 as u32;
            // location where to start clearing
            let start_location = (0, 0);

            // clear the entire screen
            let conout = Handle::conout()?;
            _fill_chars(&conout, start_location, cells_to_write, ' ')?;
            _fill_attrs(&conout, start_location, cells_to_write, curr_at)?;

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
            _fill_attrs(&conout, start_location, cells_to_write, curr_at)?;

            Ok(())
        }
        Clear::CursorUp => {
            let (x, y) = (pos.0, pos.1);
            let cells_to_write = (bsize.0 as u32 * y as u32) + (x as u32 + 1);
            let start_location = (0, 0);

            let conout = Handle::conout()?;
            _fill_chars(&conout, start_location, cells_to_write, ' ')?;
            _fill_attrs(&conout, start_location, cells_to_write, curr_at)?;

            Ok(())
        }
        Clear::CurrentLn => {
            let cells_to_write = bsize.0 as u32;
            let start_location = (0, pos.1);

            let conout = Handle::conout()?;
            _fill_chars(&conout, start_location, cells_to_write, ' ')?;
            _fill_attrs(&conout, start_location, cells_to_write, curr_at)?;

            Ok(())
        }
        Clear::NewLn => {
            let cells_to_write = (bsize.0 - pos.1 as i16) as u32;
            let start_location = (pos.0, pos.1);

            let conout = Handle::conout()?;
            _fill_chars(&conout, start_location, cells_to_write, ' ')?;
            _fill_attrs(&conout, start_location, cells_to_write, curr_at)?;

            Ok(())
        }
    };
}

pub fn size() -> (i16, i16) {
    if let Ok(handle) = Handle::conout() {
        if let Ok(info) = ConsoleInfo::of(&handle) {
            let size = info.terminal_size();
            ((size.0 + 1), (size.1 + 1))
        } else {
            (0, 0)
        }
    } else {
        (0, 0)
    }
}

pub fn resize(w: i16, h: i16) -> Result<()> {
    if w <= 0 {
        return Err(Error::new(
            ErrorKind::Other,
            "Cannot set the terminal width lower than 1"));
    }

    if h <= 0 {
        return Err(Error::new(
            ErrorKind::Other,
            "Cannot set the terminal height lower then 1"));
    }

    let handle = Handle::conout()?;
    let info = ConsoleInfo::of(&handle)?;

    let (buf_w, buf_h) = info.buffer_size();
    let (left, _, _, top) = info.window_pos();

    let (mut new_w, mut new_h) = (buf_w, buf_h);

    // If the buffer is smaller than this new window size, resize the
    // buffer to be large enough.  Include window position.
    let mut resize_buffer = false;

    if buf_w < left + w {
        if left >= i16::max_value() - w {
            return Err(Error::new(
                ErrorKind::Other,
                "Argument out of range when setting terminal width."));
        }

        new_w = left + w;
        resize_buffer = true;
    }
    if buf_h < top + h {
        if top >= i16::max_value() - h {
            return Err(Error::new(
                ErrorKind::Other,
                "Argument out of range when setting terminal height."));
        }

        new_h = top + h;
        resize_buffer = true;
    }

    let resize_error = Error::new(
        ErrorKind::Other, 
        "Something went wrong when setting screen buffer size.");

    if resize_buffer {
        if let Err(_) = _set_size(&handle, new_w, new_h) {
            return Err(resize_error);
        }
    }

    unsafe {
        if SetConsoleWindowInfo(handle.0, 1, &SMALL_RECT {
            Left: left,
            Right: left + w,
            Bottom: top + h,
            Top: top
        }) == 0 {
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

    if w > bound_w {
        return Err(Error::new(
            ErrorKind::Other, format!(
                "Argument w: {} out of range setting terminal width.", w)));
    }

    if h > bound_h {
        return Err(Error::new(
            ErrorKind::Other, format!(
                "Argument h: {} out of range setting terminal height.", h)));
    }

    Ok(())
}


fn _fill_chars(h: &Handle, s: (i16, i16), n: u32, c: char) -> Result<u32> {
    let mut written = 0;
    let coord = COORD { X: s.0, Y: s.1, };
    unsafe {
        if FillConsoleOutputCharacterA(
            h.0, c as i8, n, coord, &mut written) == 0 {
            return Err(Error::last_os_error());
        }
    }
    Ok(written)
}

fn _fill_attrs(h: &Handle, s: (i16, i16), n: u32, a: u16) -> Result<u32> {
    let mut written = 0;
    let coord = COORD { X: s.0, Y: s.1, };
    unsafe {
        if FillConsoleOutputAttribute(
            h.0, a, n, coord, &mut written) == 0 {
            return Err(Error::last_os_error());
        }
    }
    Ok(written)
}

fn _set_size(h: &Handle, x: i16, y: i16) -> Result<()> {
    unsafe {
        if SetConsoleScreenBufferSize(h.0, COORD {X: x, Y: y}, ) == 0 {
            return Err(Error::last_os_error());
        }
    }
    Ok(())
}


// NOTE: Below commented out to keep parity with ANSI portion of the library.
// pub fn scroll_up(n: i16) -> TtyResult<()> {
//     let handle = Handle::conout()?;
//     let info = ConsoleInfo::of(&handle)?;

//     let (left, right, bottom, top) = info.window_pos();

//     if top >= n {
//         let stdout = Handle::stdout()?;
//         unsafe {
//             if SetConsoleWindowInfo(stdout.0, 0, &SMALL_RECT {
//                 Left: left,
//                 Right: right,
//                 Bottom: n,
//                 Top: top - n,
//             }) == 0 {
//                 return Err(TtyErrorKind::ResizingError(String::from(
//                     "Something went wrong when scrolling up the console."
//                 )));
//             }
//         }
//     }
//     Ok(())
// }

// pub fn scroll_dn(n: i16) -> TtyResult<()> {
//     let handle = Handle::conout()?;
//     let info = ConsoleInfo::of(&handle)?;

//     let (left, right, bottom, top) = info.window_pos();
//     let (buf_w, buf_h) = info.buffer_size();

//     if bottom < buf_h - n {
//         let stdout = Handle::stdout()?;
//         unsafe {
//             if SetConsoleWindowInfo(stdout.0, 0, &SMALL_RECT {
//                 Left: left,
//                 Right: right,
//                 Bottom: bottom + n,
//                 Top: top + n,
//             }) == 0 {
//                 return Err(TtyErrorKind::ResizingError(String::from(
//                     "Something went wrong when scrolling down the console."
//                 )));
//             }
//         }
//     }
//     Ok(())
// }
