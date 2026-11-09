use rwc::{count, Counts};

#[test]
fn empty_input_is_zero() {
    let got = count("".as_bytes()).unwrap();
    assert_eq!(got, Counts::default());
}

#[test]
fn single_word_no_trailing_newline() {
    let got = count("hello".as_bytes()).unwrap();
    assert_eq!(got.words, 1);
    assert_eq!(got.lines, 0);
    assert_eq!(got.bytes, 5);
    assert_eq!(got.chars, 5);
}

#[test]
fn multiple_lines_and_words() {
    let got = count("one two\nthree four five\n".as_bytes()).unwrap();
    assert_eq!(got.lines, 2);
    assert_eq!(got.words, 5);
    assert_eq!(got.bytes, 24);
}

#[test]
fn tabs_and_spaces_are_both_separators() {
    let got = count("a\tb c\td\n".as_bytes()).unwrap();
    assert_eq!(got.words, 4);
    assert_eq!(got.lines, 1);
}

#[test]
fn utf8_characters_versus_bytes() {
    let text = "héllo wörld\n"; // 'é' and 'ö' are 2 bytes each in UTF-8.
    let got = count(text.as_bytes()).unwrap();
    assert_eq!(got.words, 2);
    assert_eq!(got.lines, 1);
    assert_eq!(got.bytes, text.len() as u64);
    assert_eq!(got.chars, text.chars().count() as u64);
}
