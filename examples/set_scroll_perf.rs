use std::io::{stdout, BufWriter, Write};
use std::thread;
use std::time::Duration;


fn prints(content: &str) {
    let output = stdout();
    let lock = output.lock();
    let mut outbuf = BufWriter::new(lock);
    outbuf.write_all(content.as_bytes()).expect("I/O error on write");
}

fn flush() {
    let output = stdout();
    let lock = output.lock();
    let mut outbuf = BufWriter::new(lock);
    outbuf.flush().expect("I/O error on flush");
}

fn printf(content: &str) {
    let output = stdout();
    let lock = output.lock();
    let mut outbuf = BufWriter::new(lock);
    outbuf.write_all(content.as_bytes()).expect("I/O error on write");
    outbuf.flush().expect("I/O error on flush");
}

fn csr(top: usize, bot: usize) -> String {
    format!("\x1B[{};{}r", top, bot)
}

fn goto(col: i16, row: i16) -> String {
    format!("\x1B[{};{}H", row + 1, col + 1)
}

fn il(n: usize) -> String {
    format!("\x1B[{}L", n)
}

fn dl(n: usize) -> String {
    format!("\x1B[{}M", n)
}

fn insert(n: usize) {
    prints(&csr(9, 23));
    printf(&goto(0, 8));

    thread::sleep(Duration::from_millis(2000));

    prints(&il(n));
    flush();
}

fn delete(n: usize) {
    prints(&csr(9, 23));
    printf(&goto(0, 8));

    thread::sleep(Duration::from_millis(2000));

    prints(&dl(n));
    flush();

}

fn scrollup() {
    insert(1);
    thread::sleep(Duration::from_millis(2000));
    printf(&"a".repeat(86));
}

fn scrolldown() {
    delete(2);
    printf(&goto(0, 21));
    thread::sleep(Duration::from_millis(2000));
    prints(&"p".repeat(86));
    printf(&"q".repeat(86));
    // printf(&"r".repeat(86));
}


fn main() {
    let (w, h) = (86, 30);
    let alph = ["b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "r"];
    let mut idx = 0;

    for i in 0..h {
        prints(&goto(0, i));
        if i < 8 || i > 22 {
            prints(&".".repeat(w));
        } else {
            prints(&alph[idx].repeat(w));
            idx += 1;
        }
        flush();
    }

    // Line feed example:
    let row = 9;
    prints(&csr(row, h as usize));
    printf(&goto(23, 12));
    thread::sleep(Duration::from_millis(2000));
    // New Line (LF):
    // prints(&goto(23, 13));
    // CRLF:
    prints(&goto(0, 13));
    printf(&il(1));
    thread::sleep(Duration::from_millis(2000));

    // Scroll boundary example: from blessed --
    // https://github.com/dominictarr/hipster/issues/15
    // thread::sleep(Duration::from_millis(2000));

    // scrollup();
    // scrolldown();

    // thread::sleep(Duration::from_millis(2000));

    // Cleanup
    prints(&csr(0, 30));
    printf(&goto(85, 29));
    thread::sleep(Duration::from_millis(2000));
}
