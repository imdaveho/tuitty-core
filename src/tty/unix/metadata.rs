//! TODO:


#[derive(Clone)]
pub struct Metadata {
    is_raw_enabled: bool,
    is_mouse_enabled: bool,
    is_cursor_visible: bool,
    pub cellbuf: super::CellBuffer,
}

impl Metadata {
    pub fn new() -> Metadata {
        let (w, h) = super::screen::ansi::size();
        Metadata {
            is_raw_enabled: false,
            is_mouse_enabled: false,
            is_cursor_visible: true,
            cellbuf: super::CellBuffer::new(w, h)
        }
    }

    pub fn _raw(&mut self) {
        self.is_raw_enabled = true;
    }

    pub fn _cook(&mut self) {
        self.is_raw_enabled = false;
    }

    pub fn _is_raw(&self) -> bool {
        self.is_raw_enabled
    }

    pub fn _enable_mouse(&mut self) {
        self.is_mouse_enabled = true;
    }

    pub fn _disable_mouse(&mut self) {
        self.is_mouse_enabled = false;
    }

    pub fn _is_mouse(&self) -> bool {
        self.is_mouse_enabled
    }

    pub fn _show_cursor(&mut self) {
        self.is_cursor_visible = true;
    }

    pub fn _hide_cursor(&mut self) {
        self.is_cursor_visible = false;
    }

    pub fn _is_cursor(&self) -> bool {
        self.is_cursor_visible
    }
}
