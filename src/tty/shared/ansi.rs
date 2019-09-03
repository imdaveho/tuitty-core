//! This module exports shared functionality to support ANSI escape sequences.

use std::io::{stdout, BufWriter, Write};


pub fn write_ansi(s: &str) {
    let cout = stdout();
    let lock = cout.lock();
    let mut writer = BufWriter::new(lock);
    writer.write(s.as_bytes()).unwrap();
}

pub fn flush_ansi() {
    let cout = stdout();
    let lock = cout.lock();
    let mut writer = BufWriter::new(lock);
    writer.flush().unwrap();
}

// Append a the first few characters of an ANSI escape code to the given string.
#[macro_export]
macro_rules! csi {
    ($( $l:expr ),*) => { concat!("\x1B[", $( $l ),*) };
}
