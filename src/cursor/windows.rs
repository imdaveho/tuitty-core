//! Platform specific functions for the library.
use std::io::{Error, ErrorKind, Result};
use crate::shared::{ConsoleInfo, Handle, TtyResult, TtyErrorKind};
use winapi::um::wincon::{
    SetConsoleCursorInfo, SetConsoleCursorPosition, COORD, 
    CONSOLE_CURSOR_INFO,
};


pub fn _goto(col: i16, row: i16) -> TtyResult<()> {
    if col < 0 || col >= <i16>::max_value() {
        return Err(TtyErrorKind::IoError(Error::new(
            ErrorKind::Other,
            format!(
                "Argument Out of Range Exception 
                when setting cursor position to X: {}", col
            ),
        )));
    }
    if row < 0 || row >= <i16>::max_value() {
        return Err(TtyErrorKind::IoError(Error::new(
            ErrorKind::Other,
            format!(
                "Argument Out of Range Exception 
                when setting cursor position to Y: {}", row
            ),
        )));
    }

    let pos = COORD { X: col, Y: row };
    let handle = Handle::conout()?;

    unsafe {
        if SetConsoleCursorPosition(handle.0, pos) == 0 {
            return Err(TtyErrorKind::IoError(Error::last_os_error()));
        }
    }

    Ok(())
}

pub fn _move_up(n: i16) -> TtyResult<()> {
    let (col, row) = _pos().unwrap();
    _goto(col, row - n)
}

pub fn _move_right(n: i16) -> TtyResult<()> {
    let (col, row) = _pos().unwrap();
    _goto(col + n, row)
}

pub fn _move_down(n: i16) -> TtyResult<()> {
    let (col, row) = _pos().unwrap();
    _goto(col, row + n)
}

pub fn _move_left(n: i16) -> TtyResult<()> {
    let (col, row) = _pos().unwrap();
    _goto(col - n, row)
}

pub fn _pos() -> Result<(i16, i16)> {
        let handle = Handle::conout()?;
        let info = ConsoleInfo::of(&handle)?;
        Ok(info.cursor_pos())
}

// pub fn _save_pos() -> TtyResult<()> {
//     // (imdaveho) NOTE: Windows stores the position
//     // in a variable. This would be most appropriate
//     // at the `Tty` level using the `Metadata` struct
//     // 1) fetch the position and 2) store it in this
//     // mutable variable
//     // (imdaveho) TODO: This pattern should simply be 
//     // used for Unix as well so that we can store 
//     // multiple marks at various points in the program 
//     // to load by name or index later 
//     Ok(())
// }

// pub fn _load_pos() -> TtyResult<()> {
//     // (imdaveho) NOTE: Windows stores the position
//     // in a variable. This would be most appropriate
//     // at the `Tty` level using the `Metadata` struct
//     // 1) fetch the stored pos and 2) call `goto` to
//     // restore the stored position
//     // (imdaveho) TODO: This pattern should simply be 
//     // used for Unix as well so that we can store 
//     // multiple marks at various points in the program 
//     // to load by name or index later 
//     Ok(())
// }

pub fn _hide() -> TtyResult<()> {
    let cursor_info = CONSOLE_CURSOR_INFO {
        dwSize: 100,
        bVisible: 0,
    };
    let handle = Handle::conout()?;
    unsafe {
        if SetConsoleCursorInfo(handle.0, &cursor_info) == 0 {
            return Err(TtyErrorKind::IoError(Error::last_os_error()));
        }
    }
    Ok(())
}

pub fn _show() -> TtyResult<()> {
    let cursor_info = CONSOLE_CURSOR_INFO {
        dwSize: 100,
        bVisible: 1,
    };
    let handle = Handle::conout()?;
    unsafe {
        if SetConsoleCursorInfo(handle.0, &cursor_info) == 0 {
            return Err(TtyErrorKind::IoError(Error::last_os_error()));
        }
    }
    Ok(())
}