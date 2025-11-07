use lesson01_section04_ownership_borrowing::*;

#[test]
fn appends_in_place() {
    let mut s = String::from("Hello");
    append_in_place(&mut s, ", world");
    assert_eq!(s, "Hello, world");
}

#[test]
fn first_char_works() {
    assert_eq!(first_char(""), None);
    assert_eq!(first_char("abc"), Some('a'));
}

#[test]
fn longer_chooses_correct_ref() {
    let a = String::from("short");
    let b = String::from("longer");
    let r = longer(&a, &b);
    assert_eq!(r, "longer");
}

