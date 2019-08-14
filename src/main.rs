extern crate tuitty;

// use std::{thread, time};
// use std::io::{stdout, Write};


fn main() {
    let size = tuitty::screen::size();
    println!("{}, {}", size.0, size.1);





    // let o = stdout();
    // let mut o = o.lock();
    // let t = "this is a long str that I believe wraps around but can't really determine if that is really the case or not\0";
    // o.write(t.as_bytes());
    // o.flush();
    // let delay = time::Duration::from_millis(3000);
    // thread::sleep(delay);





    // tuitty::screen::clear(tuitty::screen::ClearStyle::All).unwrap();





    // let delay = time::Duration::from_secs(1);

    // tuitty::output::linux::_set_fg(tuitty::output::Color::from("magenta")).unwrap();
    // print!("Magenta, ");
    // stdout().flush().unwrap();

    // thread::sleep(delay);

    // tuitty::output::linux::_set_fg(tuitty::output::Color::Yellow).unwrap();
    // print!("Yellow, ");
    // stdout().flush().unwrap();

    // thread::sleep(delay);

    // // tuitty::output::linux::_set_fg(tuitty::output::Color::Reset).unwrap();
    // // print!("Reset\n");
    // // stdout().flush().unwrap();

    // tuitty::output::linux::_set_fg(tuitty::output::Color::Reset).unwrap();
    // tuitty::output::linux::_set_attr(tuitty::output::Attribute::Underline).unwrap();
    // println!("This is after yellow was set as Fg.");




    // for col in 0..10 {
    //     for row in 0..5 {
    //         tuitty::cursor::goto(row, col).unwrap();
    //         let delay = time::Duration::from_millis(200);
    //         thread::sleep(delay);
    //     }
    // }


    // let (w, a, s, d) = (
    //     tuitty::cursor::move_up,
    //     tuitty::cursor::move_left,
    //     tuitty::cursor::move_down,
    //     tuitty::cursor::move_right
    // );

    // tuitty::screen::clear(tuitty::screen::ClearStyle::All).unwrap();

    // tuitty::cursor::goto(10, 10).unwrap();
    // thread::sleep(time::Duration::from_secs(1));

    // tuitty::cursor::save_pos().unwrap();

    // use std::time;
    // use std::thread;

    // let delay = time::Duration::from_millis(200);
    // thread::sleep(delay);

    // w(1).unwrap();
    // thread::sleep(delay);
    // d(1).unwrap();
    // thread::sleep(delay);
    // s(1).unwrap();
    // thread::sleep(delay);
    // a(1).unwrap();
    // thread::sleep(delay);

    // tuitty::cursor::hide().unwrap();
    // thread::sleep(delay);

    // use std::io::stdout;
    // use std::io::Write;

    // w(1).unwrap();
    // print!("w");
    // stdout().flush().unwrap();
    // thread::sleep(delay);
    // d(1).unwrap();
    // print!("d");
    // stdout().flush().unwrap();
    // thread::sleep(delay);
    // s(1).unwrap();
    // a(1).unwrap();
    // print!("s");
    // stdout().flush().unwrap();
    // thread::sleep(delay);
    // a(3).unwrap();
    // print!("a");
    // stdout().flush().unwrap();
    // thread::sleep(time::Duration::from_secs(1));

    // tuitty::cursor::show().unwrap();
    // thread::sleep(time::Duration::from_secs(1));
    // tuitty::cursor::load_pos().unwrap();
    // thread::sleep(time::Duration::from_secs(1));





    // use std::io::{stdout, Write};
    // use std::{time, thread};

    // // struct AltScreen;

    // // impl AltScreen {
    // //     fn new() -> ::std::io::Result<AltScreen> {
    // //         Ok(AltScreen)
    // //     }

    // //     fn enable(&self) {
    // //         tuitty::screen::enable_alt().unwrap();
    // //     }

    // //     fn disable(&self) {
    // //         tuitty::screen::disable_alt().unwrap();
    // //     }
    // // }

    // // impl Drop for AltScreen {
    // //     fn drop(&mut self) {
    // //         self.disable();
    // //     }
    // // }

    // tuitty::screen::clear(tuitty::screen::ClearStyle::CursorDown).unwrap();
    // println!("This is the main screen!");
    // thread::sleep(time::Duration::from_secs(1));

    // // if let Ok(alt) = AltScreen::new() {
    // //     alt.enable();
    // //     for i in 1..5 {
    // //         tuitty::cursor::goto(10, 2).unwrap();
    // //         thread::sleep(time::Duration::from_millis(500));
    // //         print!("{} of 5 items processed", i);
    // //         stdout().flush().unwrap();
    // //     }
    // // }

    // // {
    // tuitty::screen::enable_alt().unwrap();
    // for i in 1..5 {
    //     tuitty::cursor::goto(10, 2).unwrap();
    //     thread::sleep(time::Duration::from_millis(500));
    //     print!("{} of 5 items processed", i);
    //     stdout().flush().unwrap();
    // }
    // tuitty::screen::disable_alt().unwrap();
    // // }

    // tuitty::screen::clear(tuitty::screen::ClearStyle::CurrentLine).unwrap();
    // thread::sleep(time::Duration::from_millis(250));
    // tuitty::cursor::goto(0, 2).unwrap();
    // println!("...and you've returned!");
    // thread::sleep(time::Duration::from_secs(1));





    // use std::io::{stdout, Write};
    // use std::fmt::Display;
    // use std::{time, thread};

    // let out = stdout();
    // let mut out = out.lock();
    // // out.write("\x1Bc".as_bytes()).unwrap();

    // #[derive(Copy, Clone)]
    // pub enum CursorStyle {
    //     Reset = 0,
    //     BlinkBlock = 1,
    //     SolidBlock = 2,
    //     BlinkUnderline = 3,
    //     SolidUnderline = 4,
    //     BlinkBeam = 5,
    //     SolidBeam = 6,
    // }

    // impl Display for CursorStyle {
    //     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    //         write!(f, "{}", *self as u16)
    //     }
    // }

    // let cur_style = CursorStyle::Reset.to_string();
    // let cur_stylestr = format!("\x1B[{} q", cur_style.as_str());

    // out.write(cur_stylestr.as_bytes()).unwrap();
    // out.flush().unwrap();

    // thread::sleep(time::Duration::from_millis(1250));





    // use std::io::Write;

    // let s = String::from("\x1B[31hello\x1B[0m\n");
    // let stdout = std::io::stdout();
    // let mut stdout = stdout.lock();
    // stdout.write(s.as_bytes()).unwrap();
    // stdout.flush().unwrap();
}
