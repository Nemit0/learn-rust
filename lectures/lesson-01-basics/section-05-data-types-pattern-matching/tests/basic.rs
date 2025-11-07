use lesson01_section05_data_types_pattern_matching::*;

#[test]
fn weekday_mapping() {
    assert_eq!(weekday_from(1), Some(Weekday::Mon));
    assert_eq!(weekday_from(7), Some(Weekday::Sun));
    assert_eq!(weekday_from(0), None);
    assert_eq!(weekday_from(8), None);
}

#[test]
fn classify_quadrants() {
    assert_eq!(quadrant(Point(0, 5)), "axis");
    assert_eq!(quadrant(Point(3, 4)), "Q1");
    assert_eq!(quadrant(Point(-2, 7)), "Q2");
    assert_eq!(quadrant(Point(-1, -1)), "Q3");
    assert_eq!(quadrant(Point(2, -3)), "Q4");
}

#[test]
fn safe_division() {
    assert_eq!(safe_div(10, 2), Some(5));
    assert_eq!(safe_div(1, 0), None);
}

