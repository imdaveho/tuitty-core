//! This module sets the cursor position
//! with given coordinates in the visible
//! part of the TTY. It also shows/hides
//! the visibility of the input cursor.
#[cfg(unix)]
use crate::{TtyResult};

#[cfg(unix)]
mod linux;

#[cfg(unix)]
pub use linux::{
    _goto as goto,
    _move_up as move_up,
    _move_down as move_down,
    _move_left as move_left,
    _move_right as move_right,
    _hide as hide,
    _show as show,
    _pos_raw as pos_raw,
    _save_pos as save_pos,
    _load_pos as load_pos,
};

#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use windows::{
    _goto as goto,
    _move_up as move_up,
    _move_down as move_down,
    _move_left as move_left,
    _move_right as move_right,
    _hide as hide,
    _show as show,
    _pos as pos,
};
