// Unix specific functions that wrap the Reader type and parses user input.

use std::{
    os::unix::io::AsRawFd,
    fs, str::from_utf8, sync::atomic::Ordering,
    io::{ Read, BufReader, Error, ErrorKind, Result },
};
use super::reader::{ AsyncReader, SyncReader };


pub fn read_char() -> Result<char> {
    let mut buf = [0u8; 20];

    let fd = unsafe {
        if libc::isatty(libc::STDIN_FILENO) == 1 {
            libc::STDIN_FILENO
        } else {
            let tty_f = fs::File::open("/dev/tty")?;
            tty_f.as_raw_fd()
        }
    };

    let rv = unsafe {
        let read = libc::read(fd, buf.as_mut_ptr() as *mut libc::c_void, 20);

        if read < 0 {
            Err(Error::last_os_error())
        } else {
            let mut pressed_char = Ok(' ');

            if let Ok(s) = from_utf8(&buf[..read as usize]) {
                if let Some(c) = s.chars().next() {
                    pressed_char = Ok(c)
                }
            } else {
                pressed_char = Err(Error::new(
                    ErrorKind::Interrupted,
                    "Could not parse char to utf8 char",
                ));
            }

            return pressed_char;
        }
    };
    return rv;
}

pub fn read_async() -> AsyncReader {
    AsyncReader::new(Box::new(move |event_tx, kill_switch| {
        let systty = BufReader::new(
            fs::OpenOptions::new().read(true).write(true)
                .open("/dev/tty").expect("Error opening /dev/tty"));
        for b in systty.bytes() {
            let ch = b.expect("Error reading byte from /dev/tty");
            if event_tx.send(ch).is_err() {
                return;
            }
            if kill_switch.load(Ordering::SeqCst) {
                return;
            }
        }
    }))
}


pub fn read_until_async(delimiter: u8) -> AsyncReader {
    AsyncReader::new(Box::new(move |event_tx, kill_switch| {
        let systty = BufReader::new(
            fs::OpenOptions::new().read(true).write(true)
                .open("/dev/tty").expect("Error opening /dev/tty"));
        for b in systty.bytes() {
            let ch = b.expect("Error reading byte from /dev/tty");
            let eos = ch == delimiter;
            let send_err = event_tx.send(ch).is_err();

            if eos || send_err || kill_switch.load(Ordering::SeqCst) {
                return;
            }
        }
    }))
}


pub fn read_sync() -> SyncReader {
    let systty = BufReader::new(
        fs::OpenOptions::new().read(true).write(true)
            .open("/dev/tty").expect("Error opening /dev/tty"));
    SyncReader::new(Box::from(systty))
}