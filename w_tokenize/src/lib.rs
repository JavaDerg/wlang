#![cfg_attr(debug_assertions, allow(dead_code))]

use nom::bytes::complete::{tag, take_while};
use nom::character::complete::char;
use nom::combinator::{map};

use nom::multi::{many0};
use nom::sequence::delimited;
use nom::{Err, IResult, Offset, Parser, Slice};
use nom::branch::alt;

mod error;
mod identifier;
mod string;

use crate::error::TokenError;
use error::ToTokenError;
use crate::identifier::parse_ident;
use crate::string::parse_string;

type Span<'a> = nom_locate::LocatedSpan<&'a str>;
type TokResult<'a, R = Span<'a>> = IResult<Span<'a>, R, TokenError<'a>>;

pub struct Token<'a> {
    pub span: Span<'a>,
    pub kind: Kind<'a>,
}

pub enum Kind<'a> {
    Ident,

    /// `::`
    Defines,

    /// `:=`
    Define,
    /// `=`
    Set,

    /// `,`
    Sep,
    /// `;`
    Sim,

    /// `(TOKENS)`
    Tuple(Vec<Token<'a>>),
    /// `{TOKENS}`
    Block(Vec<Token<'a>>),

    String(String),
}

fn token(i: Span) -> TokResult<Token> {
    alt((
        map(parse_string, |(span, str)| Token {
            span,
            kind: Kind::String(str),
        }),
        parse_block,
        parse_tuple,
        map(parse_ident, |span| Token {
            span,
            kind: Kind::Ident,
        }),
        // FIXME: this should not allow = as well, fix this
        op("::", ':', || Kind::Defines),
        op(":=", ':', || Kind::Define),
        op("=", ':', || Kind::Set),
        // FIXME: Nothing not possible
        op(",", '\0', || Kind::Sep),
        op(";", '\0', || Kind::Sim),
    ))(i)
}

fn parse_tuple(oi: Span) -> TokResult<Token> {
    let (i, o) = bounded(delimited(char('('), many0(token), char(')')), |_| false)(oi)?;
    let offset = oi.offset(&i);
    let span = Span::slice(&oi, ..offset);

    Ok((i, Token {
        span,
        kind: Kind::Tuple(o)
    }))
}

fn parse_block(oi: Span) -> TokResult<Token> {
    let (i, o) = bounded(delimited(char('{'), many0(token), char('}')), |_| false)(oi)?;
    let offset = oi.offset(&i);
    let span = Span::slice(&oi, ..offset);

    Ok((i, Token {
        span,
        kind: Kind::Block(o)
    }))
}

fn op<'a>(
    op: &'static str,
    bound: char,
    kind: fn() -> Kind<'static>,
) -> impl FnMut(Span<'a>) -> TokResult<Token<'a>> + 'a {
    map(bounded(tag(op), move |c| c == bound), move |o| Token {
        span: o,
        kind: kind(),
    })
}

fn defines(i: Span) -> TokResult {
    bounded(tag("::"), |c| c == ':')(i)
}

fn boundary(i: Span, mut fail: impl FnMut(char) -> bool) -> TokResult<()> {
    if i.len() == 0 || {
        let first = i.chars().next().unwrap();
        char::is_whitespace(first) || !fail(first)
    } {
        take_while(char::is_whitespace)(i).map(|(i, _)| (i, ()))
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
