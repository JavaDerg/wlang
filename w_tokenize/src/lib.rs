#![cfg_attr(debug_assertions, allow(dead_code))]

extern crate core;

use nom::bytes::complete::{is_not, tag, take_while, take_while_m_n};
use nom::character::complete::char;
use nom::combinator::{map, not, opt, peek};

use nom::branch::alt;
use nom::complete::take;
use nom::multi::{fold_many0, many0};
use nom::sequence::{delimited, pair, terminated};
use nom::{Err, IResult, Offset, Parser, Slice};

mod error;
mod identifier;
mod number;
mod string;
#[cfg(test)]
mod tests;

use crate::error::TokenError;
use crate::identifier::parse_ident;
use crate::number::{parse_integer, Number};
use crate::string::parse_string;
use error::ToTokenError;

type Span<'a> = nom_locate::LocatedSpan<&'a str>;
type TokResult<'a, R = Span<'a>> = IResult<Span<'a>, R, TokenError<'a>>;

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub span: Span<'a>,
    pub kind: Kind<'a>,
}

#[derive(Debug, Clone)]
pub enum Kind<'a> {
    Ident,

    /// `::`
    Names,

    /// `:=`
    Define,
    /// `=`
    Set,

    /// `,`
    Sep,
    /// `;`
    Sim,

    /// `.`
    Call,

    /// `(TOKENS)`
    Tuple(Vec<Token<'a>>),
    /// `{TOKENS}`
    Block(Vec<Token<'a>>),

    String(String),
    Number(Number<'a>),
}

pub fn tokenize(mut i: Span) -> TokResult<Vec<Token>> {
    let mut tokens = vec![];
    loop {
        let (ni, token) = token(i).reason("failed to parse entire file")?;
        tokens.push(token);
        i = ni;

        if i.len() == 0 {
            break;
        }
    }

    Ok((i, tokens))
}

fn token(i: Span) -> TokResult<Token> {
    // yeet the whitespaces
    let (mut i, _) = take_while(char::is_whitespace)(i)?;

    let (i, _) = opt(consume_singleline_comments)(i)?;
    let (i, _) = opt(consume_multiline_comments)(i)?;

    alt((
        map(parse_string, |(span, str)| Token {
            span,
            kind: Kind::String(str),
        }),
        map(parse_integer, |(span, num)| Token {
            span,
            kind: Kind::Number(num),
        }),
        parse_block,
        parse_tuple,
        map(parse_ident, |span| Token {
            span,
            kind: Kind::Ident,
        }),
        op("::", ":=", || Kind::Names),
        op(":=", ":=", || Kind::Define),
        op("=", "", || Kind::Set),
        op(",", "", || Kind::Sep),
        op(".", "", || Kind::Call),
        op(";", "", || Kind::Sim),
    ))(i)
}

fn consume_singleline_comments(mut oi: Span) -> TokResult<()> {
    loop {
        let (i, _) = tag("//")(oi)?;
        let (i, _) = terminated(many0(is_not("\r\n")), take_while(char::is_whitespace))(i)?;
        oi = i;
        if tag::<_, _, TokenError>("//")(i.clone()).is_err() {
            break;
        }
    }
    Ok((oi, ()))
}

fn consume_multiline_comments(i: Span) -> TokResult<()> {
    delimited(
        tag("/*"),
        fold_many0(
            alt((
                consume_multiline_comments,
                map(pair(not(tag("*/")), take_while_m_n(1, 1, |_| true)), |_| ()),
            )),
            || (),
            |_, _| (),
        ),
        pair(tag("*/"), take_while(char::is_whitespace)),
    )(i)
}

fn parse_tuple(oi: Span) -> TokResult<Token> {
    let (i, o) = bounded(delimited(char('('), many0(token), char(')')), |_| false)(oi)?;
    let offset = oi.offset(&i);
    let span = Span::slice(&oi, ..offset);

    Ok((
        i,
        Token {
            span,
            kind: Kind::Tuple(o),
        },
    ))
}

fn parse_block(oi: Span) -> TokResult<Token> {
    let (i, o) = bounded(delimited(char('{'), many0(token), char('}')), |_| false)(oi)?;
    let offset = oi.offset(&i);
    let span = Span::slice(&oi, ..offset);

    Ok((
        i,
        Token {
            span,
            kind: Kind::Block(o),
        },
    ))
}

fn op<'a>(
    op: &'static str,
    bound: &'static str,
    kind: fn() -> Kind<'static>,
) -> impl FnMut(Span<'a>) -> TokResult<Token<'a>> + 'a {
    map(bounded(tag(op), move |c| bound.contains(c)), move |o| {
        Token {
            span: o,
            kind: kind(),
        }
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
