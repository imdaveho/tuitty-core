extern crate tuitty;
use tuitty::common::unicode::wcwidth::UnicodeWidthChar;
use tuitty::common::unicode::grapheme::UnicodeGraphemes;

// use std::{ thread, time::Duration };

// #[cfg(unix)]
// use tuitty::terminal::actions::posix;

// #[cfg(windows)]
// use tuitty::terminal::actions::win32;


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

    // #[cfg(unix)] {
    //     let initial = posix::get_mode();

    //     posix::enable_alt();
    //     posix::raw();

    //     posix::goto(0, 0);
    //     posix::printf("He㓘o, क्‍ष");
    //     posix::goto(2, 0);
    //     posix::flush();
    //     thread::sleep(Duration::from_secs(2));
    //     posix::goto(7, 0);
    //     posix::flush();
    //     thread::sleep(Duration::from_secs(2));

    //     posix::cook(&initial);
    //     posix::disable_alt();
    //     thread::sleep(Duration::from_secs(1));
    // }

    let compound_emojis = ["👦🏿", "👩‍🔬", "👨‍👩‍👦", "👪", "👪🏽", "👨🏽‍👩🏽‍👧🏽"];
    let zero_width_joiner = "\u{200d}";
    // let virama_modifier = "् \u{94d}";
    let basic_escapes = ["\t", "\0", "\n", "\r", "\r\n"];
    let ascii_cjk_mix = "He㓘o";
    let wide_symbol = "。。。";

    let emojis = format!("{}{}{}{}{}{}",
                         compound_emojis[0],
                         compound_emojis[1],
                         compound_emojis[2],
                         compound_emojis[3],
                         compound_emojis[4],
                         compound_emojis[5]);
    let zwjs = format!("{}{}{}",
                       zero_width_joiner,
                       zero_width_joiner,
                       zero_width_joiner);
    let modified_ka = "\u{915}\u{94d}";
    let devanagari = "क्‍ष";
    let devanagari_manual = "\u{915}\u{94d}\u{200d}\u{937}";
    let esc_chars = format!("{}{}{}{}{}",
                            basic_escapes[0],
                            basic_escapes[1],
                            basic_escapes[2],
                            basic_escapes[3],
                            basic_escapes[4]);

    let string = format!("{} {} {} {} {} {} {} {}",
                         emojis,
                         zwjs,
                         modified_ka,
                         devanagari,
                         devanagari_manual,
                         esc_chars,
                         ascii_cjk_mix,
                         wide_symbol);

    let mut graphemes = UnicodeGraphemes
        ::graphemes(string.as_str(), true);

    while let Some(s) = graphemes.next() {
        let mut chars = s.chars().peekable();
        if let Some(car) = chars.next() { match chars.peek() {
            // A single grapheme - can be ascii, cjk, or escape seq:
            // .width() returns the character's displayed
            // width in columns, or `None` if the character
            // is a control character other than `'\x00'`.
            None => match car.width() {
                // Ascii or CJK
                Some(w) => match w {
                    0 => continue,
                    1 => {
                        if car == ' ' { println!("Content::Blank") }
                        else { println!("Content::Single({})", car) }
                    },
                    2 => println!("Content::Double({})", car),
                    _ => println!("Content::Unsupported"),
                },
                // Escape character
                None => match car {
                    '\t' => println!("Content::Tab"),
                    '\n' => println!("Content::LF"),
                    '\r' => println!("Content::CR"),
                    _ => unreachable!(),
                }
            },
            // A complex grapheme - can be emoji, CRLF, or language:
            Some(cadr) => match (car, cadr) {
                ('\r', '\n') => println!("Content::CRLF"),
                _ => {
                    let mut width = car.width().unwrap_or(0);
                    let mut content = vec![car];
                    loop {
                        if let Some(next) = chars.next() {
                            // Continue iterating through grapheme cluster:
                            content.push(next);
                            width += next.width().unwrap_or(0);
                            let fitzpatrick = [
                                '\u{1f3fb}',
                                '\u{1f3fc}',
                                '\u{1f3fd}',
                                '\u{1f3fe}',
                                '\u{1f3ff}'];
                            if fitzpatrick.contains(content.last().unwrap()) {
                                println!("FITZ");
                            }
                        } else {
                            // End of grapheme - check if there is a joiner:
                            match content.last().unwrap() {
                                '\u{200d}' => if let Some(s) = graphemes
                                    .next() {
                                        chars = s.chars().peekable();
                                        continue;
                                    },
                                _ => break,
                            }
                        }
                    }

                    let zwj_enabled = false;
                    if zwj_enabled {
                        println!("Content::Complex({:?} | width: 2", content);
                        println!("Content::Link(L: -1, R: +1)");
                    } else {
                        println!("Content::Complex({:?}) | width: {}", content, width);
                        for i in 0..width {
                            println!("Content::Link(L: -{}, R: {})",
                                     i + 1, width - i);
                        }
                    }
                }
            }
        }}
    }
    println!("output: \"{}\"", string);
}
