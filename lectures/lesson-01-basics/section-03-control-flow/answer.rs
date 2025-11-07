// Reference solution (not compiled by default).

pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

pub fn fizz_buzz(n: i32) -> String {
    if n % 15 == 0 {
        "FizzBuzz".to_string()
    } else if n % 3 == 0 {
        "Fizz".to_string()
    } else if n % 5 == 0 {
        "Buzz".to_string()
    } else {
        n.to_string()
    }
}

pub fn sum_up_to(n: u32) -> u32 {
    let mut acc = 0;
    for i in 1..=n {
        acc += i;
    }
    acc
}

