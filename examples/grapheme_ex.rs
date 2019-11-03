extern crate tuitty;
use tuitty::common::unicode::grapheme::UnicodeGraphemes;
use tuitty::common::unicode::wcwidth::UnicodeWidthStr;

use std::{ thread, time::Duration };

#[cfg(unix)]
use tuitty::terminal::actions::posix;

#[cfg(windows)]
use tuitty::terminal::actions::win32;


fn main() {
    // let c = "क्‍ष 👪 👨👩‍👧‍👦 🤦♀";
    // println!("{}", c.width());
    // let clusters = c.graphemes(true).collect::<Vec<&str>>();
    // for c in clusters {
    //     println!("{:?}: {:?}", c, c.width());
    // }
    // let c = 'क';
    // let d = '्';
    // let e = 'ष';

    // println!("h{}{}{}h", c, d, e);

    // let fp = "|🤦‍♀️|";
    // println!("{}", fp);

    // let content = "\x1B\t\r\n";
    // let clusters = content.graphemes(true).collect::<Vec<&str>>();
    // println!("{:?}", clusters);
    // for n in clusters {
    //     println!("{}", n.is_ascii());
    // }

    #[cfg(unix)] {
        let initial = posix::get_mode();

        posix::enable_alt();
        posix::raw();

        posix::goto(0, 0);
        posix::printf("He㓘o, क्‍ष");
        posix::goto(2, 0);
        posix::flush();
        thread::sleep(Duration::from_secs(2));
        posix::goto(7, 0);
        posix::flush();
        thread::sleep(Duration::from_secs(2));

        posix::cook(&initial);
        posix::disable_alt();
        thread::sleep(Duration::from_secs(1));
    }


}
