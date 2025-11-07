use lesson01_section02_functions::*;

#[test]
fn add_works() {
    assert_eq!(add(2, 2), 4);
    assert_eq!(add(-1, 1), 0);
    assert_eq!(add(10, -3), 7);
}

#[test]
fn greeting_is_correct() {
    assert_eq!(greet("Rustacean"), "Hello, Rustacean!");
}

