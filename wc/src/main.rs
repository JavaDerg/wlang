use w_parse::{parse, TokenSpan};
use w_tokenize::{Span, tokenize};

fn main() {
    let (_, tokens) = tokenize(Span::new(include_str!("../../examples/test1.w"))).unwrap();
    let (_, parsed) = parse(TokenSpan(&tokens)).unwrap();
}
