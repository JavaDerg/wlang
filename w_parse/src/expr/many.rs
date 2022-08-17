use crate::expr::{parse_expression, Expr};
use crate::{parse_name, ErrorChain, Ident, ParResult, TokenSpan, Weak};
use assert_matches::assert_matches;
use nom::combinator::{all_consuming, map, opt};
use nom::multi::separated_list0;
use nom::sequence::{terminated, tuple};
use nom::Parser;
use std::rc::Rc;
use w_tokenize::{Kind, Span};

#[derive(Debug, Clone)]
pub struct ExprTuple<'a> {
    pub span: Span<'a>,
    pub values: Vec<Expr<'a>>,
}

#[derive(Debug, Clone)]
pub struct ExprArray<'a> {
    pub span: Span<'a>,
    pub values: Vec<Expr<'a>>,
}

#[derive(Debug, Clone)]
pub struct ExprObject<'a> {
    pub span: Span<'a>,
    pub values: Vec<(Ident<'a>, Expr<'a>)>,
}

pub fn parse_tuple(i: TokenSpan) -> ParResult<ExprTuple> {
    let (i, tuple) = Weak(Kind::Tuple(Rc::from([]))).parse(i)?;
    let span = tuple.span;
    let tuple = assert_matches!(tuple.kind, Kind::Tuple(vals) => TokenSpan::new(i.file, vals));
    let (_, vals) = all_consuming(parse_many0(parse_expression))(tuple)?;

    Ok((i, ExprTuple { span, values: vals }))
}

pub fn parse_array(i: TokenSpan) -> ParResult<ExprArray> {
    let (i, array) = Weak(Kind::Array(Rc::from([]))).parse(i)?;
    let span = array.span;
    let array = assert_matches!(array.kind, Kind::Array(vals) => TokenSpan::new(i.file, vals));
    let (_, vals) = all_consuming(parse_many0(parse_expression))(array)?;

    Ok((i, ExprArray { span, values: vals }))
}

pub fn parse_object(i: TokenSpan) -> ParResult<ExprObject> {
    let (i, block) = Weak(Kind::Block(Rc::from([]))).parse(i)?;
    let span = block.span;
    let block = assert_matches!(block.kind, Kind::Block(vals) => TokenSpan::new(i.file, vals));
    let (_, vals) = all_consuming(parse_many0(map(
        tuple((parse_name, Weak(Kind::Assign), parse_expression)),
        |(k, _, v)| (k, v),
    )))(block)?;

    Ok((i, ExprObject { span, values: vals }))
}

pub fn parse_many0<'a, F, T: 'a>(parser: F) -> impl FnMut(TokenSpan<'a>) -> ParResult<'a, Vec<T>>
where
    F: Parser<TokenSpan<'a>, T, ErrorChain<'a>>,
{
    terminated(
        separated_list0(Weak(Kind::Comma), parser),
        opt(Weak(Kind::Comma)),
    )
}
