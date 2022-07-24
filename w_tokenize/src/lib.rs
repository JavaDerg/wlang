use nom::bytes::complete::tag;
use nom::error::ParseError;
use nom::{Err, IResult, Parser, Slice};

mod error;
mod identifier;
mod string;
mod r#type;

use crate::error::TokenError;
use error::ToTokenError;

type Span<'a> = nom_locate::LocatedSpan<&'a str>;
type TokResult<'a, R = Span<'a>> = IResult<Span<'a>, R, error::TokenError<'a>>;

pub struct Token<'a> {
    pub span: Span<'a>,
    pub kind: Kind,
}

pub enum Kind {
    Ident,
    Defines,
    String(String),
}

fn defines(i: Span) -> TokResult<()> {
    tag("::")(i).map(|(o, _)| (o, ()))
}

fn boundary(i: Span, mut fail: impl FnMut(char) -> bool) -> TokResult<()> {
    if i.len() == 0 || {
        let first = i.chars().next().unwrap();
        char::is_whitespace(first) || !fail(first)
    } {
        Ok((i, ()))
    } else {
        Err(Err::Error(TokenError::new(
            Span::slice(&i, ..0),
            "Invalid boundary",
        )))
    }
}

fn bounded<'a, O, F, B>(mut parser: F, mut fail_bound: B) -> impl FnMut(Span<'a>) -> TokResult<O>
where
    F: Parser<Span<'a>, O, TokenError<'a>>,
    B: FnMut(char) -> bool,
{
    move |i| {
        let (i, o) = parser.parse(i)?;
        let (i, ()) = boundary(i, &mut fail_bound)?;
        Ok((i, o))
    }
}
