use super::cache::ScreenCache;


// (imdaveho) TODO: determine terminal default tabstop settings:
// https://superuser.com/questions/1037615/detecting-terminal-tabstop-settings
//
// The tabs program uses data from the terminal database, to tell the terminal
// what tab-stops to use. The computer does not know about the tab-stops: using
// stty you can set the terminal driver to use hard-tabs or soft (the usual
// expansion of 8 columns per tab stop). Whether you set hard/soft tabs, most
// applications on the system will assume 8, anyway. The terminal database does
// not provide a standard way to determine what the tab-stops are set to. For
// the widely-used VT100 emulations in xterm, etc., it is possible to determine
// this information by using the cursor-position report. Someone could write an
// application that wrote tabs, used the cursor-report to see where the cursor
// ended up, and compute the tab-stops. (The resize program uses the
// cursor-position report to determine the screen size). ncurses's terminal
// database provides u6 capabilities which attempt to describe cursor-position
// reports, but for practical purposes only the VT100-style reports are
// supported.


#[derive(Clone)]
pub struct Metadata {
    pub is_raw_enabled: bool,
    pub is_mouse_enabled: bool,
    pub is_cursor_visible: bool,
    pub saved_position: (i16, i16),
    pub cache: ScreenCache,
}

impl Metadata {
    pub fn new() -> Metadata {
        Metadata {
            is_raw_enabled: false,
            is_mouse_enabled: false,
            is_cursor_visible: true,
            saved_position: (0, 0),
            cache: ScreenCache::new(),
        }
    }
}
