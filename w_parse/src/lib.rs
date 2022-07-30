#![cfg_attr(debug_assertions, allow(dead_code))]

extern crate core;

mod definer;
mod error;
mod function;
mod parser;
mod types;

use crate::parser::Weak;
pub use crate::parser::{ParResult, TokenSpan};
use crate::types::{parse_type, Type};
use std::borrow::Cow;

use nom::combinator::verify;

use nom::{Err, Parser};

use crate::error::{Error, ErrorChain};
use std::rc::Rc;
use w_tokenize::{Kind, Span};

pub type SVec<T> = Rc<[T]>;

pub struct Identifier<'a>(pub Span<'a>);

pub struct Name<'a> {
    pub main: Identifier<'a>,
    pub generic_params: SVec<Identifier<'a>>,
}

fn svconv<T>(v: Vec<T>) -> Rc<[T]> {
    Rc::from(v.into_boxed_slice())
}

pub fn parse(i: TokenSpan) -> ParResult<()> {
    Ok((i, ()))
}

fn parse_identifier(i: TokenSpan) -> ParResult<Identifier> {
    let (i, tok) = Weak(Kind::Ident).parse(i)?;
    Ok((i, Identifier(tok.span)))
}

fn quick_err<T>(span: TokenSpan, reason: impl Into<Cow<'static, str>>) -> ParResult<T> {
    Err(Err::Error(ErrorChain::from(Error::new(span, reason))))
}

fn parse_name(i: TokenSpan) -> ParResult<Identifier> {
    verify(parse_identifier, keyword_check)(i)
}

fn keyword_check(ident: &Identifier) -> bool {
    !matches!(
        *ident.0,
        "struct" | "enum" | "func" | "for" | "loop" | "if" | "mut"
    )
}
