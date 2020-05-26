// Windows Console API specific functions for controlling the terminal cursor.

use std::io::{Error, ErrorKind, Result};
use winapi::um::wincon::{
    SetConsoleCursorInfo,
    SetConsoleCursorPosition,
    COORD, CONSOLE_CURSOR_INFO,
};
use super::handle::{ConsoleInfo, Handle};


pub fn goto(col: i16, row: i16) -> Result<()> {
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
    handle.close()?;
    Ok(())
}

pub fn move_up(n: i16) -> Result<()> {
    let (col, row) = pos()?;
    goto(col, row - n)
}

pub fn move_right(n: i16) -> Result<()> {
    let (col, row) = pos()?;
    goto(col + n, row)
}

pub fn move_down(n: i16) -> Result<()> {
    let (col, row) = pos()?;
    goto(col, row + n)
}

pub fn move_left(n: i16) -> Result<()> {
    let (col, row) = pos()?;
    goto(col - n, row)
}

pub fn hide_cursor() -> Result<()> {
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
    handle.close()?;
    Ok(())
}

pub fn show_cursor() -> Result<()> {
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
    handle.close()?;
    Ok(())
}

pub fn pos() -> Result<(i16, i16)> {
    let handle = Handle::conout()?;
    let info = ConsoleInfo::of(&handle)?;
    handle.close()?;
    Ok(info.cursor_pos())
}
