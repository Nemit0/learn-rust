use lesson03_section01_unsafe_basics::{sum_via_ptr, sum_slice_unsafe, split_at_mut_via_ptr};

#[test]
fn sums_match() {
    let v = vec![1, 2, 3, 4, 5];
    let seq: i32 = v.iter().copied().sum();
    let via_wrapper = sum_slice_unsafe(&v);
    assert_eq!(via_wrapper, seq);

    let via_ptr = unsafe { sum_via_ptr(v.as_ptr(), v.len()) };
    assert_eq!(via_ptr, seq);
}

#[test]
fn split_and_mutate() {
    let mut v = vec![10, 20, 30, 40, 50, 60];
    let len = v.len();
    let mid = 3;
    {
        let (a, b) = split_at_mut_via_ptr(&mut v, mid);
        assert_eq!(a, &[10, 20, 30]);
        assert_eq!(b, &[40, 50, 60]);
        // Mutate both halves to check no aliasing
        a[0] = 1;
        b[0] = 4;
    }
    assert_eq!(v.len(), len);
    assert_eq!(v, vec![1, 20, 30, 4, 50, 60]);
}

