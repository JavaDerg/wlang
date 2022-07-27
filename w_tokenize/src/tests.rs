use crate::{parse_block, parse_tuple, token, tokenize, Span};
use nom::character::complete::char;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::Slice;

#[test]
fn parses() {
    let span = Span::new(include_str!("../../examples/generics1.w"));
    let (_, tokens) = tokenize(span).unwrap();
    println!("{:#?}", tokens);
}
