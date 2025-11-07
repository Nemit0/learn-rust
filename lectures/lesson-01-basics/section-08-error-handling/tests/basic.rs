use lesson01_section08_error_handling::*;

#[test]
fn parses_and_adds() {
    assert_eq!(parse_and_add("2", "3").unwrap(), 5);
    assert!(parse_and_add("x", "3").is_err());
}

#[test]
fn converts_to_positive() {
    assert_eq!(to_positive(0).unwrap(), 0);
    assert_eq!(to_positive(5).unwrap(), 5);
    assert!(to_positive(-1).is_err());
}

#[test]
fn first_element_or_error() {
    assert_eq!(first_or_err(&[10, 20]).unwrap(), 10);
    let err = first_or_err(&[]).unwrap_err();
    assert!(err.contains("empty"));
}

