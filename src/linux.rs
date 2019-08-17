struct Tty {
    original_mode: Termios,
    id: usize,
    meta: Vec<Metadata>,
}

struct Metadata {
    is_raw_enabled: bool,
    is_mouse_enabled: bool,
}

pub fn init() {}
pub fn clear() {}
pub fn resize() {}
pub fn switch() {}
pub fn main() {}
pub fn switch_to() {}
pub fn up() {}
pub fn dn() {}
pub fn left() {}
pub fn right() {}
pub fn dpad() {}
pub fn raw() {}
pub fn cook() {}
pub fn pos() {}
pub fn mark() {}
pub fn load() {}
pub fn hide_cursor() {}
pub fn show_cursor() {}
pub fn enable_mouse() {}
pub fn disable_mouse() {}
pub fn read_char() {}
pub fn read_sync() {}
pub fn read_async() {}
pub fn read_until_async() {}
pub fn exit() {}