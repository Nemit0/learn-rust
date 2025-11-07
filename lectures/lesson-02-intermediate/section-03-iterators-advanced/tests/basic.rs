use lesson02_section03_iterators_advanced::{Fibonacci, fib_take};

#[test]
fn iterates_exactly_n() {
    let n = 10;
    let v: Vec<u64> = Fibonacci::new(n).collect();
    assert_eq!(v.len(), n);
    assert_eq!(&v[..5], &[0, 1, 1, 2, 3]);
}

#[test]
fn fib_take_collect() {
    assert_eq!(fib_take(0), Vec::<u64>::new());
    assert_eq!(fib_take(1), vec![0]);
    assert_eq!(fib_take(7), vec![0, 1, 1, 2, 3, 5, 8]);
}

