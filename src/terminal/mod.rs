mod ansi;

#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod wincon;

#[cfg(windows)]
mod windows;


trait Teletype {
    fn init();
    fn terminate();
    fn manual();
    fn automatic();
    fn screen_pos(); // cursor::pos
    fn screen_size(); // screen::size
    // screen
    fn clear();
    fn resize();
    // cursor
    fn goto();
    fn up();
    fn down();
    fn left();
    fn right();
    fn moves();
    fn mark_pos();
    fn load_pos();
    fn hide_cursor();
    fn show_cursor();
    // style
    fn set_fg(); // Style
    fn set_bg(); // STyle
    fn set_fx();
    fn set_styles(); // Color, u32
    fn reset_styles();
    // output
    fn prints();
    fn flush();
    fn printf();
}