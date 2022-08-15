use nom::combinator::all_consuming;
use w_tokenize::Span;
use crate::{ItemTy, ParResult, parse_type, tag, TokenSpan};
use crate::expr::parse_many0;

pub struct TyTuple<'a> {
    pub span: Span<'a>,
    pub types: Vec<ItemTy<'a>>,
}

pub fn parse_ty_tuple(i: TokenSpan) -> ParResult<TyTuple> {
    let (i, (span, tuple)) = tag!(Kind::Tuple(_), Token { kind: Kind::Tuple(vals), span } => (span, vals))(i)?;
    let tuple = TokenSpan::new(i.file.clone(), tuple);

    let (_, types) = all_consuming(parse_many0(parse_type))(tuple)?;

    Ok((i, TyTuple { span, types }))
}
