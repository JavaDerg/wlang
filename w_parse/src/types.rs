use crate::{parse_identifier, Identifier, ParResult, TokenSpan, Weak, parse_name};
use assert_matches::assert_matches;

use nom::branch::alt;
use nom::combinator::{all_consuming, consumed, map, opt, verify};
use nom::multi::{many0, separated_list0};
use nom::sequence::{pair, terminated};
use nom::{Offset, Parser, Slice};

use std::rc::Rc;
use w_tokenize::{Kind, Number};

pub enum Type<'a> {
    Regular(RegularType<'a>),
    Struct(StructType<'a>),
    Enum(EnumType<'a>),
    Tuple(TupleType<'a>),
    Function(FunctionType<'a>),
    Array(ArrayType<'a>),
    Pointer(PointerType<'a>),
    Never(NeverType<'a>),
}

pub struct RegularType<'a> {
    pub span: TokenSpan<'a>,
    pub ty_name: Identifier<'a>,
    pub generics: Vec<Type<'a>>,
}
pub struct StructType<'a>(pub TokenSpan<'a>, pub Vec<(Identifier<'a>, Type<'a>)>);
pub struct EnumType<'a>(pub TokenSpan<'a>, pub Vec<(Identifier<'a>, Option<Type<'a>>)>);
pub struct TupleType<'a>(pub TokenSpan<'a>, pub Vec<Type<'a>>);
pub struct FunctionType<'a> {
    pub span: TokenSpan<'a>,
    pub args: Vec<((Identifier<'a>, bool), Type<'a>)>,
    pub result: Option<Box<Type<'a>>>,
}
pub struct ArrayType<'a> {
    pub span: TokenSpan<'a>,
    pub kind: Box<Type<'a>>,
    pub size: Option<Number<'a>>,
}
pub struct PointerType<'a> {
    pub span: TokenSpan<'a>,
    pub to: Box<Type<'a>>,
    pub mutable: Option<Identifier<'a>>,
}
pub struct NeverType<'a>(pub TokenSpan<'a>);

pub fn parse_type(i: TokenSpan) -> ParResult<Type> {
    alt((
        map(parse_struct_type, Type::Struct),
        map(parse_enum_type, Type::Enum),
        map(parse_function_type, Type::Function),
        map(parse_tuple_type, Type::Tuple),
        map(parse_regular_type, Type::Regular),
        map(parse_pointer_type, Type::Pointer),
        map(parse_never_type, Type::Never),
    ))(i)
}

fn parse_pointer_type(i: TokenSpan) -> ParResult<PointerType> {
    let (i, (span, _)) = consumed(Weak(Kind::Mul))(i)?;
    let (i, mut_) = opt(verify(parse_identifier, |ident| *ident.0 == "mut"))(i)?;
    let (i, ty) = parse_type(i)?;

    Ok((i, PointerType {
        span,
        to: Box::new(ty),
        mutable: mut_,
    }))
}

fn parse_never_type(i: TokenSpan) -> ParResult<NeverType> {
    map(consumed(Weak(Kind::Not)), |(span, _)|  NeverType(span))(i)
}

fn parse_regular_type(oi: TokenSpan) -> ParResult<RegularType> {
    let (i, ty_name) = parse_name(oi.clone())?;
    let (i, generics) = many0(parse_type)(i)?;

    let offset = oi.offset(&i);
    let span = oi.slice(..offset);

    Ok((i, RegularType {
        span,
        ty_name,
        generics
    }))
}

fn parse_struct_type(oi: TokenSpan) -> ParResult<StructType> {
    let (i, _) = verify(Weak(Kind::Ident), |t| *t.span == "struct")(oi.clone())?;

    let (i, block) = Weak(Kind::Block(Rc::from([]))).parse(i)?;
    let block_tokens = assert_matches!(block.kind, Kind::Block(tk) => tk);

    let block_span = TokenSpan::new(i.file, block_tokens);
    let (i, fields) = parse_struct_block(block_span)?;

    let offset = oi.offset(&i);
    let span = oi.slice(..offset);

    Ok((i, StructType(span, fields)))
}

fn parse_struct_block(i: TokenSpan) -> ParResult<Vec<(Identifier, Type)>> {
    all_consuming(terminated(
        separated_list0(Weak(Kind::Comma), pair(parse_name, parse_type)),
        opt(Weak(Kind::Comma)),
    ))(i)
}

fn parse_enum_type(oi: TokenSpan) -> ParResult<EnumType> {
    let (i, _) = verify(Weak(Kind::Ident), |t| *t.span == "enum")(oi.clone())?;

    let (i, block) = Weak(Kind::Block(Rc::from([]))).parse(i)?;
    let block_tokens = assert_matches!(block.kind, Kind::Block(tk) => tk);

    let block_span = TokenSpan::new(i.file, block_tokens);

    let (i, block) = parse_enum_block(block_span)?;

    let offset = oi.offset(&i);
    let span = oi.slice(..offset);

    Ok((i, EnumType(span, block)))
}

fn parse_enum_block(i: TokenSpan) -> ParResult<Vec<(Identifier, Option<Type>)>> {
    all_consuming(terminated(
        separated_list0(Weak(Kind::Comma), pair(parse_name, opt(parse_type))),
        opt(Weak(Kind::Comma)),
    ))(i)
}

fn parse_tuple_type(oi: TokenSpan) -> ParResult<TupleType> {
    let (i, block) = Weak(Kind::Tuple(Rc::from([]))).parse(oi.clone())?;
    let block_tokens = assert_matches!(block.kind, Kind::Tuple(tk) => tk);

    let block_span = TokenSpan::new(i.file, block_tokens);
    let (i, fields) = parse_tuple_inner(block_span)?;

    let offset = oi.offset(&i);
    let span = oi.slice(..offset);

    Ok((i, TupleType(span, fields)))
}

fn parse_tuple_inner(i: TokenSpan) -> ParResult<Vec<Type>> {
    all_consuming(terminated(
        separated_list0(Weak(Kind::Comma), parse_type),
        opt(Weak(Kind::Comma)),
    ))(i)
}

pub(super) fn parse_function_type(oi: TokenSpan) -> ParResult<FunctionType> {
    let (i, _) = verify(Weak(Kind::Ident), |t| *t.span == "func")(oi.clone())?;
    let (i, block) = Weak(Kind::Tuple(Rc::from([]))).parse(i)?;
    let tuple_tokens = assert_matches!(block.kind, Kind::Block(tk) => tk);
    let tuple_span = TokenSpan::new(i.file, tuple_tokens);

    let (i, args) = parse_function_args(tuple_span)?;

    let (i, result) = opt(map(parse_type, Box::new))(i)?;

    let offset = oi.offset(&i);
    let span = oi.slice(..offset);

    Ok((i, FunctionType { span, args, result }))
}

fn parse_function_args(i: TokenSpan) -> ParResult<Vec<((Identifier, bool), Type)>> {
    all_consuming(terminated(
        separated_list0(
            Weak(Kind::Comma),
            pair(
                map(
                    pair(
                        opt(verify(parse_name, |ident| *ident.0 == "mut")),
                        parse_identifier,
                    ),
                    |(m, i)| (i, m.is_some()),
                ),
                parse_type,
            ),
        ),
        opt(Weak(Kind::Comma)),
    ))(i)
}
