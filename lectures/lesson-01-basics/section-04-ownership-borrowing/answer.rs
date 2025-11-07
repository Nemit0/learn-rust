// Reference solution (not compiled by default).

pub fn append_in_place(s: &mut String, suffix: &str) {
    s.push_str(suffix);
}

pub fn first_char(s: &str) -> Option<char> {
    s.chars().next()
}

pub fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

