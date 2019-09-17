// ANSI specific methods to print to the terminal.

use std::io::{stdout, BufWriter, Write};

pub fn prints(content: &str) {
    let output = stdout();
    let lock = output.lock();
    let mut outbuf = BufWriter::new(lock);
    outbuf.write(content.as_bytes()).expect("I/O error on write");
}

pub fn flush() {
    let output = stdout();
    let lock = output.lock();
    let mut outbuf = BufWriter::new(lock);
    outbuf.flush().expect("I/O error on flush");
}

pub fn printf(content: &str) {
    let output = stdout();
    let lock = output.lock();
    let mut outbuf = BufWriter::new(lock);
    outbuf.write(content.as_bytes()).expect("I/O error on write");
    outbuf.flush().expect("I/O error on flush");
}

pub fn outputs(content: &str, flush: bool) {
    let output = stdout();
    let lock = output.lock();
    let mut outbuf = BufWriter::new(lock);
    outbuf.write(content.as_bytes()).expect("I/O error on write");
    if flush { outbuf.flush().expect("I/O error on flush") }
}