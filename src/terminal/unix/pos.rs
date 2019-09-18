// Unix specific implementation of getting the cursor position.
//
// While ANSI is supported on Windows, the Console API provides a more direct
// method of providing cursor position, where as on Unix systems, there is a
// hard dependency that the terminal be set in raw mode. This difference makes
// the ANSI implementation of cursor position more suited to be Unix specific.

use std::io::{ stdin, stdout, Result, BufRead, Write };


pub fn pos() -> Result<(i16, i16)> {
    // Store the current Termios settings.
    let mode = super::get_mode();
    // (imdaveho) NOTE: Enable raw mode regardless of whether or not
    // raw mode has been previously set. This ensures that there will
    // not be any issues getting the cursor position. This will revert
    // back to the current Termios settings stored above.
    super::enable_raw();
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
    // Revert back to Termios settings at the start.    
    super::set_mode(&mode);

    Ok(((cols - 1) as i16, (rows - 1) as i16))
}
