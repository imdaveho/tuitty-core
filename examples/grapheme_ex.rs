extern crate tuitty;
use tuitty::common::unicode::wcwidth::*;
use tuitty::common::unicode::grapheme::UnicodeGraphemes;

// use std::{ thread, time::Duration };

// #[cfg(unix)]
// use tuitty::terminal::actions::posix;

// #[cfg(windows)]
// use tuitty::terminal::actions::win32;

fn main() {
    // let original_input = "---- 👦🏿 ----";
    // let original_input = "---- 👨🏿‍🦰 ----";
    // let original_input = "---- ⚠️ ----";
    // let original_input= "---- ❤️ ----";
    let original_input = "---- 👨‍👩‍👧 ----";
    let graphemes = UnicodeGraphemes::graphemes(original_input, true);
    for g in graphemes {
        println!("grapheme: {:?}, size: {}, width: {}", g, std::mem::size_of_val(g), g.width());
    }

}

fn _main() {
    let compound_emojis = ["👦🏿", "👩‍🔬", "👨‍👩‍👦", "👪", "👪🏽", "👨🏽‍👩🏽‍👧🏽",
                           "👨‍🦰", "🧗🏾‍♂\u{fe0f}", "🧗🏾‍♀\u{fe0f}",
                           "🕵🏼‍♀\u{fe0f}"];
    let zero_width_joiner = "\u{200d}";
    // let virama_modifier = "् \u{94d}";
    let basic_escapes = ["\t", "\0", "\x1b]34m", "\n", "\r", "\r\n"];
    let ascii_cjk_mix = "He㓘o, \u{2764}\u{fe0e}";
    let wide_symbol = "。。。👪";

    let emojis = format!("{}{}{}{}{}{}{}{}{}{}",
                         compound_emojis[0],
                         compound_emojis[1],
                         compound_emojis[2],
                         compound_emojis[3],
                         compound_emojis[4],
                         compound_emojis[5],
                         compound_emojis[6],
                         compound_emojis[7],
                         compound_emojis[8],
                         compound_emojis[9]);
    let zwjs = format!("{}{}{}",
                       zero_width_joiner,
                       zero_width_joiner,
                       zero_width_joiner);
    let modified_ka = "\u{915}\u{94d}";
    let devanagari = "क्‍ष";
    let devanagari_manual = "\u{915}\u{94d}\u{200d}\u{937}";
    let esc_chars = format!("{}{}{}{}{}{}",
                            basic_escapes[0],
                            basic_escapes[1],
                            basic_escapes[2],
                            basic_escapes[3],
                            basic_escapes[4],
                            basic_escapes[5]);

    // let string = format!("{} {} {} {} {} {} {} {}",
    // let string = format!("{} {} {} {} {} {} {}",
    //                      emojis,
    //                      // zwjs,
    //                      modified_ka,
    //                      devanagari,
    //                      devanagari_manual,
    //                      // esc_chars,
    //                      "",
    //                      ascii_cjk_mix,
    //                      wide_symbol);
    let string = String::from("---- 👨🏿‍🦰 ----");

    let mut graphemes = UnicodeGraphemes
        ::graphemes(string.as_str(), true).peekable();

    // From: https://eng.getwisdom.io/emoji-modifiers-and-sequence-combinations/
    let modifiers = [
        // skin tone (light, med-l, med, med-d, dark)
        '\u{1f3fb}', '\u{1f3fc}', '\u{1f3fd}', '\u{1f3fe}', '\u{1f3ff}',
        // gender (male, female)
        '\u{2640}', '\u{2642}',
        // hair (red, curly, bald, white)
        '\u{1f9b0}', '\u{1f9b1}', '\u{1f9b2}', '\u{1f9b3}'
    ];

    // let mut content = Vec::with_capacity(14);
    // let mut extra_w = 0;
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
                    0 => println!("Content::Zero({:?})", car),
                    1 => {
                        if car == ' ' { println!("Content::Blank") }
                        else { println!("Content::Single({})", car) }
                    },
                    2 => {
                        match graphemes.peek() {
                            Some(g) => {
                                let next_first = g.chars().next().unwrap();
                                if modifiers.contains(&next_first) {
                                    // println!("DBL JOIN");
                                    let mut width = 2;
                                    let mut content = vec![car];
                                    if let Some(s) = graphemes.next() {
                                        chars = s.chars().peekable();
                                        loop {
                                            if let Some(next) = chars.next() {
                                                content.push(next);
                                                width += next.width().unwrap_or(0);
                                            } else {
                                                if let '\u{200d}' = content.last().unwrap() {
                                                    if let Some(s) = graphemes.next() {
                                                        chars = s.chars().peekable();
                                                        continue;
                                                    }
                                                } else { break }
                                            }
                                        }
                                        println!("Content::Complex({:?}) | width: {}",
                                                 content, width);
                                        continue;
                                    }
                                }
                            },
                            _ => (),
                        }
                        println!("Content::Double({})", car);
                    },
                    _ => println!("Content::Unsupported"),
                },
                // Escape character
                None => match car {
                    '\t' => println!("Content::Tab"),
                    '\n' => println!("Content::LF"),
                    '\r' => println!("Content::CR"),
                    _ => println!("Content::Esc({:?})", car),
                }
            },
            // A complex grapheme - can be emoji, CRLF, or language:
            Some(cadr) => match (car, cadr) {
                ('\r', '\n') => println!("Content::CRLF"),
                _ => {
                    let mut width = car.width().unwrap_or(0);
                    // let mut width = car.width().unwrap_or(0) + extra_w;
                    let mut content = vec![car];
                    // content.push(car);
                    loop {
                        if let Some(next) = chars.next() {
                            // Continue iterating through grapheme cluster:
                            content.push(next);
                            width += next.width().unwrap_or(0);
                            // if modifiers.contains(content.last().unwrap()) {
                            //     println!("FITZ");
                            // }
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

                    let pred = |m: &char| *m == '\u{200d}';
                    let slice = content.split(pred).next();
                    let mut cutoff = 0;
                    for c in slice.unwrap_or(&[]) {
                        cutoff += c.len_utf8();
                    }
                    println!("cutoff: {}", cutoff);
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
                    // content.clear();
                    // extra_w = 0;
                }
            }
        }}
    }
    println!("output: \"{}\"", string);
}
