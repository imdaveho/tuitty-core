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


    let mut tty = tuitty::tty::Tty::init();
    tty.write(&format!("{}\n", tty.original_mode));

    tty.write(&format!{"{}{}\n", tty.size().0, tty.size().1});

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
    
    tty.write(&format!("\n\n\n{} ", tty.meta[tty.id].is_raw_enabled));
    tty.write(&format!("\n{} ", tty.meta[tty.id].is_mouse_enabled));

    tty.flush();

    use std::time::Duration;
    use std::thread;
    thread::sleep(Duration::from_secs(3));

    tty.main();

    tty.write(&format!("\n\n\n{} ", tty.meta[tty.id].is_raw_enabled));
    tty.write(&format!("\n{} ", tty.meta[tty.id].is_mouse_enabled));

    tty.write("Hello (r), "); // since fg red was on the altern screen, the main screen is still white
    tty.set_fg("darkblue");
    tty.write("Hello (db), ");
    tty.reset();
    tty.write("End\n");
    tty.flush();
    thread::sleep(Duration::from_secs(2));

    tty.exit();
    thread::sleep(Duration::from_secs(2));

    // let handle = tuitty::Handle::stdout().unwrap();
    // let info = tuitty::ConsoleInfo::of(&handle).unwrap();
    // let attrs = info.attributes();
    
    // // // println!("attributes: {}", attrs);

    // // let blue = 0x0001;
    // let green = 0x0002;
    // let red = 0x0004;
    // let intense = 0x0008;

    // // // let lead_byte = 0x0100;
    // // // let tral_byte = 0x0200;
    // // // let top_horiz = 0x0400;
    // // // let left_vert = 0x0800;
    // // // let right_vert = 0x1000;
    // // // let reverse = 0x4000;
    // // let underln = 0x8000;

    // // let white = red | green | blue;

    // // let color = white;

    // unsafe {
    //     if SetConsoleTextAttribute(handle.0, green) == 0 {
    //         // return Err(TtyErrorKind::IoError(Error::last_os_error()));
    //         panic!("Something went wrong with setting the text attribute")
    //     }
    // }

    // println!("Hello");

    // // unsafe {
    // //     if SetConsoleTextAttribute(handle.0, attrs & !intense) == 0 {
    // //         // return Err(TtyErrorKind::IoError(Error::last_os_error()));
    // //         panic!("Something went wrong with setting the text attribute")
    // //     }
    // // }

    // // println!("Hello");



    // // println!("something after");
    // // println!("setting the attr");

    // // RESET
    // unsafe {
    //     if SetConsoleTextAttribute(handle.0, attrs) == 0 {
    //         // return Err(TtyErrorKind::IoError(Error::last_os_error()));
    //         panic!("Something went wrong with setting the text attribute")
    //     }
    // }

    // // println!("Hello");


    // // // SWITCH TO ALT SCREEN
    // let altern = tuitty::Handle::buffer().unwrap();
    // tuitty::clear("newln");

    // altern.show().unwrap();

    // tuitty::clear("all");

    // unsafe {
    //     if SetConsoleTextAttribute(altern.0, red|intense) == 0 {
    //         // return Err(TtyErrorKind::IoError(Error::last_os_error()));
    //         panic!("Something went wrong with setting the text attribute")
    //     }
    // }
    // // // https://docs.microsoft.com/en-us/windows/console/writeconsoleoutput
    // // // https://docs.microsoft.com/en-us/windows/console/reading-and-writing-blocks-of-characters-and-attributes
    // // let words = "something after in altern; with widechar: 𝕊 🗻 ∈ 🌏".encode_utf16();
    // // // let words = "something after in altern; with widechar: 𝕊 🗻∈🌏".as_bytes();
    // // let char_info_buffer = words
    // //     // .iter()
    // //     .map(|ch| unsafe {
    // //         let mut char_info: CHAR_INFO = zeroed();
    // //         char_info.Attributes = blue | intense;
    // //         // *char_info.Char.UnicodeChar_mut() = *ch as u16;
    // //         *char_info.Char.UnicodeChar_mut() = ch;
    // //         char_info
    // //     }).collect::<Vec<CHAR_INFO>>();
    
    // // let length = char_info_buffer.len();
    // // let bsize = tuitty::size();
    // // let rows = length as i16 / bsize.0 + 1;
    
    // // // // println!("{}", rows);

    // // // // this informs how much of the pointer the function needs to traverse
    // // let buf_size = COORD {X: length as i16, Y: rows}; 
    // // let buf_cord = COORD {X: 0, Y: 0};
    // // let mut dest_rect = SMALL_RECT {
    // //     Top: 0,
    // //     Left: 0,
    // //     Bottom: bsize.1,
    // //     Right: bsize.0,
    // // };
    
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

    // //                 // unsafe {
    // //                 //     if SetConsoleTextAttribute(handle.0, red) == 0 {
    // //                 //         // return Err(TtyErrorKind::IoError(Error::last_os_error()));
    // //                 //         panic!("Something went wrong with setting the text attribute")
    // //                 //     }
    // //                 // }

    // //                 // use std::io::Write;

    // //                 // let stdout = ::std::io::stdout();
    // //                 // let mut stdout = stdout.lock();

    // //                 // // stdout.flush().unwrap();
    // //                 // stdout.write("\x1B[?1049h".as_bytes()).unwrap();
    // //                 // // tuitty::clear("all");
    // //                 // stdout.write("\x1B[2J".as_bytes()).unwrap();
    // //                 // stdout.flush().unwrap();

    // //                 // stdout.write("something after alt\0".as_bytes()).unwrap();
    // //                 // stdout.write("setting the attr alt\n".as_bytes()).unwrap();

    // //                 // stdout.flush().unwrap();

    // //                 // println!("something after");
    // //                 // println!("setting the attr");

    // //                 // stdout.write("\x1B[?1049l".as_bytes()).unwrap();


    // use std::time::Duration;
    // use std::thread;

    // thread::sleep(Duration::from_secs(3));
    // handle.show().unwrap();
    // altern.close().unwrap();


    
    // // RESET
    // unsafe {
    //     if SetConsoleTextAttribute(handle.0, attrs) == 0 {
    //         // return Err(TtyErrorKind::IoError(Error::last_os_error()));
    //         panic!("Something went wrong with setting the text attribute")
    //     }
    // }

    // // use std::io::Write;

    // // let stdout = ::std::io::stdout();
    // // let mut stdout = stdout.lock();

    // // stdout.write("\x1B[38;5;1m\x1B[39;mHello,\x1B[0m \x1B[38;5;10;1mHello, \x1B[38;5;10;2mHello, \x1B[38;5;10;4mHello \x1B[0m\n".as_bytes()).unwrap();
    // // stdout.flush().unwrap();

}
