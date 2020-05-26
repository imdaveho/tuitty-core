// Windows Console API functions for terminal size, clearing the screen, and
// handling of disabling the alternate screen. (Enabling is implemented at 
// the consolidated `windows` module where it all comes together).
//
// Typically you work on the main screen but there are cases where you may want
// to switch to an temporary alternate screen. The alternative screen on Windows
// is created by associating a new `Handle` with some kind of `File` with Read /
// Write traits.

use std::io::{Error, ErrorKind, Result};
use winapi::um::wincon::{
    GetLargestConsoleWindowSize, COORD, SMALL_RECT,
    SetConsoleScreenBufferSize, SetConsoleWindowInfo,
    FillConsoleOutputCharacterA, FillConsoleOutputAttribute,
};
use super::handle::{Handle, ConsoleInfo};
use crate::common::enums::Clear;


pub fn clear(clr: Clear) -> Result<()> {
    let handle = Handle::conout()?;
    let info = ConsoleInfo::of(&handle)?;

    let (mut col, mut row) = info.cursor_pos();
    let (w, h) = info.buffer_size();

    // Inputs to FillConsoleOutput.
    // let conout = Handle::conout()?;
    let ch = ' ' as i8;
    // Because the current Handle could have attributes that modify the
    // surrounding console cell (eg. underscore or left vertical), we
    // strip them out of the resulting attribute before clearing the
    // screen. If you do not do this, artifacts may result instead of
    // the intended blank screen. This maintains fg and bg colors.
    // let attribs = info.attributes() & !(LB | TB | TH | LV | RV | REV | UN);
    let fx = info.attributes() & !0xdf00;
    // Length of cells to write.
    let n: u32;
    // Starting location for clearing.
    let at: COORD;
    let mut len = 0;

    match clr {
        Clear::All => {
            n = (w * h) as u32;
            at = COORD {X: 0, Y: 0};
        }
        Clear::CursorDn => {
            // Cursor is wrapped.
            if col > w { row += 1; col = 0; }
            n = ((w - col) + (w * (h - row))) as u32;
            at = COORD {X: col, Y: row};
        }
        Clear::CursorUp => {
            n = ((w * row) + (col + 1)) as u32;
            at = COORD {X: 0, Y: 0};
        }
        Clear::CurrentLn => {
            n = w as u32;
            at = COORD {X: 0, Y: row};
        }
        Clear::NewLn => {
            n = (w - col) as u32;
            at = COORD {X: col, Y: row};
        }
    }

    unsafe {
        if FillConsoleOutputCharacterA(handle.0, ch, n, at, &mut len) == 0 {
            return Err(Error::last_os_error())
        }

        if FillConsoleOutputAttribute(handle.0, fx, n, at, &mut len) == 0 {
            return Err(Error::last_os_error())
        }
    }
    handle.close()?;
    Ok(())
}

pub fn size() -> Result<(i16, i16)> {
    let handle = Handle::conout()?;
    let info = ConsoleInfo::of(&handle)?;
    let size = info.terminal_size();
    handle.close()?;
    Ok((size.0 + 1, size.1 + 1))
    // let err_msg = "Error closing $CONOUT when getting console size";
    // if let Ok(handle) = Handle::conout() {
    //     if let Ok(info) = ConsoleInfo::of(&handle) {
    //         handle.close().expect(err_msg);
    //         let size = info.terminal_size();
    //         ((size.0 + 1), (size.1 + 1))
    //     } else {
    //         handle.close().expect(err_msg);
    //         (0, 0)
    //     }
    // } else {
    //     (0, 0)
    // }
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

    unsafe {
        if resize_buffer {
            let new_coord = COORD {X: new_w - 1, Y: new_h - 1};

            if SetConsoleScreenBufferSize(handle.0, new_coord) == 0 {
                return Err(resize_error)
            }
        }

        if SetConsoleWindowInfo(handle.0, 1, &SMALL_RECT {
            Left: left,
            Right: left + w - 1,
            Bottom: top + h - 1,
            Top: top
        }) == 0 {
            return Err(resize_error);
        }

        // If we resized the buffer, un-resize it.
        if resize_buffer {
            let buf_coord = COORD {X: buf_w - 1, Y: buf_h - 1};
            if SetConsoleScreenBufferSize(handle.0, buf_coord) == 0 {
                return Err(resize_error)
            }
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

    handle.close()?;
    Ok(())
}

pub fn disable_alt() -> Result<()> {
    let handle = Handle::stdout()?;
    handle.show()?;
    handle.close()?;
    Ok(())
}
