mod error;
mod parser;
mod types;

use crate::parser::Weak;
pub use crate::parser::{ParResult, TokenSpan};
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::Parser;
use std::collections::HashMap;
use std::rc::Rc;
use w_tokenize::{Kind, Span, TokResult, Token};

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
    let (i, test) = tag(Weak(Kind::Ident))(i)?;
    Ok((i, ()))
}

fn parse_identifier(i: TokenSpan) -> ParResult<Identifier> {
    let (i, tok) = Weak(Kind::Ident).parse(i)?;
    Ok((i, Identifier(tok.span)))
}
