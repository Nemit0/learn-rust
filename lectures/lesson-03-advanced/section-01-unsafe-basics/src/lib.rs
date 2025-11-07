//! Section: Unsafe Basics — Raw Pointers and Invariants
//!
//! Implement simple functions using raw pointers. Keep `unsafe` blocks small
//! and document invariants in comments.

use std::slice;

/// Sum `len` integers starting at `ptr`.
///
/// Safety contract (caller must uphold when calling this function):
/// - `ptr` must be valid for reads of `len` contiguous `i32` elements.
/// - The memory must not be mutated by others for the duration of the call.
/// - `ptr` may be null only if `len == 0`.
pub unsafe fn sum_via_ptr(_ptr: *const i32, _len: usize) -> i32 {
    // TODO: Create a temporary slice with `slice::from_raw_parts` and sum it.
    // Example idea:
    // let xs = slice::from_raw_parts(ptr, len);
    // xs.iter().copied().sum()
    unimplemented!("build a slice from raw parts and sum")
}

/// Safe wrapper that forwards to `sum_via_ptr`.
pub fn sum_slice_unsafe(xs: &[i32]) -> i32 {
    // TODO: Call `unsafe { sum_via_ptr(xs.as_ptr(), xs.len()) }`
    unimplemented!("forward to the unsafe function using as_ptr/len")
}

/// Split a mutable slice at `mid` using raw pointers.
/// Should behave like `xs.split_at_mut(mid)`.
pub fn split_at_mut_via_ptr(xs: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    // TODO: Use `xs.as_mut_ptr()` + `slice::from_raw_parts_mut` and `add(mid)`.
    // Remember to assert that `mid <= xs.len()` first.
    unimplemented!("construct two non-overlapping mutable slices via raw parts")
}

