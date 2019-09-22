#[test]
fn test_unicode_width() {
    let ascii = "AZaz09@&=?";
    assert_eq!(super::unicode::wcwidth::UnicodeWidthStr::width(ascii), 10);

    let nippon = "だちぷへぺほぼぽまぢ";
    assert_eq!(super::unicode::wcwidth::UnicodeWidthStr::width(nippon), 20);

    let hangul = "ㅁㅂㅃㅄㅅㅆㅇㅈㅉㅊ";
    assert_eq!(super::unicode::wcwidth::UnicodeWidthStr::width(hangul), 20);
    
    
    let putong = "㓐㓒㓓㓕㓗㓘㓙㓚㓝㓞";
    assert_eq!(super::unicode::wcwidth::UnicodeWidthStr::width(putong), 20);

    let unicode_single = "⓴█│┘▶⚜";
    assert_eq!(super::unicode::wcwidth::UnicodeWidthStr::width(unicode_single), 6);

    let unicode_double = "❌⚡😂🔍㊙🐘🙇🙋🤦";
    assert_eq!(super::unicode::wcwidth::UnicodeWidthStr::width(unicode_double), 18);

    let facepalm = "🤦\u{200d}\u{fe0f}";
    let facepalm_zero_width = '\u{200d}';
    assert_eq!(super::unicode::wcwidth::UnicodeWidthStr::width(facepalm), 2);
    assert_eq!(super::unicode::wcwidth::UnicodeWidthChar::width(facepalm_zero_width), Some(0));

    let fem_facepalm = "🤦‍♀️";
    let fem_facepalm_literal = "\u{1F926}\u{200d}\u{2640}\u{fe0f}";
    assert_eq!(fem_facepalm, fem_facepalm_literal);
}

#[test]
fn test_effect_bitwise_traits() {
    use super::enums::Effect;
    assert_eq!(1, 0b0001);
    let single_bold = Effect::Bold;
    assert_eq!(single_bold as u32, 0b0100_0000_0000);
    assert_eq!(format!("{:x}", single_bold as u32), format!("{:x}", 0x0400));
    assert_eq!(single_bold as u32, 1024);

    let single_dim = Effect::Dim;
    assert_eq!(single_dim as u32, 0b1000_0000_0000);
    assert_eq!(format!("{:x}", single_dim as u32), format!("{:x}", 0x0800));
    assert_eq!(single_dim as u32, 2048);

    let combo = Effect::Bold | Effect::Dim;
    assert_eq!(combo as u32, 0b1100_0000_0000);
    assert_eq!(format!("{:x}", combo as u32), format!("{:x}", 0x0c00));
    assert_eq!(combo as u32, 3072);

    let check_none = combo & Effect::Reverse;
    assert_eq!(check_none, 0);
    let check_some = combo & Effect::Bold;
    assert_ne!(check_some, 0);
}