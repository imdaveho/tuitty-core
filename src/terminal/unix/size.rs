use libc::{ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ};


pub fn size() -> (i16, i16) {
    // Reference source:
    // http://rosettacode.org/wiki/Terminal_control/Dimensions#Library:_BSD_libc
    let mut size = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ.into(), &mut size) };

    if r == 0 {
        (size.ws_col as i16, size.ws_row as i16)
    } else {
        (0, 0)
    }
}