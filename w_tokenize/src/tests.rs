use crate::{tokenize, Span};
use w_rcstr::{Origin, RcStr};

#[test]
fn parses() {
    let span = Span::new(RcStr::new(
        include_str!("../../WIP_tests/old_1/generics1.w").to_string(),
        Origin::Unknown,
    ));
    let (_, tokens) = tokenize(span).unwrap();
    println!("{:#?}", tokens);
}
