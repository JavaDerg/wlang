use crate::expr::parse_many0;
use crate::{parse_type, tag, ItemTy, ParResult, TokenSpan};
use nom::combinator::all_consuming;
use w_tokenize::Span;

#[derive(Debug, Clone)]
pub struct TyTuple {
    pub span: Span,
    pub types: Vec<ItemTy>,
}

pub fn parse_ty_tuple(i: TokenSpan) -> ParResult<TyTuple> {
    let (i, (span, tuple)) =
        tag!(Kind::Tuple(_), Token { kind: Kind::Tuple(vals), span } => (span, vals))(i)?;
    let tuple = TokenSpan::new(i.file.clone(), tuple);

    let (_, types) = all_consuming(parse_many0(parse_type))(tuple)?;

    Ok((i, TyTuple { span, types }))
}
