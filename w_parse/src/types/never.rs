use crate::{tag, ParResult, TokenSpan};
use nom::combinator::map;
use w_tokenize::Span;

pub struct TyNever<'a>(Span<'a>);

pub fn parse_ty_never(i: TokenSpan) -> ParResult<TyNever> {
    map(tag!(Kind::Not), TyNever)(i)
}
