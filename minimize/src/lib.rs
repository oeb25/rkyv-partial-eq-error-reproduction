#![allow(unused, clippy::needless_borrow, clippy::no_effect)]

use std::marker::PhantomData;

struct ArchivedOption<T> {
    ph: PhantomData<T>,
}

impl<T: PartialEq<U>, U> PartialEq<ArchivedOption<T>> for Option<U> {
    fn eq(&self, _other: &ArchivedOption<T>) -> bool {
        todo!()
    }
}

fn test() {
    Some("") == Some(&"");
}
