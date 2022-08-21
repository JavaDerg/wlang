use crate::item::func::{parse_item_func, ItemFunc};
use crate::{parse_name, parse_type, Ident, ItemTy, ParResult, TokenSpan, Weak};
use nom::branch::alt;
use nom::combinator::{cond, map};
use nom::Parser;
use w_tokenize::{Kind, Token};

#[derive(Debug, Clone)]
pub struct ItemNamed<'a> {
    pub name: Ident<'a>,
    pub kind: NamedKind<'a>,
}

#[derive(Debug, Clone)]
pub enum NamedKind<'a> {
    Type(ItemNamedType<'a>),
    Func(ItemFunc<'a>),
}

#[derive(Debug, Clone)]
pub struct ItemNamedType<'a> {
    pub ty: ItemTy<'a>,
    pub terminated: Option<Token<'a>>,
}

pub fn parse_named(i: TokenSpan) -> ParResult<ItemNamed> {
    let (i, name) = parse_name(i)?;
    let (i, _) = Weak(Kind::DoubleCol).parse(i)?;

    let (i, kind) = alt((
        map(parse_item_func, NamedKind::Func),
        map(parse_type_definer, NamedKind::Type),
    ))(i)?;

    Ok((i, ItemNamed { name, kind }))
}

pub fn parse_type_definer(i: TokenSpan) -> ParResult<ItemNamedType> {
    let (i, ty) = parse_type(i)?;

    let terminated = match &ty {
        ItemTy::Named(_) => true,
        ItemTy::Struct(_) => false,
        ItemTy::Enum(_) => false,
        ItemTy::Tuple(_) => true,
        ItemTy::Func(_) => true,
        ItemTy::Array(_) => true,
        ItemTy::Pointer(_) => true,
        ItemTy::Never(_) => true,
    };

    let (i, terminated) = cond(terminated, Weak(Kind::Semicolon))(i)?;

    Ok((i, ItemNamedType { ty, terminated }))
}
