use nom::branch::alt;
use crate::{parse_name, parse_type, Ident, ParResult, TokenSpan, Type, Weak};
use nom::combinator::{cond, map};
use nom::Parser;
use w_tokenize::{Kind, Token};
use crate::function::{ItemFunc, parse_func};

pub struct ItemDefiner<'a> {
    pub name: Ident<'a>,
    pub kind: DefinerKind<'a>,
}

pub enum DefinerKind<'a> {
    Type(TypeDefiner<'a>),
    Func(ItemFunc<'a>),
}

pub struct TypeDefiner<'a> {
    pub ty: Type<'a>,
    pub terminated: Option<Token<'a>>,
}

pub fn parse_definer(i: TokenSpan) -> ParResult<ItemDefiner> {
    let (i, name) = parse_name(i)?;
    let (i, _) = Weak(Kind::DoubleCol).parse(i)?;

    let (i, kind) = alt((
        map(parse_func, DefinerKind::Func),
        map(parse_type_definer, DefinerKind::Type),
    ))(i)?;

    Ok((i, ItemDefiner {
        name,
        kind,
    }))
}

pub fn parse_type_definer(i: TokenSpan) -> ParResult<TypeDefiner> {
    let (i, ty) = parse_type(i)?;

    let terminated = match &ty {
        Type::Regular(_) => true,
        Type::Struct(_) => false,
        Type::Enum(_) => false,
        Type::Tuple(_) => true,
        Type::Function(_) => true,
        Type::Array(_) => true,
        Type::Pointer(_) => true,
        Type::Never(_) => true,
    };

    let (i, terminated) = cond(terminated, Weak(Kind::Semicolon))(i)?;

    Ok((i, TypeDefiner { ty, terminated }))
}
