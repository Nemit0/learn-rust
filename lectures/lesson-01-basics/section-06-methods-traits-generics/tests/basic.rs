use lesson01_section06_methods_traits_generics::*;

#[test]
fn rectangle_methods() {
    let r = Rectangle { width: 3, height: 4 };
    assert_eq!(r.area(), 12);

    let a = Rectangle { width: 10, height: 8 };
    let b = Rectangle { width: 9, height: 7 };
    assert!(a.can_hold(&b));
    assert!(!b.can_hold(&a));
}

#[test]
fn trait_summary() {
    let p = Person { name: "Ada".to_string() };
    let s = p.summary();
    assert!(s.contains("Ada"));
}

#[test]
fn generic_max() {
    assert_eq!(max_of(3, 5), 5);
    assert_eq!(max_of('z', 'a'), 'z');
}

