extern crate tuitty;

use std::thread;
use std::time::Duration;
#[cfg(not(windows))]
use std::io::{stdout, BufWriter, Write};

#[cfg(unix)]
use tuitty::terminal::actions::posix;

// #[cfg(not(windows))]
// fn prints(content: &str) {
//     let output = stdout();
//     let lock = output.lock();
//     let mut outbuf = BufWriter::new(lock);
//     outbuf.write_all(content.as_bytes()).expect("I/O error on write");
// }

// #[cfg(not(windows))]
// fn flush() {
//     let output = stdout();
//     let lock = output.lock();
//     let mut outbuf = BufWriter::new(lock);
//     outbuf.flush().expect("I/O error on flush");
// }

// #[cfg(not(windows))]
// fn printf(content: &str) {
//     let output = stdout();
//     let lock = output.lock();
//     let mut outbuf = BufWriter::new(lock);
//     outbuf.write_all(content.as_bytes()).expect("I/O error on write");
//     outbuf.flush().expect("I/O error on flush");
// }

#[cfg(unix)]
fn pos_raw() -> (i16, i16) {
    use std::io::{ Write, BufRead };
    let ln = 603;
    // Where is the cursor?
    // Use `ESC [ 6 n`.
    let mut stdout = std::io::stdout();
    let stdin = std::io::stdin();

    // Write command
    stdout.write_all(b"\x1B[6n").expect(&format!(
        "buffer.rs [Ln: {}]: Error writing to stdout", ln + 9));
    stdout.flush().expect(&format!(
        "buffer.rs [Ln: {}]: Error flushing stdout", ln + 11));

    stdin.lock().read_until(b'[', &mut vec![]).expect(&format!(
        "buffer.rs [Ln {}]: Error reading stdin", ln + 14));

    let mut rows = vec![];
    stdin.lock().read_until(b';', &mut rows).expect(&format!(
        "buffer.rs [Ln {}]: Error reading stdin", ln + 18));

    let mut cols = vec![];
    stdin.lock().read_until(b'R', &mut cols).expect(&format!(
        "buffer.rs [Ln {}]: Error reading stdin", ln + 22));

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
        .expect(&format!(
            "buffer.rs [Ln {}]: Error parsing row position.", ln + 29
        ));
    let cols = cols
        .into_iter()
        .map(|b| (b as char))
        .fold(String::new(), |mut acc, n| {
            acc.push(n);
            acc
        })
        .parse::<usize>()
        .expect(&format!(
            "buffer.rs [Ln {}]: Error parsing col position.", ln + 40
        ));

    ((cols - 1) as i16, (rows - 1) as i16)
}

// #[cfg(not(windows))]
// fn goto(col: i16, row: i16) -> String {
//     format!("\x1B[{};{}H", row + 1, col + 1)
// }

// #[cfg(not(windows))]
// fn enable_alt() -> String {
//     "\x1B[?1049h".to_string()
// }

// #[cfg(not(windows))]
// fn disable_alt() -> String {
//     "\x1B[?1049l".to_string()
// }

#[cfg(unix)]
fn main() {
    posix::enable_alt();
    let mode = posix::get_mode();
    posix::raw();

    posix::goto(0, 0);

    // let string = "👨🏽‍👩🏽‍👧🏽";
    let string = &["🧗", "🏽", "\u{200d}", "♀", "\u{fe0f}"].concat();
    // let string = "👨🏿‍🦰";

    // let string = "👨🏽‍👩🏽‍👧🏽cursor - bad";
    // let string = &["🧗", "🏽", "\u{200d}", "♀", "\u{fe0f}", "cursor - good"].concat();
    // let string = "👨🏿‍🦰cursor - good";
    posix::printf(string);

    thread::sleep(Duration::from_millis(2000));
    let (col, row) = pos_raw();

    posix::cook(&mode);
    posix::disable_alt();

    println!("{}, {}", col, row);
}

#[cfg(windows)]
fn main() {
    println!("Not implemented")
}
