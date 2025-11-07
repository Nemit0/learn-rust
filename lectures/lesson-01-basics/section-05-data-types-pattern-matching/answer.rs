// Reference solution (not compiled by default).

#[derive(Debug, PartialEq, Eq)]
pub enum Weekday {
    Mon, Tue, Wed, Thu, Fri, Sat, Sun,
}

pub fn weekday_from(n: u8) -> Option<Weekday> {
    match n {
        1 => Some(Weekday::Mon),
        2 => Some(Weekday::Tue),
        3 => Some(Weekday::Wed),
        4 => Some(Weekday::Thu),
        5 => Some(Weekday::Fri),
        6 => Some(Weekday::Sat),
        7 => Some(Weekday::Sun),
        _ => None,
    }
}

#[derive(Debug, PartialEq)]
pub struct Point(pub i32, pub i32);

pub fn quadrant(Point(x, y): Point) -> &'static str {
    match (x, y) {
        (_, 0) | (0, _) => "axis",
        (x, y) if x > 0 && y > 0 => "Q1",
        (x, y) if x < 0 && y > 0 => "Q2",
        (x, y) if x < 0 && y < 0 => "Q3",
        _ => "Q4",
    }
}

pub fn safe_div(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}

