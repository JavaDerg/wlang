use crate::expr::block::{parse_block, ExprBlock};
use crate::types::func::{parse_ty_func, TyFunc};
use crate::{ParResult, TokenSpan};

#[derive(Debug, Clone)]
pub struct ItemFunc<'a> {
    pub func: TyFunc<'a>,
    pub body: ExprBlock<'a>,
}

pub fn parse_item_func(i: TokenSpan) -> ParResult<ItemFunc> {
    let (i, func) = parse_ty_func(i)?;
    let (i, body) = parse_block(i)?;

    Ok((i, ItemFunc { func, body }))
}
