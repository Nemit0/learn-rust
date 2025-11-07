use lesson01_section09_modules_workspaces::*;

#[test]
fn reexports_work() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(mul(4, 5), 20);
    assert_eq!(shout("rust"), "RUST!");
}

