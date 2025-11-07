// Reference solution (not compiled by default).

use std::slice;

pub unsafe fn sum_via_ptr(ptr: *const i32, len: usize) -> i32 {
    let xs = slice::from_raw_parts(ptr, len);
    xs.iter().copied().sum()
}

pub fn sum_slice_unsafe(xs: &[i32]) -> i32 {
    unsafe { sum_via_ptr(xs.as_ptr(), xs.len()) }
}

pub fn split_at_mut_via_ptr(xs: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    assert!(mid <= xs.len(), "mid out of bounds");
    let len = xs.len();
    let ptr = xs.as_mut_ptr();
    unsafe {
        let left = slice::from_raw_parts_mut(ptr, mid);
        let right = slice::from_raw_parts_mut(ptr.add(mid), len - mid);
        (left, right)
    }
}

