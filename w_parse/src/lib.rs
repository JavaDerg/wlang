mod types;
mod error;
mod parser;

use std::collections::HashMap;
use std::rc::Rc;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use w_tokenize::{Kind, Span, Token, TokResult};
pub use crate::parser::{ParResult, TokenSpan};
use crate::parser::Weak;

pub type SVec<T> = Rc<[T]>;

pub struct Identifier<'a>(pub Span<'a>);

pub struct Name<'a> {
    pub main: Identifier<'a>,
    pub generic_params: SVec<Identifier<'a>>,
}

fn svconv<T>(v: Vec<T>) -> Rc<[T]> {
    Rc::from(v.into_boxed_slice())
}

pub fn parse<'a, 'b>(i: TokenSpan<'a, 'b>) -> ParResult<'a, 'b, ()> {
    let (i, test) = tag(Weak(Kind::Ident))(i)?;
    Ok((i, ()))
}
