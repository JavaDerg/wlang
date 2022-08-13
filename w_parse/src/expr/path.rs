use crate::{parse_name, Ident, ParResult, TokenSpan, Weak};
use nom::combinator::{consumed, map};
use nom::multi::{separated_list0, separated_list1};
use w_tokenize::{Kind, Span};

pub struct Path<'a> {
    pub span: TokenSpan<'a>,
    pub path: Vec<Ident<'a>>,
}

pub fn parse_path(i: TokenSpan) -> ParResult<Path> {
    map(
        consumed(separated_list1(Weak(Kind::Colon), parse_name)),
        |(span, path)| Path { span, path },
    )(i)
}
