//! Section: Data Types & Pattern Matching
//!
//! Define enums/structs and use `match` to branch on values.

#[derive(Debug, PartialEq, Eq)]
pub enum Weekday {
    Mon, Tue, Wed, Thu, Fri, Sat, Sun,
}

/// Convert 1..=7 to `Weekday`; otherwise return `None`.
pub fn weekday_from(n: u8) -> Option<Weekday> {
    // Hint: use `match n { 1 => ..., _ => None }`
    unimplemented!("map 1..=7 to Weekday variants")
}

#[derive(Debug, PartialEq)]
pub struct Point(pub i32, pub i32);

/// Describe the quadrant of a point.
/// - On axes => "axis"
/// - x>0,y>0 => "Q1"; x<0,y>0 => "Q2"; x<0,y<0 => "Q3"; x>0,y<0 => "Q4"
pub fn quadrant(p: Point) -> &'static str {
    // Hint: match with guards, e.g., `Point(x, y) if x == 0 || y == 0 => ...`
    unimplemented!("use match with guards to classify")
}

/// Safe integer division. Return `None` on divide-by-zero.
pub fn safe_div(a: i32, b: i32) -> Option<i32> {
    // Hint: early return None when b == 0
    unimplemented!("return Some(a / b) when b != 0")
}