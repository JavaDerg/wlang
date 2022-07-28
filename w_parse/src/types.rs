use crate::{parse_identifier, Identifier, ParResult, TokenSpan, Weak};
use assert_matches::assert_matches;
use either::Either;
use nom::branch::alt;
use nom::combinator::{all_consuming, map, opt, verify};
use nom::multi::{many0, many1, separated_list0};
use nom::sequence::{pair, terminated};
use nom::Parser;
use std::collections::HashMap;
use std::rc::Rc;
use w_tokenize::{Kind, TokResult, Token};

pub enum Type<'a> {
    Regular(RegularType<'a>),
    Struct(StructType<'a>),
    Enum(EnumType<'a>),
    Tuple(TupleType<'a>),
    Function(FunctionType<'a>),
}

pub struct RegularType<'a>(Vec<Identifier<'a>>);
pub struct StructType<'a>(Vec<(Identifier<'a>, Type<'a>)>);
pub struct EnumType<'a>(
    Vec<(
        Identifier<'a>,
        Option<Either<TupleType<'a>, StructType<'a>>>,
    )>,
);
pub struct TupleType<'a>(Vec<Type<'a>>);
pub struct FunctionType<'a> {
    args: Vec<(Identifier<'a>, Type<'a>)>,
    result: Option<Box<Type<'a>>>,
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
    map(many1(parse_identifier), RegularType)(i)
}


fn parse_struct_type(i: TokenSpan) -> ParResult<StructType> {
    let (i, _) = verify(Weak(Kind::Ident), |t| *t.span == "struct")(i)?;

    let (i, block) = Weak(Kind::Block(Rc::from([]))).parse(i)?;
    let block_tokens = assert_matches!(block.kind, Kind::Block(tk) => tk);

    let block_span = TokenSpan::new(i.file, block_tokens);
    let (i, fields) = parse_struct_block(block_span)?;

    Ok((i, StructType(fields)))
}

fn parse_struct_block(i: TokenSpan) -> ParResult<Vec<(Identifier, Type)>> {
    all_consuming(terminated(
        separated_list0(Weak(Kind::Comma), pair(parse_identifier, parse_type)),
        opt(Weak(Kind::Comma)),
    ))(i)
}


fn parse_enum_type(
    i: TokenSpan,
) -> ParResult<Vec<(Identifier, Option<Either<TupleType, StructType>>)>> {
    let (i, _) = verify(Weak(Kind::Ident), |t| *t.span == "enum")(i)?;

    let (i, block) = Weak(Kind::Block(Rc::from([]))).parse(i)?;
    let block_tokens = assert_matches!(block.kind, Kind::Block(tk) => tk);

    let block_span = TokenSpan::new(i.file, block_tokens);

    parse_enum_block(block_span)
}

fn parse_enum_block(
    i: TokenSpan,
) -> ParResult<Vec<(Identifier, Option<Either<TupleType, StructType>>)>> {
    all_consuming(terminated(
        separated_list0(
            Weak(Kind::Comma),
            pair(
                parse_identifier,
                opt(alt((
                    map(parse_tuple_type, Either::Left),
                    map(parse_struct_type, Either::Right),
                ))),
            ),
        ),
        opt(Weak(Kind::Comma)),
    ))(i)
}


fn parse_tuple_type(i: TokenSpan) -> ParResult<TupleType> {
    let (i, block) = Weak(Kind::Tuple(Rc::from([]))).parse(i)?;
    let block_tokens = assert_matches!(block.kind, Kind::Tuple(tk) => tk);

    let block_span = TokenSpan::new(i.file, block_tokens);
    let (i, fields) = parse_tuple_inner(block_span)?;

    Ok((i, TupleType(fields)))
}

fn parse_tuple_inner(i: TokenSpan) -> ParResult<Vec<Type>> {
    all_consuming(terminated(
        separated_list0(Weak(Kind::Comma), parse_type),
        opt(Weak(Kind::Comma)),
    ))(i)
}


fn parse_function_type(i: TokenSpan) -> ParResult<FunctionType> {
    let (i, _) = verify(Weak(Kind::Ident), |t| *t.span == "func")(i)?;
    let (i, block) = Weak(Kind::Tuple(Rc::from([]))).parse(i)?;
    let tuple_tokens = assert_matches!(block.kind, Kind::Block(tk) => tk);
    let tuple_span = TokenSpan::new(i.file, tuple_tokens);

    let (i, args) = parse_function_args(tuple_span)?;

    let (i, result) = opt(map(parse_type, Box::new))(i)?;

    Ok((i, FunctionType { args, result }))
}

fn parse_function_args(i: TokenSpan) -> ParResult<Vec<(Identifier, Type)>> {
    all_consuming(terminated(
        separated_list0(Weak(Kind::Comma), pair(parse_identifier, parse_type)),
        opt(Weak(Kind::Comma)),
    ))(i)
}
