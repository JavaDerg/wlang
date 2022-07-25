use crate::{tokenize, Span};

#[test]
fn parses() {
    let span = Span::new(include_str!("../../examples/test1.w"));
    let (_, tokens) = tokenize(span).unwrap();
    println!("{:#?}", tokens);
}
