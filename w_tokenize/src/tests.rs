use crate::{tokenize, Span};

#[test]
fn parses() {
    let span = Span::new(include_str!("../../WIP_tests/old_1/generics1.w"));
    let (_, tokens) = tokenize(span).unwrap();
    println!("{:#?}", tokens);
}
