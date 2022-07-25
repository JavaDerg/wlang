use crate::{parse_block, token, tokenize, Span, parse_tuple};
use nom::character::complete::char;
use nom::multi::many0;
use nom::sequence::delimited;

#[test]
fn parses() {
    let span = Span::new(include_str!("../../examples/generics1.w"));
    let (_, tokens) = tokenize(span).unwrap();
    println!("{:#?}", tokens);
}
