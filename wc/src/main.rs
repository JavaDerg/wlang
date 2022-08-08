#![cfg_attr(debug_assertions, allow(dead_code))]

use std::rc::Rc;
use w_parse::{parse, TokenSpan};
use w_tokenize::{tokenize, Span};

fn main() {
    let file = Span::new(include_str!("../../WIP_tests/old_1/test1.w"));
    let (_, tokens) = tokenize(file).unwrap();
    let (_, _parsed) = parse(TokenSpan::new(file, Rc::from(tokens))).unwrap();
}
