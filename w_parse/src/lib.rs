#![cfg_attr(debug_assertions, allow(dead_code))]

pub mod error;
pub mod expr;
pub mod item;
pub mod module;
pub mod parser;
pub mod types;
pub mod util;

use crate::error::{Error, ErrorChain};
use crate::parser::Weak;
use crate::types::{parse_type, ItemTy};
use nom::combinator::{map, verify};
use nom::{Err, Parser};
use std::borrow::Cow;
use std::hash::{Hash, Hasher};

use w_tokenize::{Kind, Span};

pub use crate::parser::{ParResult, TokenSpan};
pub use module::{parse_module, ParsedModule};

#[derive(Debug, Clone)]
pub struct Ident(pub Span);

pub fn parse(i: TokenSpan) -> ParResult<()> {
    Ok((i, ()))
}

fn parse_keyword(specific: &str) -> impl FnMut(TokenSpan) -> ParResult<Span> + '_ {
    move |i| {
        verify(map(parse_identifier, |id| id.0), |ident| {
            **ident == specific
        })(i)
    }
}

fn parse_identifier(i: TokenSpan) -> ParResult<Ident> {
    let (i, tok) = Weak(Kind::Ident).parse(i)?;
    Ok((i, Ident(tok.span)))
}

fn quick_err<T>(span: TokenSpan, reason: impl Into<Cow<'static, str>>) -> ParResult<T> {
    Err(Err::Error(ErrorChain::from(Error::new(span, reason))))
}

fn parse_name(i: TokenSpan) -> ParResult<Ident> {
    verify(parse_identifier, keyword_check)(i)
}

fn keyword_check(ident: &Ident) -> bool {
    !matches!(
        &**ident.0,
        "struct"
            | "enum"
            | "func"
            | "for"
            | "while"
            | "loop"
            | "if"
            | "else"
            | "mut"
            | "defer"
            | "mod"
    )
}

impl Hash for Ident {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (*self.0).hash(state)
    }
}

impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        (*self.0) == (*other.0)
    }
}
impl Eq for Ident {}
