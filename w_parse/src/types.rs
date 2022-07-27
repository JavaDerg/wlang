use std::collections::HashMap;
use std::rc::Rc;
use nom::branch::alt;
use nom::combinator::{map, verify};
use nom::multi::many1;
use nom::Parser;
use w_tokenize::{Kind, TokResult};
use crate::{Identifier, ParResult, TokenSpan, Weak};
use assert_matches::assert_matches;

pub enum Type<'a> {
    Regular(RegularType<'a>),
    Struct(StructType<'a>),
    Tuple(TupleType<'a>),
    Function(FunctionType<'a>),
}

pub struct RegularType<'a>(Vec<Identifier<'a>>);
pub struct StructType<'a>(Vec<(Identifier<'a>, Type<'a>)>);
pub struct TupleType<'a>(Vec<Type<'a>>);
pub struct FunctionType<'a> {
    args: Vec<(Identifier<'a>, RegularType<'a>)>,
    result: RegularType<'a>,
}

pub fn parse_type(i: TokenSpan) -> ParResult<Type> {
    alt((
        map(parse_struct_type, Type::Struct),
        map(parse_function_type, Type::Function),
        map(parse_tuple_type, Type::Tuple),
        map(parse_regular_type, Type::Regular),
    ))(i)
}

fn parse_regular_type(i: TokenSpan) -> ParResult<RegularType> {
    map(many1(Weak(Kind::Ident)), RegularType)(i)
}


fn parse_struct_type(i: TokenSpan) -> ParResult<StructType> {
    let (i, _) = verify(Weak(Kind::Ident), |t| *t.span == "struct")(i)?;
    let (i, block) = Weak(Kind::Block(Rc::from([]))).parse(i)?;
    let block_tokens = assert_matches!(block.kind, Kind::Block(tk) => tk);

}

fn parse_block(i: TokenSpan) -> ParResult<Vec<(Identifier, Type)>> {
    todo!()
}


fn parse_tuple_type(i: TokenSpan) -> ParResult<TupleType> {
    todo!()
}

fn parse_function_type(i: TokenSpan) -> ParResult<FunctionType> {
    todo!()
}
