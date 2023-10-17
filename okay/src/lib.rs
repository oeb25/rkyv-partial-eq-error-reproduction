#![allow(unused, clippy::needless_borrow)]

fn test() {
    assert!(Some("") == Some(&""));
}
