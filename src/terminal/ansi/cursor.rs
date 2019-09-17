// ANSI specific functions for controlling the terminal cursor.

use std::io::{stdin, stdout, Result, Write};


pub fn goto(col: i16, row: i16) -> String {
    format!("\x1B[{};{}H"), row + 1, col + 1)
}

pub fn move_up(n: i16) -> String {
    format!("\x1B[{}A"), n)
}

pub fn move_right(n: i16) -> String {
    format!("\x1B[{}C"), n)
}

pub fn move_dn(n: i16) -> String {
    format!("\x1B[{}B"), n)
}

pub fn move_left(n: i16) -> String {
    format!("\x1B[{}D"), n)
}

pub fn hide_cursor() -> String {
    "\x1B[?25l".to_string()
}

pub fn show_cursor() -> String {
    "\x1B[?25h".to_string()
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

// (imdaveho) NOTE: Implemented internally to work with library features.
// pub fn mark_pos() -> String {
//     "\x1B[s".to_string()
// }

// pub fn load_pos() -> String {
//     "\x1B[u".to_string()
// }
