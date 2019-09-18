// ANSI specific functions for controlling the terminal cursor.


pub fn goto(col: i16, row: i16) -> String {
    format!("\x1B[{};{}H"), row + 1, col + 1)
}

pub fn move_up(n: i16) -> String {
    format!("\x1B[{}A"), n)
}

pub fn move_right(n: i16) -> String {
    format!("\x1B[{}C"), n)
}

pub fn move_dn(n: i16) -> String {
    format!("\x1B[{}B"), n)
}

pub fn move_left(n: i16) -> String {
    format!("\x1B[{}D"), n)
}

pub fn hide_cursor() -> String {
    "\x1B[?25l".to_string()
}

pub fn show_cursor() -> String {
    "\x1B[?25h".to_string()
}


// (imdaveho) NOTE: Implemented internally to work with library features.
// pub fn mark_pos() -> String {
//     "\x1B[s".to_string()
// }

// pub fn load_pos() -> String {
//     "\x1B[u".to_string()
// }
