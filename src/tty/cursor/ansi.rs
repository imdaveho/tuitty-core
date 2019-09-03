// ANSI specific functions for controlling the terminal cursor.

use std::io::{stdin, stdout, BufRead, Result, Write};
use crate::csi;


pub fn goto(col: i16, row: i16) -> String {
    format!(csi!("{};{}H"), row + 1, col + 1)
}

pub fn move_up(n: i16) -> String {
    format!(csi!("{}A"), n)
}

pub fn move_right(n: i16) -> String {
    format!(csi!("{}C"), n)
}

pub fn move_down(n: i16) -> String {
    format!(csi!("{}B"), n)
}

pub fn move_left(n: i16) -> String {
    format!(csi!("{}D"), n)
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

pub fn save_pos() -> String {
    csi!("s").to_string()
}

pub fn load_pos() -> String {
    csi!("u").to_string()
}

pub fn hide() -> String {
    csi!("?25l").to_string()
}

pub fn show() -> String {
    csi!("?25h").to_string()
}
