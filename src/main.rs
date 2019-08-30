extern crate tuitty;
// use winapi::um::{
//     wincon::{
//         SetConsoleTextAttribute,
//         WriteConsoleOutputW,
//         COORD, CHAR_INFO, SMALL_RECT,
//     },
//     consoleapi::{WriteConsoleA, WriteConsoleW},
// };

// use winapi::um::wincontypes::CHAR_INFO_Char;
// use std::mem::zeroed;
// use winapi::shared::ntdef::{NULL, VOID};

fn main() {
    // println!("{}", std::env::var("MSYSTEM").unwrap());
    // let mut tty = tuitty::tty::Tty::trial();
    // println!("{}", tty);


    let mut tty = tuitty::Tty::init();
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

    tty.main();

    tty.write("Hello (r), "); // since fg red was on the altern screen, the main screen is still white
    tty.set_fg("darkblue");
    tty.write("Hello (db), ");
    tty.reset();
    tty.write("End\n");
    tty.flush();
    thread::sleep(Duration::from_secs(2));

    tty.exit();
    thread::sleep(Duration::from_secs(2));

    // https://docs.microsoft.com/en-us/windows/console/writeconsoleoutput
    // https://docs.microsoft.com/en-us/windows/console/reading-and-writing-blocks-of-characters-and-attributes
    // let words = "something after in altern; with widechar: 𝕊 🗻 ∈ 🌏".encode_utf16();
    // let words = "something after in altern; with widechar: 𝕊 🗻∈🌏".as_bytes();
    // let char_info_buffer = words
    //     // .iter()
    //     .map(|ch| unsafe {
    //         let mut char_info: CHAR_INFO = zeroed();
    //         char_info.Attributes = blue | intense;
    //         // *char_info.Char.UnicodeChar_mut() = *ch as u16;
    //         *char_info.Char.UnicodeChar_mut() = ch;
    //         char_info
    //     }).collect::<Vec<CHAR_INFO>>();

    // let length = char_info_buffer.len();
    // let bsize = tuitty::size();
    // let rows = length as i16 / bsize.0 + 1;

    // println!("{}", rows);

    // this informs how much of the pointer the function needs to traverse
    // let buf_size = COORD {X: length as i16, Y: rows};
    // let buf_cord = COORD {X: 0, Y: 0};
    // let mut dest_rect = SMALL_RECT {
    //     Top: 0,
    //     Left: 0,
    //     Bottom: bsize.1,
    //     Right: bsize.0,
    // };

    // let words = "A good choice of font for your coding can make a huge difference and improve your productivity, \
    // so take a look at the fonts in this post that can make your text editor or terminal emulator look little bit nicer. \
    // Andale® Mono — is a monospaced sans-serif typeface designed by Steve Matteson for terminal emulation and software \
    // development environments, originally for the Taligent project by Apple Inc. and IBM. The Andalé design was originally \
    // created by Monotype, as part of Andalé font families. Aperçu — Aperçu was started in December 2009, and has been \
    // trialled and tested through a number of design comissions taken on by The Entente through 2010. The conceit behind Aperçu \
    // was to create a synopsis or amalgamation of classic realist typefaces: Johnston, Gill Sans, Neuzeit & Franklin Gothic.";

    // let chars = words.encode_utf16().map(|x| x).collect::<Vec<u16>>();
    // let words_ptr = chars;
    // let length = words_ptr.len() as u32;
    // let mut size = 0;
    // let currout = tuitty::Handle::conout().unwrap();

    // unsafe {
    //     // https://docs.microsoft.com/en-us/windows/console/writeconsole
    //     if WriteConsoleW(
    //         currout.0,
    //         words_ptr.as_ptr() as *const VOID,
    //         length,
    //         &mut size, NULL
    //     ) == 0 {
    //         panic!("Something went wrong writing in altern")
    //     }

    //     // if WriteConsoleOutputW(
    //     //     altern.0,
    //     //     char_info_buffer.as_ptr(),
    //     //     buf_size, buf_cord,
    //     //     &mut dest_rect
    //     // ) == 0 {
    //     //     panic!("Something went wrong writing in altern")
    //     // }

    // }

    // // // if size == length {
    // // //     println!("write success!");
    // // // }
}
