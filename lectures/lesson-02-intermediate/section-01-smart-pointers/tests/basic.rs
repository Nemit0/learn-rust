use lesson02_section01_smart_pointers::{List, make_rc, clone_rc, strong_count};

#[test]
fn cons_and_len() {
    // Build list [1, 2, 3]
    let list = List::cons(3, List::Nil);
    let list = List::cons(2, list);
    let list = List::cons(1, list);

    assert_eq!(list.len(), 3);
    assert_eq!(list.to_vec(), vec![1, 2, 3]);
}

#[test]
fn rc_sharing_counts() {
    let rc = make_rc(42);
    assert_eq!(strong_count(&rc), 1);
    {
        let a = clone_rc(&rc);
        let b = clone_rc(&rc);
        assert_eq!(strong_count(&rc), 3);
        // ensure the inner value is accessible and shared
        assert_eq!(*a, 42);
        assert_eq!(*b, 42);
    }
    assert_eq!(strong_count(&rc), 1);
}

