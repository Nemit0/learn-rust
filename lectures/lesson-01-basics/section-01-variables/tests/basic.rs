use lesson01_section01_variables::*;

#[test]
fn returns_ten() {
    assert_eq!(make_ten(), 10);
}

#[test]
fn doubles_numbers() {
    assert_eq!(double(4), 8);
    assert_eq!(double(0), 0);
    assert_eq!(double(-3), -6);
}

