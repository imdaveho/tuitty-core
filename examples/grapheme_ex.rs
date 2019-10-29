extern crate tuitty;
use tuitty::common::unicode::grapheme::UnicodeGraphemes;
// use tuitty::common::unicode::wcwidth::UnicodeWidthStr;


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

    let content = "\x1B\t\r\n";
    let clusters = content.graphemes(true).collect::<Vec<&str>>();
    println!("{:?}", clusters);
    for n in clusters {
        println!("{}", n.is_ascii());
    }
}
