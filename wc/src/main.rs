use w_parse::{parse, TokenSpan};
use w_tokenize::{Span, tokenize};

fn main() {
    let file = Span::new(include_str!("../../examples/test1.w"));
    let (_, tokens) = tokenize(file.clone()).unwrap();
    let (_, parsed) = parse(TokenSpan::new(file, &tokens)).unwrap();
}
