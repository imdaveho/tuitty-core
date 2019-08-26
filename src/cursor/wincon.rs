// Windows Console API specific function for controlling the terminal cursor.

use std::io::{Error, ErrorKind, Result};
use winapi::um::wincon::{
    SetConsoleCursorInfo, SetConsoleCursorPosition,
    COORD, CONSOLE_CURSOR_INFO,
};
use crate::shared::{ConsoleInfo, Handle};


pub fn _goto(col: i16, row: i16) -> Result<()> {
    if col < 0 || col >= <i16>::max_value() {
        return Err(Error::new(
            ErrorKind::Other,
            format!(
                "Argument Out of Range Exception
                when setting cursor position to X: {}", col
            ),
        ));
    }
    if row < 0 || row >= <i16>::max_value() {
        return Err(Error::new(
            ErrorKind::Other,
            format!(
                "Argument Out of Range Exception
                when setting cursor position to Y: {}", row
            ),
        ));
    }

    let pos = COORD { X: col, Y: row };
    let handle = Handle::conout()?;

    unsafe {
        if SetConsoleCursorPosition(handle.0, pos) == 0 {
            return Err(Error::last_os_error());
        }
    }

    Ok(())
}

pub fn _move_up(n: i16) -> Result<()> {
    let (col, row) = _pos().unwrap();
    _goto(col, row - n)
}

pub fn _move_right(n: i16) -> Result<()> {
    let (col, row) = _pos().unwrap();
    _goto(col + n, row)
}

pub fn _move_down(n: i16) -> Result<()> {
    let (col, row) = _pos().unwrap();
    _goto(col, row + n)
}

pub fn _move_left(n: i16) -> Result<()> {
    let (col, row) = _pos().unwrap();
    _goto(col - n, row)
}

pub fn _pos() -> Result<(i16, i16)> {
        let handle = Handle::conout()?;
        let info = ConsoleInfo::of(&handle)?;
        Ok(info.cursor_pos())
}

pub fn _hide() -> Result<()> {
    let cursor_info = CONSOLE_CURSOR_INFO {
        dwSize: 100,
        bVisible: 0,
    };
    let handle = Handle::conout()?;
    unsafe {
        if SetConsoleCursorInfo(handle.0, &cursor_info) == 0 {
            return Err(Error::last_os_error());
        }
    }
    Ok(())
}

pub fn _show() -> Result<()> {
    let cursor_info = CONSOLE_CURSOR_INFO {
        dwSize: 100,
        bVisible: 1,
    };
    let handle = Handle::conout()?;
    unsafe {
        if SetConsoleCursorInfo(handle.0, &cursor_info) == 0 {
            return Err(Error::last_os_error());
        }
    }
    Ok(())
}
