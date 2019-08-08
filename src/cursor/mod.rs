//! This module sets the cursor position
//! with given coordinates in the visible
//! part of the TTY. It also shows/hides
//! the visibility of the input cursor.
use std::io::Result;
use crate::shared::TtyResult;

#[cfg(unix)]
mod linux;


pub fn goto(col: u16, row: u16) -> TtyResult<()> {
    #[cfg(unix)]
    linux::_goto(col, row)

}

pub fn move_up(n: u16) -> TtyResult<()> {
    #[cfg(unix)]
    linux::_move_up(n)
}

pub fn move_right(n: u16) -> TtyResult<()> {
    #[cfg(unix)]
    linux::_move_right(n)
}

pub fn move_down(n: u16) -> TtyResult<()> {
    #[cfg(unix)]
    linux::_move_down(n)
}

pub fn move_left(n: u16) -> TtyResult<()> {
    #[cfg(unix)]
    linux::_move_left(n)
}

// (imdaveho) TODO: reminder to make sure that `get_cursor_position`
// gets implemented in cursor module or wrapped in TTY top-level API.
pub fn pos_raw() -> Result<(u16, u16)> {
    #[cfg(unix)]
    linux::_pos_raw()
}

pub fn save_pos() -> TtyResult<()> {
    #[cfg(unix)]
    linux::_save_pos()
}

pub fn load_pos() -> TtyResult<()> {
    #[cfg(unix)]
    linux::_load_pos()
}

pub fn hide() -> TtyResult<()> {
    #[cfg(unix)]
    linux::_hide()
}

pub fn show() -> TtyResult<()> {
    #[cfg(unix)]
    linux::_show()
}
