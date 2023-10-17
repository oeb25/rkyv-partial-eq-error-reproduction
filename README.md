# Reproduction for [scylladb/scylla-rust-driver#838](https://github.com/scylladb/scylla-rust-driver/pull/838)

Rust automatically dereferences `&&str` when comparing against `&str` as exemplified by the following example.

```rs
fn compiles() {
    Some("") == Some(&"");
}
```

However, adding any use of `rkyv` as in the following code:

```rs
use rkyv::AlignedVec;
fn does_not_compile() {
    Some("") == Some(&"");
}
```

Results in this error:

```rs
error[E0277]: can't compare `Option<&str>` with `Option<&&str>`
 --> src/lib.rs:3:14
  |
3 |     Some("") == Some(&"");
  |              ^^ no implementation for `Option<&str> == Option<&&str>`
  |
  = help: the trait `PartialEq<Option<&&str>>` is not implemented for `Option<&str>`
  = help: the following other types implement trait `PartialEq<Rhs>`:
            <Option<Box<U>> as PartialEq<ArchivedOptionBox<T>>>
            <Option<T> as PartialEq>
            <Option<U> as PartialEq<ArchivedOption<T>>>
```

---

This repository contains two crates, `okay` and `errors`, which reproduce the above successful and failing compile respectively.
