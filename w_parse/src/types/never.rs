use crate::{tag, ParResult, TokenSpan};
use nom::combinator::map;
use w_tokenize::Span;

#[derive(Debug, Clone)]
pub struct TyNever(pub Span);

pub fn parse_ty_never(i: TokenSpan) -> ParResult<TyNever> {
    map(tag!(Kind::Not), TyNever)(i)
}
