//! The `tty` module wraps the various components that make up a terminal. These
//! are represented by the sub-modules: `cursor`, `screen`, `input`, `output`.
//! The `Tty` struct is meant to be a thin abstraction to standardize between
//! operating systems and APIs (ANSI vs Windows Console).

#[cfg(unix)]
use libc::termios as Termios;

#[cfg(windows)]
type Termios = u32;

#[cfg(windows)]
use shared::{Handle, ConsoleInfo};

mod cursor;
mod input;
mod output;
mod screen;
mod shared;

pub use input::{InputEvent, KeyEvent, MouseEvent, MouseButton};

#[cfg(unix)]
pub use input::ansi::{AsyncReader, SyncReader};

#[cfg(windows)]
pub use input::wincon::{AsyncReader, SyncReader};

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use unix::Tty;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::Tty;


#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        let mut tty = super::Tty::init();

        tty.write(&format!{"w: {}, h: {}\n", tty.size().0, tty.size().1});

        tty.set_fg("green");
        tty.write("Hello (g), ");
        tty.reset();
        tty.write("Hello (d), ");
        tty.flush();
        tty.switch();
        tty.clear("all");

        tty.raw();
        tty.enable_mouse();

        let words = "A good choice of font for your coding can make a huge difference and improve your productivity, \
                     so take a look at the fonts in this post that can make your text editor or terminal emulator look little bit nicer. \
                     Andale® Mono — is a monospaced sans-serif typeface designed by Steve Matteson for terminal emulation and software \
                     development environments, originally for the Taligent project by Apple Inc. and IBM. The Andalé design was originally \
                     created by Monotype, as part of Andalé font families. Aperçu — Aperçu was started in December 2009, and has been \
                     trialled and tested through a number of design comissions taken on by The Entente through 2010. The conceit behind Aperçu \
                     was to create a synopsis or amalgamation of classic realist typefaces: Johnston, Gill Sans, Neuzeit & Franklin Gothic.";

        tty.set_fg("red"); // this sets fg on altern screen
        tty.write(words);
        tty.flush();

        use std::time::Duration;
        use std::thread;
        thread::sleep(Duration::from_secs(3));

        tty.to_main();

        tty.write("Hello (r), "); // since fg red was on the altern screen, the main screen is still white
        tty.set_fg("darkblue");
        tty.write("Hello (db), ");
        tty.reset();
        tty.write("End\n");
        tty.flush();
        thread::sleep(Duration::from_secs(2));

        tty.exit();
        thread::sleep(Duration::from_secs(2));
    }
}
