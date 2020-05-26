use std::io::{self, Write, BufRead};


fn pos_raw() -> (i16, i16) {
    let ln = 4;
    // Where is the cursor?
    // Use `ESC [ 6 n`.
    let mut stdout = io::stdout();
    let stdin = io::stdin();

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


fn main() {
    let check = "";
}
