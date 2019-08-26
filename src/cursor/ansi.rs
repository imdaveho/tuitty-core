// ANSI specific functions for controlling the terminal cursor.

use std::io::{stdin, stdout, BufRead, Result, Write};
use crate::{csi, write_cout};



pub fn goto(col: i16, row: i16) -> Result<()> {
    write_cout!(format!(csi!("{};{}H"), row + 1, col + 1))?;
    Ok(())
}

pub fn move_up(n: i16) -> Result<()> {
    write_cout!(&format!(csi!("{}A"), n))?;
    Ok(())
}

pub fn move_right(n: i16) -> Result<()> {
    write_cout!(&format!(csi!("{}C"), n))?;
    Ok(())
}

pub fn move_down(n: i16) -> Result<()> {
    write_cout!(&format!(csi!("{}B"), n))?;
    Ok(())
}

pub fn move_left(n: i16) -> Result<()> {
    write_cout!(&format!(csi!("{}D"), n))?;
    Ok(())
}

pub fn pos_raw() -> Result<(i16, i16)> {
    // Where is the cursor?
    // Use `ESC [ 6 n`.
    let mut stdout = stdout();
    let stdin = stdin();

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

    Ok(((cols - 1) as i16, (rows - 1) as i16))
}

pub fn save_pos() -> Result<()> {
    write_cout!(csi!("s"))?;
    Ok(())
}

pub fn load_pos() -> Result<()> {
    write_cout!(csi!("u"))?;
    Ok(())
}

pub fn hide() -> Result<()> {
    write_cout!(csi!("?25l"))?;
    Ok(())
}

pub fn show() -> Result<()> {
    write_cout!(csi!("?25h"))?;
    Ok(())
}
