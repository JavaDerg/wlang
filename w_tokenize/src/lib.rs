#![cfg_attr(debug_assertions, allow(dead_code))]

extern crate core;

use nom::bytes::complete::{is_not, tag, take_while, take_while_m_n};
use nom::character::complete::char;
use nom::combinator::{map, not, opt};
use std::rc::Rc;

use nom::branch::alt;

use nom::multi::{fold_many0, many0};
use nom::sequence::{delimited, pair, terminated};
use nom::{Err, IResult, InputLength, Offset, Parser, Slice};

mod error;
mod identifier;
mod number;
mod string;
#[cfg(test)]
mod tests;

use crate::error::{TokenError, TokenErrorKind};
use crate::identifier::parse_ident;
use crate::number::parse_integer;
pub use crate::number::Number;
use crate::string::parse_string;
use error::ToTokenError;

pub type Span<'a> = nom_locate::LocatedSpan<&'a str, &'a str>;
pub type TokResult<'a, R = Span<'a>> = IResult<Span<'a>, R, TokenError<'a>>;

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub span: Span<'a>,
    pub kind: Kind<'a>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Kind<'a> {
    /// Regular identifier
    Ident,

    /// `_`
    Fill,

    /// `:`
    Colon,
    /// `::`
    DoubleCol,

    /// `:=`
    Define,
    /// `=`
    Assign,

    /// `,`
    Comma,
    /// `;`
    Semicolon,
    /// `.`
    Dot,

    // Math operands
    /// `+`
    Add,
    /// `-`
    Sub,
    /// `*`
    Mul,
    /// `/`
    Div,
    /// `%`
    Mod,

    // Bitwise operands
    /// `&`
    And,
    /// `|`
    Or,
    /// `^`
    Xor,
    /// `<<`
    Shl,
    /// `>>`
    Shr,

    // Comparison operands
    /// `==`
    Eq,
    /// `!=`
    Neq,
    /// `<`
    Lt,
    /// `<=`
    Le,
    /// `>`
    Gt,
    /// `>=`
    Ge,

    // Logical operands
    /// `&&`
    AndL,
    /// `||`
    OrL,
    /// `!`
    Not,

    // Assignment operands
    /// `+=`
    AddAssign,
    /// `-=`
    SubAssign,
    /// `*=`
    MulAssign,
    /// `/=`
    DivAssign,
    /// `%=`
    ModAssign,
    /// `&=`
    AndAssign,
    /// `|=`
    OrAssign,
    /// `^=`
    XorAssign,
    /// `<<=`
    ShlAssign,
    /// `>>=`
    ShrAssign,

    /// `(TOKENS)`
    Tuple(Rc<[Token<'a>]>),
    /// `{TOKENS}`
    Block(Rc<[Token<'a>]>),
    /// `[TOKENS]`
    Array(Rc<[Token<'a>]>),

    String(String),
    Number(Box<Number<'a>>),
}

pub fn tokenize(mut i: Span) -> TokResult<Vec<Token>> {
    let mut tokens = vec![];
    loop {
        let (ni, token) = token(i).reason("failed to parse entire file")?;
        if let Some(token) = token {
            tokens.push(token);
        }
        i = ni;

        if i.len() == 0 {
            break;
        }
    }

    Ok((i, tokens))
}

fn token(i: Span) -> TokResult<Option<Token>> {
    // yeet the whitespaces
    let (oi, _) = whitespace(i)?;

    let (i, _) = opt(consume_singleline_comments)(oi)?;
    let (i, _) = opt(consume_multiline_comments)(i)?;

    let comments_pruned = oi.offset(&i) != 0;

    // this had to be done due to limitations with `alt`
    let res = alt((
        map(parse_string, |(span, str)| Token {
            span,
            kind: Kind::String(str),
        }),
        map(parse_integer, |(span, num)| Token {
            span,
            kind: Kind::Number(Box::new(num)),
        }),
        parse_block,
        parse_tuple,
        parse_array,
        map(parse_ident, |span| Token {
            span,
            kind: Kind::Ident,
        }),
        // assignment operators
        op("_", "", || Kind::Fill),
        alt((
            op("<<=", "", || Kind::ShlAssign),
            op(">>=", "", || Kind::ShrAssign),
            op("+=", "", || Kind::AddAssign),
            op("-=", "", || Kind::SubAssign),
            op("*=", "", || Kind::MulAssign),
            op("/=", "", || Kind::DivAssign),
            op("%=", "", || Kind::ModAssign),
            op("&=", "", || Kind::AndAssign),
            op("|=", "", || Kind::OrAssign),
            op("^=", "", || Kind::XorAssign),
        )),
        // logic operators
        alt((
            op("&&", "", || Kind::AndL),
            op("||", "", || Kind::OrL),
            op("!", "", || Kind::Not),
        )),
        // bitwise operators
        alt((
            op("<<", "", || Kind::Shl),
            op(">>", "", || Kind::Shr),
            op("&", "", || Kind::And),
            op("|", "", || Kind::Or),
            op("^", "", || Kind::Xor),
        )),
        // math operations
        alt((
            op("+", "", || Kind::Add),
            op("-", "", || Kind::Sub),
            op("*", "", || Kind::Mul),
            op("/", "", || Kind::Div),
            op("%", "", || Kind::Mod),
        )),
        // comparison operators
        alt((
            op("==", "", || Kind::Eq),
            op("!=", "", || Kind::Neq),
            op("<=", "", || Kind::Le),
            op(">=", "", || Kind::Ge),
            op("<", "", || Kind::Lt),
            op(">", "", || Kind::Gt),
        )),
        // other
        alt((
            op("::", ":=", || Kind::DoubleCol),
            op(":=", ":=", || Kind::Define),
            op(":", "=", || Kind::DoubleCol),
            op(",", "", || Kind::Comma),
            op(".", "", || Kind::Dot),
            op(";", "", || Kind::Semicolon),
            op("=", "", || Kind::Assign),
        )),
    ))(i);

    if res.is_ok() {
        res.map(|(i, tok)| (i, Some(tok)))
    } else if comments_pruned {
        Ok((i, None))
    } else {
        res.map(|_| unreachable!())
    }
}

fn consume_singleline_comments(mut oi: Span) -> TokResult<()> {
    loop {
        let (i, _) = tag("//")(oi)?;
        let (i, _) = terminated(many0(is_not("\r\n")), whitespace)(i)?;
        oi = i;
        if tag::<_, _, TokenError>("//")(i).is_err() {
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
        pair(tag("*/"), whitespace),
    )(i)
}

fn parse_tuple(oi: Span) -> TokResult<Token> {
    let (i, (span, o)) = parsed_delimited(oi, '(', ')')?;

    Ok((
        i,
        Token {
            span,
            kind: Kind::Tuple(Rc::from(o.into_boxed_slice())),
        },
    ))
}

fn parse_block(oi: Span) -> TokResult<Token> {
    let (i, (span, o)) = parsed_delimited(oi, '{', '}')?;

    Ok((
        i,
        Token {
            span,
            kind: Kind::Block(Rc::from(o.into_boxed_slice())),
        },
    ))
}

fn parse_array(oi: Span) -> TokResult<Token> {
    let (i, (span, o)) = parsed_delimited(oi, '[', ']')?;

    Ok((
        i,
        Token {
            span,
            kind: Kind::Array(Rc::from(o.into_boxed_slice())),
        },
    ))
}

fn parsed_delimited(oi: Span, start: char, end: char) -> TokResult<(Span, Vec<Token>)> {
    let (mut i, _) = pair(char(start), whitespace)(oi)?;
    let mut acc = vec![];

    let last_err;
    loop {
        match token(i) {
            Ok((ni, token)) => {
                if let Some(token) = token {
                    acc.push(token)
                };
                i = ni;
            }
            Err(Err::Error(mut err) | Err::Failure(mut err)) => {
                err.reason = Some(
                    format!(
                        "failure to parse at {}:{}",
                        i.location_line(),
                        i.location_offset()
                    )
                    .into(),
                );
                let err = TokenError {
                    span: oi,
                    kind: TokenErrorKind::Other(Box::new(err)),
                    reason: Some(
                        "Failed to parse delimited section due to unparseable token inside".into(),
                    ),
                };
                last_err = Some(Err(Err::Failure(err)));
                break;
            }
            err @ Err(_) => {
                return err.map(|_| unreachable!());
            }
        }
    }

    let (i, end_p) = opt(char(end))(i)?;
    if end_p.is_none() {
        if let Some(err) = last_err {
            err
        } else {
            Err(Err::Error(TokenError::new(i, format!("expected `{end}`"))))
        }
    } else {
        let offset = oi.offset(&i);
        let span = Span::slice(&oi, ..offset);
        Ok((whitespace(i)?.0, (span, acc)))
    }
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
        whitespace(i).map(|(i, _)| (i, ()))
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

fn whitespace(i: Span) -> TokResult<Span> {
    take_while(char::is_whitespace)(i)
}

impl<'a> PartialEq for Token<'a> {
    fn eq(&self, other: &Self) -> bool {
        *self.span == *other.span || self.kind == other.kind
    }
}
impl<'a> Eq for Token<'a> {}

impl<'a> Kind<'a> {
    // Important for weak comparison
    pub fn cmp_id(&self) -> u32 {
        match self {
            Kind::Ident => 0,
            Kind::DoubleCol => 1,
            Kind::Define => 2,
            Kind::Assign => 3,
            Kind::Comma => 4,
            Kind::Semicolon => 5,
            Kind::Dot => 6,
            Kind::Add => 7,
            Kind::Sub => 8,
            Kind::Mul => 9,
            Kind::Div => 10,
            Kind::Mod => 11,
            Kind::And => 12,
            Kind::Or => 13,
            Kind::Xor => 14,
            Kind::Shl => 15,
            Kind::Shr => 16,
            Kind::Eq => 17,
            Kind::Neq => 18,
            Kind::Lt => 19,
            Kind::Le => 20,
            Kind::Gt => 21,
            Kind::Ge => 22,
            Kind::AndL => 23,
            Kind::OrL => 24,
            Kind::Not => 25,
            Kind::AddAssign => 26,
            Kind::SubAssign => 27,
            Kind::MulAssign => 28,
            Kind::DivAssign => 29,
            Kind::ModAssign => 30,
            Kind::AndAssign => 31,
            Kind::OrAssign => 32,
            Kind::XorAssign => 33,
            Kind::ShlAssign => 34,
            Kind::ShrAssign => 35,
            Kind::Tuple(_) => 36,
            Kind::Block(_) => 37,
            Kind::Array(_) => 38,
            Kind::String(_) => 39,
            Kind::Number(_) => 40,
            Kind::Colon => 41,
            Kind::Fill => 42,
        }
    }
}

impl<'a> InputLength for Token<'a> {
    fn input_len(&self) -> usize {
        1
    }
}
