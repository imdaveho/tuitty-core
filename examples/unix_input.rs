extern crate tuitty;

use std::{ fs, io::{ Read, BufReader }};

#[cfg(unix)]
use tuitty::terminal::actions::posix;

#[cfg(unix)]
fn main() {

    let original_mode = posix::get_mode();
    // posix::enable_alt();
    posix::raw();
    posix::enable_mouse();
    posix::flush();
    // posix::hide_cursor();


    // loop {
        let tty = BufReader::new(
            fs::OpenOptions::new().read(true).write(true)
                .open("/dev/tty")
                .expect("Error opening /dev/tty"));
        let mut buf = Vec::with_capacity(12);
        for byte in tty.bytes() {
            let res = byte.unwrap();
            buf.push(res);
            println!("{:?}", buf);
            buf.clear();
        }
    // }


    // posix::show_cursor();
    posix::disable_mouse();
    posix::cook(&original_mode);
    // posix::disable_alt();

    // thread::sleep(Duration::from_secs(2));
}
