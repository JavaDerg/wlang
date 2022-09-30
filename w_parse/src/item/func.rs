use crate::expr::block::{parse_block, ExprBlock};
use crate::types::func::{parse_ty_named_func, TyNamedFunc};
use crate::{ParResult, TokenSpan};

#[derive(Debug, Clone)]
pub struct ItemFunc {
    pub func: TyNamedFunc,
    pub body: ExprBlock,
}

pub fn parse_item_func(i: TokenSpan) -> ParResult<ItemFunc> {
    let (i, func) = parse_ty_named_func(i)?;
    let (i, body) = parse_block(i)?;

    Ok((i, ItemFunc { func, body }))
}
