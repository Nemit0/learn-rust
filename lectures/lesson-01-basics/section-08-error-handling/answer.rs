// Reference solution (not compiled by default).

use std::num::ParseIntError;

pub fn parse_and_add(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x: i32 = a.parse()?;
    let y: i32 = b.parse()?;
    Ok(x + y)
}

pub fn to_positive(n: i32) -> Result<u32, String> {
    if n < 0 { Err("negative value".to_string()) } else { Ok(n as u32) }
}

pub fn first_or_err(xs: &[i32]) -> Result<i32, String> {
    xs.first().copied().ok_or_else(|| "slice is empty".to_string())
}

