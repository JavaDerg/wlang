use crate::{parse_name, parse_type, Ident, ItemTy, ParResult, TokenSpan};

pub struct NameTyPair<'a> {
    pub name: Ident<'a>,
    pub ty: ItemTy<'a>,
}

pub fn parse_name_ty_pair(i: TokenSpan) -> ParResult<NameTyPair> {
    let (i, name) = parse_name(i)?;
    let (i, ty) = parse_type(i)?;

    Ok((i, NameTyPair { name, ty }))
}
