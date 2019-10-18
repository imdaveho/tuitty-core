// This module provides a wrapper around state changes for each screen that
// gets created by the user to manipulate the terminal.

mod meta;
mod cache;


pub struct TerminalStore {
    pub index: usize,
    pub state: meta::Metadata,
    pub stash: Vec<meta::Metadata>,
}

impl TerminalStore {
    fn new() -> TerminalStore {
        TerminalStore {
            index: 0,
            state: meta::Metadata::new(),
            stash: Vec::with_capacity(5),
        }
    }

    // // Sync CursorActions
    // fn update_goto(&mut self, col: i16, row: i16) {
    //     self.state.cache.sync_pos(col, row);
    // }

    // fn update_up(&mut self, n: i16) {
    //     self.state.cache.sync_up(n);
    // }

    // fn update_down(&mut self, n: i16) {
    //     self.state.cache.sync_down(n);
    // }
    
    // fn update_left(&mut self, n: i16) {
    //     self.state.cache.sync_left(n);
    // }
    
    // fn update_right(&mut self, n: i16) {
    //     self.state.cache.sync_right(n);
    // }
    
}