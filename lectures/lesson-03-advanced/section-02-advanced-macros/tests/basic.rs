use lesson03_section02_advanced_macros::HashMap;

#[test]
fn vec_of_strings_builds() {
    let v = lesson03_section02_advanced_macros::vec_of_strings!("a", 1, String::from("x"));
    assert_eq!(v, vec!["a".to_string(), "1".to_string(), "x".to_string()]);
}

#[test]
fn make_map_pairs() {
    let m: HashMap<String, i32> = lesson03_section02_advanced_macros::make_map!(
        "a" => 1, "b" => 2, String::from("c") => 3
    );
    assert_eq!(m.get("a"), Some(&1));
    assert_eq!(m.get("b"), Some(&2));
    assert_eq!(m.get("c"), Some(&3));
    assert_eq!(m.len(), 3);
}
