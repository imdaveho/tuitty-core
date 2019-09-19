// extern crate tuitty;

use std::thread;
use std::time::Duration;
mod terminal;
mod common;
use common::{
    traits::{
        TerminalCursor, TerminalFormatter, TerminalInput,
        TerminalModifier, TerminalSwitcher, TerminalWriter
    }, enums::{ Color, Effect },
};

use std::io::{ stdin, stdout, Result, BufRead, Write };


fn pos_raw() -> Result<(i16, i16)> {
    // Where is the cursor?
    // Use `ESC [ 6 n`.
    let mut stdout = stdout();
    let stdin = stdin();

    // Write command
    stdout.write_all(b"\x1B[6n")?;
    stdout.flush()?;

    stdin.lock().read_until(b'[', &mut vec![])?;

    let mut rows = vec![];
    stdin.lock().read_until(b';', &mut rows).unwrap();

    let mut cols = vec![];
    stdin.lock().read_until(b'R', &mut cols).unwrap();

    // remove delimiter
    rows.pop();
    cols.pop();

    let rows = rows
        .into_iter()
        .map(|b| (b as char))
        .fold(String::new(), |mut acc, n| {
            acc.push(n);
            acc
        })
        .parse::<usize>()
        .unwrap();
    let cols = cols
        .into_iter()
        .map(|b| (b as char))
        .fold(String::new(), |mut acc, n| {
            acc.push(n);
            acc
        })
        .parse::<usize>()
        .unwrap();

    Ok(((cols - 1) as i16, (rows - 1) as i16))
}

fn main() {

    let mut t = terminal::Terminal::init();
    t.switch();

    t.printf("Hello, World");
    thread::sleep(Duration::from_millis(1500));
    t.goto(5, 6);
    t.flush();
    thread::sleep(Duration::from_millis(1500));
    t.set_fg(Color::Blue);
    t.set_fx(Effect::Dim | Effect::Underline);
    t.printf("Hello, Again");
    t.reset_styles();
    thread::sleep(Duration::from_millis(1500));

    t.switch();
    t.printf("Did the cursor reset?");
    thread::sleep(Duration::from_millis(1500));
    t.goto(8, 10);
    t.flush();
    thread::sleep(Duration::from_millis(1500));
    t.printf("Where am I now?");
    thread::sleep(Duration::from_millis(1500));
    t.switch_to(1);
    thread::sleep(Duration::from_millis(2500));

    // let mut t = terminal::Terminal::init();
    // t.raw();
    // let (col, row) = pos_raw().unwrap();
    // t.cook();
    // t.switch();
    // t.goto(5, 5);
    // t.set_fg(Color::Yellow);
    // t.printf(&format!("{}, {}", col, row));
    // thread::sleep(Duration::from_millis(2000));

    // let mut t = tty::Tty::init();
    // let a = t.is_ansi();
    // t.printf(&format!("\n{}\n", a));
    // let mut buf = tty::shared::CellBuffer::new();
    // let (w, h) = buf._screen_size();
    // t.printf(&format!("{}, {}", w, h));

    // t.switch();

    // t.goto(84, 29);
    // t.prints("1");
    // t.flush();
    // thread::sleep(Duration::from_millis(1000));
    // t.goto(85, 29);
    // t.prints("23");
    // t.flush();
    // thread::sleep(Duration::from_millis(1000));


    // let chars = "hello, world".chars();
    // let length = "hello, world".len();
    // let here = ((2 * w) + 5) as usize;
    // let there = here + length;

    // let mut iteration = 0;
    // for ch in chars {
    //     buf.cells[here + iteration] = Some(tty::shared::Cell {
    //         rune: ch,
    //         width: 0,
    //         style: tty::shared::CellStyle::new(),
    //     });
    //     iteration += 1;
    // }

    // t.goto(0, 0);

    // let mut content = String::new();
    // // let mut spaces = 0;
    // for cell in buf.cells {
    //     match cell {
    //         Some(c) => {
    //             //t.prints(&format!("{}", c.rune));
    //             content.push(c.rune);
    //             // if c.rune == 'd' {
    //             //     break
    //             // }
    //         },
    //         None => {
    //             // t.prints(" ")
    //             content.push(' ');
    //             // spaces += 1;
    //         }
    //     }
    //     // thread::sleep(Duration::from_millis(80));
    // }

    // t.prints(&content);
    // // t.goto(0, 5);
    // // t.prints(&format!("{}", spaces));

    // thread::sleep(Duration::from_millis(2000));



    // // TESTING WINCON SCREEN-CACHE

    // use winapi::um::{
    //     wincon::{
    //         SetConsoleTextAttribute,
    //         WriteConsoleOutputW, ReadConsoleOutputW,
    //         COORD, CHAR_INFO, SMALL_RECT,
    //         FOREGROUND_BLUE as BLUE,
    //         FOREGROUND_INTENSITY as INTENSE,
    //         FillConsoleOutputCharacterA, SetConsoleCursorPosition,
    //     },
    //     // consoleapi::{WriteConsoleA, WriteConsoleW},
    //     consoleapi::WriteConsoleW,
    // };
    // use std::mem::zeroed;
    // use winapi::shared::ntdef::{NULL, VOID};

    // let stdout = tty::Handle::conout().unwrap();
    // // stdout.show().unwrap();

    // let sA = tty::Handle::buffer().unwrap();
    // sA.show().unwrap();

    // // MOVE =====================================================================

    // let sA_pos = COORD { X: 0, Y: 5 };

    // unsafe {
    //     if SetConsoleCursorPosition(sA.0, sA_pos) == 0 {
    //         panic!("Error setting sA_pos")
    //     }
    // }

    // let sA_words = "A good choice of font for your coding can make a huge difference and improve your productivity, \
    // so take a look at the fonts in this post that can make your text editor or terminal emulator look little bit nicer. \
    // Andale® Mono — is a monospaced sans-serif typeface designed by Steve Matteson for terminal emulation and software \
    // development environments, originally for the Taligent project by Apple Inc. and IBM. The Andalé design was originally \
    // created by Monotype, as part of Andalé font families. Aperçu — Aperçu was started in December 2009, and has been \
    // trialled and tested through a number of design comissions taken on by The Entente through 2010. The conceit behind Aperçu \
    // was to create a synopsis or amalgamation of classic realist typefaces: Johnston, Gill Sans, Neuzeit & Franklin Gothic.";

    // let sA_chars = sA_words.encode_utf16().map(|x| x).collect::<Vec<u16>>();
    // // let words_ptr = chars;
    // // let length = words_ptr.len() as u32;
    // let mut sA_size = 0;
    // // let currout = tuitty::Handle::conout().unwrap();

    // // PRINT SCREEN A =====================================================================

    // unsafe {
    //         // https://docs.microsoft.com/en-us/windows/console/writeconsole
    //         if WriteConsoleW(
    //             sA.0,
    //             sA_chars.as_ptr() as *const VOID,
    //             sA_chars.len() as u32,
    //             &mut sA_size, NULL
    //         ) == 0 {
    //             panic!("Something went wrong writing in altern")
    //         }
    // }

    // // CACHE INTO CELLBUF =====================================================================

    // let mut sA_cellbuf: Vec<CHAR_INFO> = unsafe {vec![zeroed(); 86 * 30]};
    // let sA_length = sA_cellbuf.len();
    // // let sA_dimens = (86, 30);
    // // let sA_rows = 30;
    // let sA_buf_size = COORD {X: 86, Y: 30};
    // let sA_buf_coord = COORD {X: 0, Y: 0};
    // let mut dest_rect = SMALL_RECT {
    //     Top: 0,
    //     Left: 0,
    //     Bottom: 30,
    //     Right: 86,
    // };

    // unsafe {
    //     if ReadConsoleOutputW(
    //         sA.0,
    //         sA_cellbuf.as_mut_ptr(),
    //         sA_buf_size, sA_buf_coord,
    //         &mut dest_rect
    //     ) == 0 {
    //         panic!("Something went wrong writing in altern")
    //     }

    // }

    // thread::sleep(Duration::from_millis(2000));

    // // "SWITCH" TO SCREEN B =====================================================================

    // // Tty does not create another handle but clears the current one:
    // // let sB = tty::Handle::buffer().unwrap();
    // // sB.show().unwrap();

    // // CLEAR =====================================================================

    // let mut written = 0;
    // let clr_coord = COORD { X: 0, Y: 0, };
    // unsafe {
    //     if FillConsoleOutputCharacterA(
    //         sA.0, ' ' as i8, (86 * 30), clr_coord, &mut written) == 0 {
    //         panic!("Problem clearing!")
    //     }
    // }

    // thread::sleep(Duration::from_millis(2000));


    // // MOVE =====================================================================

    // let sB_pos = COORD { X: 0, Y: 5 };

    // unsafe {
    //     if SetConsoleCursorPosition(sA.0, sB_pos) == 0 {
    //         panic!("Error setting sA_pos")
    //     }
    // }

    // // https://docs.microsoft.com/en-us/windows/console/writeconsoleoutput
    // // https://docs.microsoft.com/en-us/windows/console/reading-and-writing-blocks-of-characters-and-attributes
    // let sB_words = "something after in altern; with widechar: 𝕊 🗻 ∈ 🌏";
    // let sB_chars = sB_words.encode_utf16().map(|x| x).collect::<Vec<u16>>();
    // let mut sB_size = 0;

    // // PRINT SCREEN B =====================================================================

    // unsafe {
    //         // https://docs.microsoft.com/en-us/windows/console/writeconsole
    //         if WriteConsoleW(
    //             sA.0,
    //             sB_chars.as_ptr() as *const VOID,
    //             sB_chars.len() as u32,
    //             &mut sB_size, NULL
    //         ) == 0 {
    //             panic!("Something went wrong writing in altern")
    //         }
    // }

    // thread::sleep(Duration::from_millis(2000));

    // // SWITCH TO SCREEN A =====================================================================

    // // Tty does not create another handle but clears the current one,
    // // therefore we will "restore" it with WriteConsoleOutputW
    // // sA.show().unwrap();

    // // CLEAR =====================================================================

    // unsafe {
    //     if FillConsoleOutputCharacterA(
    //         sA.0, ' ' as i8, (86 * 30), clr_coord, &mut written) == 0 {
    //         panic!("Problem clearing!")
    //     }
    // }

    // thread::sleep(Duration::from_millis(2000));

    // // RESTORE =====================================================================

    // unsafe {
    //     if WriteConsoleOutputW(
    //         sA.0,
    //         sA_cellbuf.as_ptr(),
    //         sA_buf_size, sA_buf_coord,
    //         &mut dest_rect
    //     ) == 0 {
    //         panic!("Something went wrong writing in altern")
    //     }
    // }

    // // // let words = "something after in altern; with widechar: 𝕊 🗻∈🌏".as_bytes();
    // // let sB_cellbuf = sB_words
    // // //     // .iter()
    // //     .map(|ch| unsafe {
    // //         let mut char_info: CHAR_INFO = zeroed();
    // //         char_info.Attributes = INTENSE; // BLUE | INTENSE;
    // //         *char_info.Char.UnicodeChar_mut() = ch;
    // //         // *char_info.Char.UnicodeChar_mut() = *ch as u16;
    // //         char_info
    // //     }).collect::<Vec<CHAR_INFO>>();

    // // // let length = char_info_buffer.len();
    // // // let bsize = (86, 30);
    // // // let rows = length as i16 / bsize.0 + 1;

    // // // println!("{}", rows);

    // // // this informs how much of the pointer the function needs to traverse
    // // let buf_size = COORD {X: length as i16, Y: rows};
    // // let buf_cord = COORD {X: 0, Y: 0};
    // // let mut dest_rect = SMALL_RECT {
    // //     Top: 0,
    // //     Left: 0,
    // //     Bottom: bsize.1,
    // //     Right: bsize.0,
    // // };

    // // // let mut t = tty::Tty::init();
    // // // t.switch();
    // // // t.flush();

    // // unsafe {
    // //     if WriteConsoleOutputW(
    // //         sB.0,
    // //         char_info_buffer.as_ptr(),
    // //         buf_size, buf_cord,
    // //         &mut dest_rect
    // //     ) == 0 {
    // //         panic!("Something went wrong writing in altern")
    // //     }

    // // }
    // thread::sleep(Duration::from_millis(2000));

    // // let stdout = tty::Handle::stdout().unwrap();
    // stdout.show().unwrap();

    // sA.close().unwrap();
    // // Tty does not create another handle but clears the current one:
    // // sB.close().unwrap();

    // // // // if size == length {
    // // // //     println!("write success!");
    // // // // }
    // // END SCREEN-CACHE WINDOWS

    // let mut t = tty::Tty::init();
    // let pos = tty::cursor::ansi::pos_raw();

    // println!("{:?}", pos);

    // thread::sleep(Duration::from_millis(1000));
}
