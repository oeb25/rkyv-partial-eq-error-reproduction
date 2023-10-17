#![allow(unused, clippy::needless_borrow, clippy::no_effect)]

fn test() {
    Some("") == Some(&"");
}
