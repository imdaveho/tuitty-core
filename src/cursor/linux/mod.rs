//! Platform specific functions for the library.
use std::io::{self, Write, BufRead};
use crate::{csi, write_cout};
use super::{Result, TtyResult};


pub fn _goto(col: u16, row: u16) -> TtyResult<()> {
    write_cout!(format!(csi!("{};{}H"), row + 1, col + 1))?;
    Ok(())
}

pub fn _move_up(n: u16) -> TtyResult<()> {
    write_cout!(&format!(csi!("{}A"), n))?;
    Ok(())
}

pub fn _move_right(n: u16) -> TtyResult<()> {
    write_cout!(&format!(csi!("{}C"), n))?;
    Ok(())
}

pub fn _move_down(n: u16) -> TtyResult<()> {
    write_cout!(&format!(csi!("{}B"), n))?;
    Ok(())
}

pub fn _move_left(n: u16) -> TtyResult<()> {
    write_cout!(&format!(csi!("{}D"), n))?;
    Ok(())
}

pub fn _pos_raw() -> Result<(u16, u16)> {
    // Where is the cursor?
    // Use `ESC [ 6 n`.
    let mut stdout = io::stdout();
    let stdin = io::stdin();

    // Write command
    stdout.write_all(b"\x1B[6n")?;
    stdout.flush()?;

    stdin.lock().read_until(b'[', &mut vec![])?;

    let mut rows = vec![];
    stdin.lock().read_until(b';', &mut rows).unwrap();

    let mut cols = vec![];
    stdin.lock().read_until(b'R', &mut cols).unwrap();

    // remove delimiter
    rows.pop();
    cols.pop();

    let rows = rows
        .into_iter()
        .map(|b| (b as char))
        .fold(String::new(), |mut acc, n| {
            acc.push(n);
            acc
        })
        .parse::<usize>()
        .unwrap();
    let cols = cols
        .into_iter()
        .map(|b| (b as char))
        .fold(String::new(), |mut acc, n| {
            acc.push(n);
            acc
        })
        .parse::<usize>()
        .unwrap();

    Ok(((cols - 1) as u16, (rows - 1) as u16))
}

pub fn _save_pos() -> TtyResult<()> {
    write_cout!(csi!("s"))?;
    Ok(())
}

pub fn _load_pos() -> TtyResult<()> {
    write_cout!(csi!("u"))?;
    Ok(())
}

pub fn _hide() -> TtyResult<()> {
    write_cout!(csi!("?25l"))?;
    Ok(())
}

pub fn _show() -> TtyResult<()> {
    write_cout!(csi!("?25h"))?;
    Ok(())
}
