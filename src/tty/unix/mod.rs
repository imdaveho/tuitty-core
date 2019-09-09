//! This module consolidates the parts that make up a TTY into a single struct
//! and exposes the methods that work specifically on Unix systems.

use super::cursor;
use super::input;
use super::screen::{self, Clear::*};
use super::output::{self, Color, Format};
use super::shared::{
    ansi_write, ansi_flush,
    UnicodeWidthChar, UnicodeWidthStr,
};
use super::{AsyncReader, SyncReader, Termios};


pub struct Tty {
    index: usize,
    metas: Vec<Metadata>,
    original_mode: Termios,
    autoflush: bool,
}

pub struct Metadata {
    is_raw_enabled: bool,
    is_mouse_enabled: bool,
    is_cursor_visible: bool,
    cursor_pos: (i16, i16),
    cell_style: CellStyle,
    backbuf: Vec<Option<Cell>>,
    screen_size: (i16, i16),
}

#[derive(Clone)]
struct Cell {
    ch: char,
    style: CellStyle,
    width: usize,
}

#[derive(Clone, Copy)]
struct CellStyle {
    fg: Color,
    bg: Color,
    fmts: [Option<Format>; 6],
}


impl Tty {
    pub fn init() -> Tty {
        let (w, h) = screen::ansi::size();
        Tty {
            index: 0,
            metas: vec![Metadata {
                is_raw_enabled: false,
                is_mouse_enabled: false,
                is_cursor_visible: true,
                cursor_pos: (0, 0),
                // (imdaveho) NOTE: Reason for this is to record
                // char styles into the backbuf. There is no other
                // way to fetch this information aside from setting
                // an explicit variable to store this metadata.
                cell_style: Default::default(),
                backbuf: vec![None; (w * h) as usize],
                screen_size: (w, h),
            }],
            original_mode: output::ansi::get_mode().unwrap(),
            autoflush: false,
        }
    }

    pub fn terminate(&mut self) {
        self.to_main();
        output::ansi::set_mode(&self.original_mode).unwrap();
        ansi_write(&cursor::ansi::show(), false);
        ansi_write("\r", true);
        self.metas.clear();
    }

    pub fn size(&self) -> (i16, i16) {
        // Unix specific: depends on `libc`.
        screen::ansi::size()
    }

    pub fn screen_size(&self) -> (i16, i16) {
        (&self.metas[self.index]).screen_size
    }

    // "cooked" vs "raw" mode terminology from Wikipedia:
    // https://en.wikipedia.org/wiki/Terminal_mode
    // A terminal mode is one of a set of possible states of a terminal or
    // pseudo terminal character device in Unix-like systems and determines how
    // characters written to the terminal are interpreted. In cooked mode data
    // is preprocessed before being given to a program, while raw mode passes
    // data as-is to the program without interpreting any special characters.
    pub fn raw(&mut self) {
        // Unix specific: depends on `libc::Termios`.
        output::ansi::enable_raw().unwrap();
        self.metas[self.index].raw();
    }

    pub fn cook(&mut self) {
        // Unix specific: depends on `libc::Termios`.
        output::ansi::set_mode(&self.original_mode).unwrap();
        self.metas[self.index].cook();
    }

    // Input module functions are OS-specific.
    // * enable/disable mouse
    // * read_char/sync/async/until_async

    pub fn enable_mouse(&mut self) {
        ansi_write(&input::ansi::enable_mouse_mode(), self.autoflush);
        self.metas[self.index].enable_mouse();
    }

    pub fn disable_mouse(&mut self) {
        ansi_write(&input::ansi::disable_mouse_mode(), self.autoflush);
        self.metas[self.index].disable_mouse();
    }

    pub fn read_char(&self) -> char {
        input::ansi::read_char()
            .expect("Could not read the character")
    }

    pub fn read_sync(&self) -> SyncReader {
        input::ansi::read_sync()
    }

    pub fn read_async(&self) -> AsyncReader {
        input::ansi::read_async()
    }

    pub fn read_until_async(&self, delimiter: u8) -> AsyncReader {
        input::ansi::read_until_async(delimiter)
    }


    pub fn clear(&mut self, method: &str) {
        match method {
            "all" => {
                let meta = &mut (self.metas[self.index]);
                ansi_write(&screen::ansi::clear(All), self.autoflush);
                meta.backbuf = vec![None; meta.buffer_size()];
                self.goto(0, 0);
            }
            "newln" => {
                let meta = &mut (self.metas[self.index]);
                ansi_write(&screen::ansi::clear(NewLn), self.autoflush);
                for i in meta.buffer_pos()
                    ..meta.buffer_newln_pos() { meta.backbuf[i] = None }
            }
            "currentln" => {
                let row = {
                    let meta = &mut (self.metas[self.index]);
                    ansi_write(&screen::ansi::clear(CurrentLn), self.autoflush);
                    for i in meta.buffer_pos()
                        ..meta.buffer_newln_pos() { meta.backbuf[i] = None }
                    meta.cursor_pos.1
                };
                self.goto(0, row);
            }
            "cursorup" => {
                let meta = &mut (self.metas[self.index]);
                ansi_write(&screen::ansi::clear(CursorUp), self.autoflush);
                for i in 0..=meta.buffer_pos() { meta.backbuf[i] = None }
            }
            "cursordn" => {
                let meta = &mut (self.metas[self.index]);
                ansi_write(&screen::ansi::clear(CursorDn), self.autoflush);
                for i in meta.buffer_pos()
                    ..meta.buffer_size() { meta.backbuf[i] = None }
            }
            // (imdaveho) TODO: implement backspace / delete or "cell"
            _ => ()
        }
    }

    pub fn resize(&mut self, w: i16, h: i16) {
        // NOTE (imdaveho): this method must call `flush`
        // otherwise nothing happens.
        ansi_write(&screen::ansi::resize(w, h), true);
        let meta = &mut self.metas[self.index];
        meta.screen_size = (w, h);
        meta.backbuf.resize((w * h) as usize, None);
    }

    pub fn manual(&mut self) {
        self.autoflush = false;
    }

    pub fn automatic(&mut self) {
        self.autoflush = true;
    }

    pub fn switch(&mut self) {
        // In order to support multiple "screens", this function creates a new
        // Metadata entry which stores any screen specific changes that a User
        // might want to be restored when switching between screens.
        if self.index == 0 {
            // There is no point to switch if you're on another screen
            // since ANSI terminals provide a single "alternate screen".
            ansi_write(&screen::ansi::enable_alt(), false);
        }
        // Add new `Metadata` for the new screen and increment the index.
        self._add_metadata();
        self.index = self.metas.len() - 1;
        // If this wasn't a switch to the alternate screen (ie. the current
        // screen is already the alternate screen), then we need to clear it.
        if self.index >= 1 {
            ansi_write(&screen::ansi::clear(All), self.autoflush);
        };
        // Prevent multiple `flush()` calls due to `autoflush` setting.
        let autoflush = self.autoflush;
        if self.autoflush { self.manual() }
        // Explicitly set default screen settings.
        self.reset();
        self.cook();
        self.disable_mouse();
        self.show_cursor();
        self.goto(0, 0);
        // Revert back to previous `autoflush` configuration.
        if autoflush { self.flush(); self.automatic() }
    }


    pub fn to_main(&mut self) {
        // Only execute if the User is not on the main screen buffer.
        if self.index == 0 { return }
        self.switch_to(0);
    }


    pub fn switch_to(&mut self, index: usize) {
        // If the id and the current id are the same, well, there is nothing to
        // do, you're already on the active screen buffer.
        if index == self.index { return }

        self.index = index;
        let meta = &self.metas[index];
        let raw = meta.is_raw_enabled;
        let mouse = meta.is_mouse_enabled;
        let show = meta.is_cursor_visible;

        // Restore screen contents.
        // (imdaveho) TODO: Confirm if main screen will have native buffer logs,
        // thereby not needing to restore content manually via library. Also,
        // because there is going to be output that is not from `tty` which is
        // not possible to save in the backbuf.
        if index == 0 {
            ansi_write(&screen::ansi::disable_alt(), false)
        } else {
            ansi_write(&screen::ansi::clear(All), false);
            ansi_write(&cursor::ansi::goto(0, 0), false);
            self._flush_backbuf(index);
            let (col, row) = self.metas[index].cursor_pos;
            self.goto(col, row);
        }

        // Prevent multiple `flush()` calls due to `autoflush` setting.
        let autoflush = self.autoflush;
        if self.autoflush { self.manual() }
        // Restore settings based on metadata.
        if raw { self.raw() } else { self.cook() }
        if mouse { self.enable_mouse() } else { self.disable_mouse() }
        if show { self.show_cursor() } else { self.hide_cursor() }

        // Revert back to previous `autoflush` configuration.
        // (imdaveho) NOTE: `_flush_backbuf` always calls `flush` so there
        // is no need to call it again below as `switch()` does.
        if autoflush { self.automatic() }
    }

    pub fn goto(&mut self, col: i16, row: i16) {
        ansi_write(&cursor::ansi::goto(col, row), self.autoflush);
        self.metas[self.index].sync_pos(col, row);
    }

    pub fn up(&mut self) {
        // TODO: implement cursor wrapping if not native.
        ansi_write(&cursor::ansi::move_up(1), self.autoflush);
        self.metas[self.index].vsync_up(1);
    }

    pub fn dn(&mut self) {
        // TODO: implement cursor wrapping if not native.
        ansi_write(&cursor::ansi::move_down(1), self.autoflush);
        self.metas[self.index].vsync_dn(1);
    }

    pub fn left(&mut self) {
        // TODO: implement cursor wrapping if not native.
        ansi_write(&cursor::ansi::move_left(1), self.autoflush);
        self.metas[self.index].hsync_lt(1);
    }

    pub fn right(&mut self) {
        // TODO: implement cursor wrapping if not native.
        ansi_write(&cursor::ansi::move_right(1), self.autoflush);
        self.metas[self.index].hsync_gt(1);
    }

    pub fn dpad(&mut self, dir: &str, n: i16) {
        // (imdaveho) NOTE: Only deal with non-negative `n`. We use i16 to
        // mirror types for getting cursor position and returning terminal size.
        if n < 0 { return }
        // Case-insensitive.
        let d = dir.to_lowercase();
        let meta = &mut self.metas[self.index];
        if n > 0 {
            match d.as_str() {
                "up" => {
                    ansi_write(&cursor::ansi::move_up(n), self.autoflush);
                    meta.vsync_up(n)
                },
                "dn" => {
                    ansi_write(&cursor::ansi::move_down(n), self.autoflush);
                    meta.vsync_dn(n)
                },
                "left" => {
                    ansi_write(&cursor::ansi::move_left(n), self.autoflush);
                    meta.hsync_lt(n)
                },
                "right" => {
                    ansi_write(&cursor::ansi::move_right(n), self.autoflush);
                    meta.hsync_gt(n)
                },
                _ => ()
            };
        }
    }

    pub fn pos(&mut self) -> (i16, i16) {
        let (col, row) = if self.metas[self.index].is_raw_enabled {
            cursor::ansi::pos_raw().unwrap()
        } else {
            self.raw();
            let (col, row) = cursor::ansi::pos_raw().unwrap();
            self.cook();
            (col, row)
        };
        self.metas[self.index].sync_pos(col, row);
        return (col, row);
    }

    pub fn mark(&mut self) {
        ansi_write(&cursor::ansi::save_pos(), self.autoflush);
    }

    pub fn load(&mut self) {
        ansi_write(&cursor::ansi::load_pos(), true);
        // NOTE: On ANSI we need to flush immediately after load so that the
        // call to `pos()` can properly sync `cursor_pos` metadata.
        self.pos();
    }

    pub fn hide_cursor(&mut self) {
        ansi_write(&cursor::ansi::hide(), self.autoflush);
        self.metas[self.index].hide_cursor();
    }

    pub fn show_cursor(&mut self) {
        ansi_write(&cursor::ansi::show(), self.autoflush);
        self.metas[self.index].show_cursor();
    }

    pub fn set_fg(&mut self, color: &str) {
        let color = Color::from(color);
        ansi_write(&output::ansi::set_fg(color), self.autoflush);
        self.metas[self.index].set_fg(color)
    }

    pub fn set_bg(&mut self, color: &str) {
        let color = Color::from(color);
        ansi_write(&output::ansi::set_bg(color), self.autoflush);
        self.metas[self.index].set_bg(color);
    }

    pub fn set_fmt(&mut self, format: &str) {
        // (imdaveho) NOTE: `format` will be `reset` if the passed in
        // `&str` contains multiple values (eg. "bold, underline").
        let fmt = Format::from(format);
        ansi_write(&output::ansi::set_fmt(fmt), self.autoflush);
        self.metas[self.index].set_fmt(fmt);
    }

    pub fn set_fg_rgb(&mut self, r: u8, g:u8, b: u8) {
        let color = output::Color::Rgb{
            r: r,
            g: g,
            b: b,
        };
        ansi_write(&output::ansi::set_fg(color), self.autoflush);
        self.metas[self.index].set_fg(color)
    }

    pub fn set_bg_rgb(&mut self, r: u8, g:u8, b: u8) {
        let color = output::Color::Rgb{
            r: r,
            g: g,
            b: b,
        };
        ansi_write(&output::ansi::set_bg(color), self.autoflush);
        self.metas[self.index].set_bg(color)
    }

    pub fn set_fg_ansi(&mut self, value: u8) {
        let color = output::Color::AnsiValue(value);
        ansi_write(&output::ansi::set_fg(color), self.autoflush);
        self.metas[self.index].set_fg(color)
    }

    pub fn set_bg_ansi(&mut self, value: u8) {
        let color = output::Color::AnsiValue(value);
        ansi_write(&output::ansi::set_bg(color), self.autoflush);
        self.metas[self.index].set_bg(color)
    }

    pub fn set_style(&mut self, fg: &str, bg: &str, fmts: &str) {
        // The params fg is a single word, bg is also a single word, however
        // the tx param can be treated as a comma-separated list of words that
        // match the various text styles that are supported: "bold", "dim",
        // "underline", "reverse", "hide", and "reset".
        ansi_write(&output::ansi::set_all(fg, bg, fmts), self.autoflush);
        self.metas[self.index].set_style(fg, bg, fmts)
    }

    pub fn reset(&mut self) {
        ansi_write(&output::ansi::reset(), self.autoflush);
        self.metas[self.index].cell_style = Default::default()
    }

    pub fn prints(&mut self, string: &str) {
        // TODO: THEN TEST ON WINDOWS!
        // TODO: THEN TEST test_screen...
        let coords = &self.metas[self.index].cursor_pos;
        ansi_write(&cursor::ansi::goto(0, 25), false);
        ansi_write(&format!("col: {}, row: {}", coords.0, coords.1), true);
        ansi_write(&cursor::ansi::goto(0, 26), false);
        ansi_write(&format!("buffer_pos: {}", &self.metas[self.index].buffer_pos()), true);
        ansi_write(&cursor::ansi::goto(coords.0, coords.1), false);
        // BUG: -->
        self._write_backbuf(string);

        ansi_write(&string, false);
        // DEBUG: -->
        let (col, row) = &self.metas[self.index].cursor_pos;
        ansi_write(&cursor::ansi::goto(0, 27), false);
        ansi_write(&format!("aft col: {}, aft row: {}", col, row), true);
        ansi_write(&cursor::ansi::goto(0, 28), false);
        ansi_write(&format!("aft buffer_pos: {}", &self.metas[self.index].buffer_pos()), true);
    }

    pub fn flush(&mut self) {
        ansi_flush();
    }

    pub fn printf(&mut self, string: &str) {
        self.prints(string);
        ansi_flush();
    }

    // pub fn paint() {
    //     // write with colors and styles
    // }

    // pub fn render() {
    //     // write from a template
    // }

    // pub fn intellisense() {
    //     // write from a set of rules
    //     // eg. syntax highlighting
    // }



    fn _add_metadata(&mut self) {
        // TODO: perhaps this should always be default?
        let metas = &mut self.metas;
        let rstate = metas[self.index].is_raw_enabled;
        let mstate = metas[self.index].is_mouse_enabled;
        let cstate = metas[self.index].is_cursor_visible;
        let (w, h) = screen::ansi::size();
        metas.push(Metadata{
            is_raw_enabled: rstate,
            is_mouse_enabled: mstate,
            is_cursor_visible: cstate,
            cursor_pos: (0, 0),
            cell_style: CellStyle {
                fg: Color::Reset,
                bg: Color::Reset,
                fmts: [None; 6],
            },
            backbuf: vec![None; (w * h) as usize],
            screen_size: (w, h),
        });
    }

    fn _flush_backbuf(&mut self, index: usize) {
        let meta = &self.metas[index];
        let backbuf = &meta.backbuf;
        let capacity = meta.buffer_size();
        let mut frontbuf = String::with_capacity(capacity);
        let mut last_style: CellStyle = Default::default();
        for item in backbuf {
            // (imdaveho) NOTE: stackoverflow.com/questions/
            // 23975391/how-to-convert-a-string-into-a-static-str
            let length = UnicodeWidthStr::width(&*frontbuf) as isize;
            match item {
                Some(cell) => {
                    if capacity as isize - (
                        length + cell.width as isize) < 0 { break }

                    // TODO: this might be broken;
                    // if cell.style != last_style {
                    //     frontbuf.push_str(&output::ansi::reset());

                    //     if cell.style.fg != last_style.fg {
                    //         frontbuf.push_str(
                    //             &output::ansi::set_fg(cell.style.fg))
                    //     }
                    //     if cell.style.bg != last_style.bg {
                    //         frontbuf.push_str(
                    //             &output::ansi::set_bg(cell.style.bg))
                    //     }
                    //     if &cell.style.fmts[..] != &last_style.fmts[..] {
                    //         for fmt in cell.style.fmts.iter() {
                    //             if let Some(f) = fmt {
                    //                 frontbuf.push_str(
                    //                     &output::ansi::set_fmt(*f))
                    //             }
                    //         }
                    //     }
                    // }

                    frontbuf.push(cell.ch);
                    last_style = cell.style;
                }
                None => {
                    if capacity as isize - (length + 1) < 0 { break }
                    // (imdaveho) NOTE: This is to prevent multiple reset seqs
                    // from being pushed on the frontbuf String.
                    if last_style == Default::default() {
                        frontbuf.push(' ');
                    } else {
                        frontbuf.push(' ');
                        frontbuf.push_str(&output::ansi::reset());
                        last_style = Default::default();
                    }

                }
            }
        }
        ansi_write(&frontbuf, true);
    }

    fn _write_backbuf(&mut self, string: &str) {
        let meta = &mut self.metas[self.index];

        let length = UnicodeWidthStr::width(string);
        let chars = string.chars();

        let (w, _) = meta.screen_size;  // TODO: `h` to be used when truncating
        let bufpos = meta.buffer_pos();
        let newpos = bufpos + length;
        let new_col = newpos % w as usize;
        let new_row = newpos / w as usize;

        // (imdaveho) NOTE: Remember that buffer indices are 0-based, which
        // means that index 0 (col: 0, row: 0) is actually capacity: 1.
        //
        // If length == capacity, the cursor will overflow by 1, so subtract it.
        // TODO: Truncate the first n rows, and print the overflow n rows. Needs
        // to handle control characters in loop...
        // let capacity = meta.buffer_size();
        // if length > capacity - 1 { return };

        let mut i = 0;
        for ch in chars {
            match UnicodeWidthChar::width(ch) {
                Some(w) => {
                    // (imdaveho) NOTE: The only control character that returns
                    // Some() is the null byte. If for some reason, there is a
                    // null byte passed within the &str parameter, we should
                    // simple ignore it and not update the backbuf.
                    if ch == '\x00' { continue } ;

                    meta.backbuf[i + bufpos] = Some(Cell {
                        ch: ch,
                        width: w,
                        style: meta.cell_style,
                    });
                    i += 1;
                }
                None => {
                    // (imdaveho) NOTE: This is an escape sequence or a `char`
                    // with ambiguous length defaulting to `::width()` == 1 or
                    // `::width_cjk()` == 2.

                    // (imdaveho) TODO: This would only happen if the
                    // user is trying to manually write an escape sequence.
                    // Attempt to interpret what the escape sequence is, and
                    // update meta.cell_style with the details of the sequence.
                    // Difficulty: medium/hard -
                    // * create a byte vector that fills with an ansi esc seq
                    // * when you hit a printable char, take the byte vector,
                    //   and map it to a cell style (medium) or specific
                    //   ANSII function (hard).
                    ()
                }
            }
        }
        // self.goto(new_col as i16, new_row as i16);
        meta.cursor_pos = (new_col as i16, new_row as i16);
    }
}


impl Drop for Tty {
    fn drop(&mut self) {
        self.terminate()
    }
}

impl PartialEq for CellStyle {
    fn eq(&self, other: &CellStyle) -> bool {
        self.fg == other.fg
            && self.bg == other.bg
            && &self.fmts[..] == &other.fmts[..]
    }
}

impl Default for CellStyle {
    fn default() -> Self {
        CellStyle {
            fg: Color::Reset,
            bg: Color::Reset,
            fmts: [None; 6],
        }
    }
}


impl Metadata {
    // Toggle: raw, mouse, cursor
    fn raw(&mut self) { self.is_raw_enabled = true }
    fn cook(&mut self) { self.is_raw_enabled = false }
    fn enable_mouse(&mut self) { self.is_mouse_enabled = true }
    fn disable_mouse(&mut self) { self.is_mouse_enabled = false }
    fn show_cursor(&mut self) { self.is_cursor_visible = true }
    fn hide_cursor(&mut self) { self.is_cursor_visible = false }


    // Backbuf: helper functions
    fn buffer_size(&self) -> usize {
        // returns the size (capacity) of the backbuf
        let (w, h) = self.screen_size;
        return (w * h) as usize
    }

    fn buffer_pos(&self) -> usize {
        // returns the calculated index of the buffer from
        // the cursor position
        let w = self.screen_size.0;
        let (col, row) = self.cursor_pos;
        return ((row * w) + col) as usize
    }

    fn buffer_newln_pos(&self) -> usize {
        // returns the calculated index of column 0 for the
        // next row / new line
        let w = self.screen_size.0;
        let row = self.cursor_pos.1;
        return ((row + 1) * w) as usize
    }

    // Cursor: setter functions
    fn sync_pos(&mut self, col: i16, row: i16) {
        self.cursor_pos = (col, row)
    }

    fn hsync_lt(&mut self, n: i16) {
        if n < 0 { return }
        if (self.cursor_pos.0 - n) > 0 {
            self.cursor_pos.0 -= n
        } else { self.cursor_pos.0 = 0 }
    }

    fn hsync_gt(&mut self, n: i16) {
        if n < 0 { return }
        let w = self.screen_size.0;
        if (self.cursor_pos.0 + n) < w {
            self.cursor_pos.0 += n
        } else { self.cursor_pos.0 = w }
    }

    fn vsync_up(&mut self, n: i16) {
        if n < 0 { return }
        if (self.cursor_pos.1 - n) > 0 {
            self.cursor_pos.1 -= n
        } else { self.cursor_pos.1 = 0 }
    }

    fn vsync_dn(&mut self, n: i16) {
        if n < 0 { return }
        let h = self.screen_size.1;
        if (self.cursor_pos.1 + n) < h {
            self.cursor_pos.1 += n
        } else { self.cursor_pos.1 = h }
    }

    // CellStyle: setter functions
    fn set_fg(&mut self, color: Color) {
        self.cell_style.fg = color
    }

    fn set_bg(&mut self, color: Color) {
        self.cell_style.bg = color
    }

    fn set_fmt(&mut self, format: Format) {
        match format {
            Format::Reset => self.cell_style.fmts[0] = Some(format),
            Format::Dim => self.cell_style.fmts[1] = Some(format),
            Format::Bold => self.cell_style.fmts[2] = Some(format),
            Format::Underline => self.cell_style.fmts[3] = Some(format),
            Format::Reverse => self.cell_style.fmts[4] = Some(format),
            Format::Hide => self.cell_style.fmts[5] = Some(format),
        }
    }

    fn set_style(&mut self, fg: &str, bg: &str, fmts: &str) {
        self.set_fg(Color::from(fg));
        self.set_bg(Color::from(bg));

        let fmt_arr: Vec<&str> = fmts.split(',').map(|t| t.trim()).collect();
        for fmt in fmt_arr.iter() {
            let format = Format::from(*fmt);
            self.set_fmt(format);
        }
    }
}
