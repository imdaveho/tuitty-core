//! This module sets the cursor position
//! with given coordinates in the visible
//! part of the TTY. It also shows/hides
//! the visibility of the input cursor.
use std::io::Result;
use crate::shared::TtyResult;

#[cfg(unix)]
mod linux;

#[cfg(windows)]
mod windows;


pub fn goto(col: i16, row: i16) -> TtyResult<()> {
    #[cfg(unix)] {
        linux::_goto(col, row)
    }

    #[cfg(windows)] {
        windows::_goto(col, row)
    }

}

pub fn move_up(n: i16) -> TtyResult<()> {
    #[cfg(unix)] {
        linux::_move_up(n)
    }

    #[cfg(windows)] {
        windows::_move_up(n)
    }
}

pub fn move_right(n: i16) -> TtyResult<()> {
    #[cfg(unix)] {
        linux::_move_right(n)
    }

    #[cfg(windows)] {
        windows::_move_right(n)
    }
}

pub fn move_down(n: i16) -> TtyResult<()> {
    #[cfg(unix)] {
        linux::_move_down(n)
    }

    #[cfg(windows)] {
        windows::_move_down(n)
    }
}

pub fn move_left(n: i16) -> TtyResult<()> {
    #[cfg(unix)] {
        linux::_move_left(n)
    }

    #[cfg(windows)] {
        windows::_move_left(n)
    }
}

#[cfg(unix)]
pub fn pos_raw() -> Result<(i16, i16)> {
    linux::_pos_raw()
}

#[cfg(windows)]
pub fn pos() -> Result<(i16, i16)> {
    windows::_pos()
}

#[cfg(unix)]
pub fn save_pos() -> TtyResult<()> {
    linux::_save_pos()
}

#[cfg(unix)]
pub fn load_pos() -> TtyResult<()> {
    linux::_load_pos()
}

pub fn hide() -> TtyResult<()> {
    #[cfg(unix)] {
        linux::_hide()
    }

    #[cfg(windows)] {
        windows::_hide()
    }
}

pub fn show() -> TtyResult<()> {
    #[cfg(unix)] {
        linux::_show()
    }

    #[cfg(windows)] {
        windows::_show()
    }
}
