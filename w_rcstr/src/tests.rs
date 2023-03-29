use crate::*;
use pretty_assertions::assert_eq;

#[test]
fn test_rcstr() {
    let s = RcStr::from("Hello, world!");
    let (a, b) = s.take_split(7);
    assert_eq!(a, "Hello, ");
    assert_eq!(b, "world!");
}

#[test]
fn test_rcstr_offset() {
    let s = RcStr::from("Hello, world!");
    let (a, b) = s.take_split(7);
    assert_eq!(a.offset(&b), 7);
}

#[test]
fn test_rcstr_compare() {
    let s = RcStr::from("Hello, world!");
    assert_eq!(s.compare("Hello, world!"), CompareResult::Ok);
    assert_eq!(s.compare("Hello, world"), CompareResult::Ok);
    assert_eq!(s.compare("Hello, world!!"), CompareResult::Incomplete);
    assert_eq!(s.compare("Hello, world!"), CompareResult::Ok);
    assert_eq!(s.compare("hello, world!"), CompareResult::Error);
    assert_eq!(s.compare_no_case("hello, world!"), CompareResult::Ok);
}

#[test]
fn test_rcstr_compare_rcstr() {
    let s = RcStr::from("Hello, world!");
    let t = RcStr::from("Hello, world!");
    assert_eq!(s.compare(t), CompareResult::Ok);
}

// rcstr slice tests

#[test]
fn test_rcstr_slice() {
    let s = RcStr::from("Hello, world!");
    let a = s.slice(7..);
    assert_eq!(a, "world!");
}

#[test]
fn test_rcstr_slice_index() {
    let s = RcStr::from("Hello, world!");
    let a = s.slice_index(7);
    assert_eq!(a, Ok(7));
}
