use crate::expr::{parse_expression, Expr};
use crate::{parse_type, tag, ItemTy, ParResult, TokenSpan};
use nom::combinator::{all_consuming, map, opt};
use nom::{Offset, Slice};
use w_tokenize::Span;

#[derive(Debug, Clone)]
pub struct TyArray<'a> {
    pub span: Span<'a>,
    pub ty: Box<ItemTy<'a>>,
    pub size: Option<Expr<'a>>,
}

pub fn parse_ty_array(oi: TokenSpan) -> ParResult<TyArray> {
    let (i, array) =
        tag!(Kind::Array(_), Token { kind: Kind::Array(vals), .. } => vals)(oi.clone())?;
    let array = TokenSpan::new(i.file, array);

    let (_, size) = all_consuming(opt(parse_expression))(array)?;

    let (i, ty) = map(parse_type, Box::new)(i)?;

    let offset = oi.offset(&i);
    let span = oi.slice(..offset);

    Ok((
        i,
        TyArray {
            span: Span::from(&span),
            ty,
            size,
        },
    ))
}
