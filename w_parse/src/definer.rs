use nom::combinator::{cond};
use nom::multi::many0;
use nom::Parser;
use w_tokenize::{Kind, Token};
use crate::{Identifier, ParResult, parse_identifier, parse_name, parse_type, TokenSpan, Type, Weak};

pub struct TypeDefiner<'a> {
    pub name: Identifier<'a>,
    pub generics: Vec<Identifier<'a>>,
    pub ty: Type<'a>,
    pub terminated: Option<Token<'a>>,
}

pub fn parse_type_definer(i: TokenSpan) -> ParResult<TypeDefiner> {
    let (i, name) = parse_name(i)?;
    let (i, generics) = many0(parse_identifier)(i)?;
    let (i, _) = Weak(Kind::DoubleCol).parse(i)?;
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

    let (i, sim) = cond(terminated, Weak(Kind::Semicolon))(i)?;

    Ok((i, TypeDefiner {
        name,
        generics,
        ty,
        terminated: sim,
    }))
}

