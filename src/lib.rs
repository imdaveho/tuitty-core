//! (imdaveho) TODO: ...

mod shared;

pub mod screen;
pub mod cursor;
pub mod output;
pub mod input;


#[cfg(unix)]
use libc::termios as Termios;

#[cfg(windows)]
use shared::Termios;


struct Tty {
    original_mode: Termios,
    id: usize,
    meta: Vec<Metadata>,
    #[cfg(windows)]
    altscrn: Option<shared::Handle>,
    #[cfg(windows)]
    reset_color: u16,
}

struct Metadata {
    termios: Termios,
    is_raw: bool,
    #[cfg(windows)]
    marker: Option<(i16, i16)>, // TODO: implement multi markers
}

impl Tty {
    fn init() -> Tty {
        Tty {
            original_mode: output::get_mode().unwrap(),
            id: 0,
            meta: vec![Metadata {
                termios: output::get_mode().unwrap(),
                is_raw: false,
                #[cfg(windows)]
                marker: None,
            }],
            #[cfg(windows)]
            altscrn: None,
            #[cfg(windows)]
            reset_color: {
                shared::ConsoleInfo::of(
                    &shared::Handle::conout().unwrap()
                ).unwrap().attributes()
            },
        }
    }

    fn clear(&self, s: &str) {
        match s {
            "all" => {
                screen::clear(screen::Clear::All).unwrap();
                self.goto(0, 0);
            }
            "currentln" => {
                screen::clear(screen::Clear::CurrentLn).unwrap();
                #[cfg(windows)] {
                    self.goto(0, self.pos().1);
                }
            }
            "cursorup" => {
                screen::clear(screen::Clear::CursorUp).unwrap();
            }
            "cursordn" => {
                screen::clear(screen::Clear::CursorDn).unwrap();
            }
            "newln" => {
                screen::clear(screen::Clear::NewLn).unwrap();
                #[cfg(windows)] {
                    let p = self.pos();
                    self.goto(p.0, p.1);
                }
            }
            _ => ()
        }
    }

    fn size(&self) -> (u16, u16) {
        screen::size()
    }

    fn resize(&self, width: u16, height: u16) {
        screen::resize(width, height).unwrap();
    }

    fn switch(&mut self) {
        if self.id == 0 {
            let m = Metadata {
                termios: self.original_mode.clone(),
                is_raw: false,
                #[cfg(windows)]
                marker: None,
            };
            self.meta.push(m);
            self.id += 1;
            
            #[cfg(unix)] {
                screen::enable_alt().unwrap();
            }

            #[cfg(windows)] {
                // Save the current mode before switching
                self.meta[0].termios.update_mode();
                match &self.altscrn {
                    Some(screen) => {
                        screen.set_mode(
                            &self.original_mode.mode).unwrap();
                        screen.show().unwrap();
                    }
                    None => {
                        self.altscrn = Some(
                            shared::Handle::buffer().unwrap());
                        if let Some(screen) = &self.altscrn {
                            screen.set_mode(
                                &self.original_mode.mode).unwrap();
                            screen.show().unwrap();
                        }
                    },
                }                
            }
        }
    }

    fn main(&mut self) {
        #[cfg(windows)] {
            self.meta[self.id].termios.update_mode();
        }
        screen::disable_alt().unwrap();
        self.id = 0;
        let _ = &self.meta[0];
        // (imdaveho) TODO: implement a load buffer method...
    }

    fn switch_to(&mut self, id: usize) {
        #[cfg(windows)] {
            self.meta[self.id].termios.update_mode();
        }
        self.id = id;
        let _ = &self.meta[id];
        // (imdaveho) TODO: implement a load buffer method...
    }

    // (imdaveho) NOTE: removing scroll_up and scroll_down as
    // the unix native implementations are not really usable --
    // will consider adding it back in, if necessary.

    fn goto(&self, col: i16, row: i16) {
        cursor::goto(col, row).unwrap();
    }

    fn up(&self) {
        cursor::move_up(1).unwrap();
    }

    fn dn(&self) {
        cursor::move_down(1).unwrap();
    }

    fn left(&self) {
        cursor::move_left(1).unwrap();
    }

    fn right(&self) {
        cursor::move_right(1).unwrap();
    }

    fn dpad(&self, dir: &str, n: i16) {
        match dir {
            "up" => cursor::move_up(n).unwrap(),
            "dn" => cursor::move_down(n).unwrap(),
            "left" => cursor::move_left(n).unwrap(),
            "right" => cursor::move_right(n).unwrap(),
            _ => (),
        }
    }

    fn raw(&mut self) {
        output::enable_raw().unwrap();
        self.meta[self.id].is_raw = true;
    }

    fn cook(&mut self) {
        // "cooked" vs "raw" mode terminology from Wikipedia:
        // https://en.wikipedia.org/wiki/Terminal_mode
        // A terminal mode is one of a set of possible states of a
        // terminal or pseudo terminal character device in Unix-like
        // systems and determines how characters written to the terminal
        // are interpreted. In cooked mode data is preprocessed before
        // being given to a program, while raw mode passes the data as-is
        // to the program without interpreting any of the special characters.
        if self.meta[self.id].is_raw {
            #[cfg(unix)] {
                output::set_mode
                    (&self.meta[self.id].mode)
                    .unwrap();
            }

            #[cfg(windows)] {
                output::disable_raw().unwrap();
            }
            self.meta[self.id].is_raw = false;
        }
    }

    fn pos(&self) -> (i16, i16) {
        #[cfg(unix)] {
            if self.meta[self.id].is_raw {
                cursor::pos_raw().unwrap()
            } else {
                // unix needs to be raw to use pos()
                self.raw();
                let (col, row) = cursor::pos_raw().unwrap();
                // since the output was not in raw_mode before
                // we need to revert back to the cooked state
                self.cook();
                return (col, row);
            }
        }

        #[cfg(windows)] {
            cursor::pos().unwrap()
        }
    }

    #[cfg(unix)]
    fn mark(&self) {
        cursor::save_pos().unwrap()
    }

    #[cfg(unix)]
    fn load(&self) {
        cursor::load_pos().unwrap();
    }

    #[cfg(windows)]
    fn mark(&mut self) {
        self.meta[self.id].marker = Some(cursor::pos().unwrap());
    }

    #[cfg(windows)]
    fn load(&self) {
        let meta = &self.meta[self.id];
        let (col, row) = meta.marker.unwrap();
        self.goto(col, row);
    }

    fn hide_cursor(&self) {
        cursor::hide().unwrap();
    }

    fn show_cursor(&self) {
        cursor::show().unwrap();
    }

    fn read_char() {()}
    fn read_line() {()}
    fn read_async() {()}
    fn read_sync() {()}
    fn read_until_async() {()}
    fn enable_mouse_input() {()}
    fn disable_mouse_input() {()}
}
