use super::cache::{ ScreenCache, CacheUpdater };


#[derive(Clone)]
pub struct Metadata {
    is_raw_enabled: bool,
    is_mouse_enabled: bool,
    is_cursor_visible: bool,
    saved_position: (i16, i16),
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

    pub fn _raw(&mut self) {
        self.is_raw_enabled = true;
    }

    pub fn _cook(&mut self) {
        self.is_raw_enabled = false;
    }

    pub fn _is_raw_on(&self) -> bool {
        self.is_raw_enabled
    }

    pub fn _enable_mouse(&mut self) {
        self.is_mouse_enabled = true;
    }

    pub fn _disable_mouse(&mut self) {
        self.is_mouse_enabled = false;
    }

    pub fn _is_mouse_on(&self) -> bool {
        self.is_mouse_enabled
    }

    pub fn _show_cursor(&mut self) {
        self.is_cursor_visible = true;
    }

    pub fn _hide_cursor(&mut self) {
        self.is_cursor_visible = false;
    }

    pub fn _is_cursor_on(&self) -> bool {
        self.is_cursor_visible
    }

    pub fn _mark_position(&mut self) {
        self.saved_position = self.cache._screen_pos();
    }

    pub fn _saved_position(&self) -> (i16, i16) {
        self.saved_position
    }
}
