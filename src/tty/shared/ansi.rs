//! This module exports shared functionality to support ANSI escape sequences.

use std::io::{stdout, BufWriter, Write};


pub fn ansi_write(string: &str, flush: bool) {
    let output = stdout();
    let lock = output.lock();
    let mut outbuf = BufWriter::new(lock);
    outbuf.write(string.as_bytes()).expect("I/O error on write");
    if flush { outbuf.flush().expect("I/O error on flush") }
}

pub fn ansi_flush() {
    let output = stdout();
    let lock = output.lock();
    let mut outbuf = BufWriter::new(lock);
    outbuf.flush().expect("I/O error on flush");
}

// Append a the first few characters of an ANSI escape code to the given string.
#[macro_export]
macro_rules! csi {
    ($( $l:expr ),*) => { concat!("\x1B[", $( $l ),*) };
}
