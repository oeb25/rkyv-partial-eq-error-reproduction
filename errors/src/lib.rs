#![allow(unused, clippy::needless_borrow, clippy::no_effect)]

use rkyv::AlignedVec;
fn test() {
    Some("") == Some(&"");
}
