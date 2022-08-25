use crate::expr::{parse_expression, Expr};
use crate::{parse_type, tag, ItemTy, ParResult, TokenSpan};
use nom::combinator::{all_consuming, map, opt};
use w_tokenize::Number;

#[derive(Debug, Clone)]
pub struct TyArray<'a> {
    pub ty: Box<ItemTy<'a>>,
    pub size: Option<Expr<'a>>,
}

pub fn parse_ty_array(i: TokenSpan) -> ParResult<TyArray> {
    let (i, array) = tag!(Kind::Array(_), Token { kind: Kind::Array(vals), .. } => vals)(i)?;
    let array = TokenSpan::new(i.file, array);

    let (_, size) = all_consuming(opt(parse_expression))(array)?;

    let (i, ty) = map(parse_type, Box::new)(i)?;

    Ok((i, TyArray { ty, size }))
}
