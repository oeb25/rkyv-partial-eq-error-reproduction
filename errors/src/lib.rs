#![allow(unused, clippy::needless_borrow)]

use rkyv::AlignedVec;
fn test() {
    assert!(Some("") == Some(&""));
}
