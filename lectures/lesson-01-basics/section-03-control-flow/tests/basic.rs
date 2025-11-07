use lesson01_section03_control_flow::*;

#[test]
fn even_check() {
    assert!(is_even(0));
    assert!(is_even(2));
    assert!(!is_even(3));
    assert!(is_even(-4));
}

#[test]
fn fizz_buzz_rules() {
    assert_eq!(fizz_buzz(3), "Fizz");
    assert_eq!(fizz_buzz(5), "Buzz");
    assert_eq!(fizz_buzz(15), "FizzBuzz");
    assert_eq!(fizz_buzz(2), "2");
}

#[test]
fn sum_to_n() {
    assert_eq!(sum_up_to(0), 0);
    assert_eq!(sum_up_to(1), 1);
    assert_eq!(sum_up_to(5), 15);
}

