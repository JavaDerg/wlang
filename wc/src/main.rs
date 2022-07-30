#![cfg_attr(debug_assertions, allow(dead_code))]

use std::rc::Rc;
use w_parse::{parse, TokenSpan};
use w_tokenize::{tokenize, Span};

fn main() {
    let file = Span::new(include_str!("../../WIP_tests/test1.w"));
    let (_, tokens) = tokenize(file.clone()).unwrap();
    let (_, parsed) = parse(TokenSpan::new(file, Rc::from(tokens))).unwrap();
}
