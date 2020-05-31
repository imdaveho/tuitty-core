// Windows Console API specific functions for controlling the terminal cursor.

use std::io::{Error, ErrorKind, Result};
use winapi::um::wincon::{
    SetConsoleCursorInfo,
    SetConsoleCursorPosition,
    COORD, CONSOLE_CURSOR_INFO,
};
use super::handle::{ConsoleInfo, Handle};


pub fn goto(col: i16, row: i16, conout: &Handle) -> Result<()> {
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
    unsafe {
        if SetConsoleCursorPosition(conout.0, pos) == 0 {
            return Err(Error::last_os_error());
        }
    }
    Ok(())
}

pub fn move_up(n: i16, conout: &Handle) -> Result<()> {
    let (col, row) = pos(conout)?;
    goto(col, row - n, conout)
}

pub fn move_right(n: i16, conout: &Handle) -> Result<()> {
    let (col, row) = pos(conout)?;
    goto(col + n, row, conout)
}

pub fn move_down(n: i16, conout: &Handle) -> Result<()> {
    let (col, row) = pos(conout)?;
    goto(col, row + n, conout)
}

pub fn move_left(n: i16, conout: &Handle) -> Result<()> {
    let (col, row) = pos(conout)?;
    goto(col - n, row, conout)
}

pub fn hide_cursor(conout: &Handle) -> Result<()> {
    let cursor_info = CONSOLE_CURSOR_INFO {
        dwSize: 100,
        bVisible: 0,
    };
    unsafe {
        if SetConsoleCursorInfo(conout.0, &cursor_info) == 0 {
            return Err(Error::last_os_error());
        }
    }
    Ok(())
}

pub fn show_cursor(conout: &Handle) -> Result<()> {
    let cursor_info = CONSOLE_CURSOR_INFO {
        dwSize: 100,
        bVisible: 1,
    };
    unsafe {
        if SetConsoleCursorInfo(conout.0, &cursor_info) == 0 {
            return Err(Error::last_os_error());
        }
    }
    Ok(())
}

pub fn pos(conout: &Handle) -> Result<(i16, i16)> {
    let info = ConsoleInfo::of(&conout)?;
    Ok(info.cursor_pos())
}
