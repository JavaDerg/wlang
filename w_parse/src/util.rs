use crate::{parse_name, parse_type, Ident, ItemTy, ParResult, TokenSpan};

#[derive(Debug, Clone)]
pub struct NameTyPair {
    pub name: Ident,
    pub ty: ItemTy,
}

pub fn parse_name_ty_pair(i: TokenSpan) -> ParResult<NameTyPair> {
    let (i, name) = parse_name(i)?;
    let (i, ty) = parse_type(i)?;

    Ok((i, NameTyPair { name, ty }))
}
