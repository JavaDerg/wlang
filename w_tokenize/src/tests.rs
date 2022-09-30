use crate::{tokenize, Span};
use w_rcstr::RcStr;

#[test]
fn parses() {
    let span = Span::new(RcStr::new(
        include_str!("../../WIP_tests/old_1/generics1.w").to_string(),
    ));
    let (_, tokens) = tokenize(span).unwrap();
    println!("{:#?}", tokens);
}
