use crate::func::{parse_item_func, ItemFunc};
use crate::{parse_name, parse_type, Ident, ItemTy, ParResult, TokenSpan, Weak};
use nom::branch::alt;
use nom::combinator::{cond, map};
use nom::Parser;
use w_tokenize::{Kind, Token};

pub struct ItemDefiner<'a> {
    pub name: Ident<'a>,
    pub kind: DefinerKind<'a>,
}

pub enum DefinerKind<'a> {
    Type(TypeDefiner<'a>),
    Func(ItemFunc<'a>),
}

pub struct TypeDefiner<'a> {
    pub ty: ItemTy<'a>,
    pub terminated: Option<Token<'a>>,
}

pub fn parse_definer(i: TokenSpan) -> ParResult<ItemDefiner> {
    let (i, name) = parse_name(i)?;
    let (i, _) = Weak(Kind::DoubleCol).parse(i)?;

    let (i, kind) = alt((
        map(parse_item_func, DefinerKind::Func),
        map(parse_type_definer, DefinerKind::Type),
    ))(i)?;

    Ok((i, ItemDefiner { name, kind }))
}

pub fn parse_type_definer(i: TokenSpan) -> ParResult<TypeDefiner> {
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

    Ok((i, TypeDefiner { ty, terminated }))
}
