use std::rc::Rc;
use assert_matches::assert_matches;
use nom::combinator::all_consuming;
use nom::multi::many0;
use nom::Parser;
use w_tokenize::Kind;
use crate::{Identifier, ParResult, parse_identifier, parse_name, TokenSpan, Weak};
use crate::types::parse_function_type;

pub fn parse_function(oi: TokenSpan) -> ParResult {
    let (i, _name) = parse_name(oi.clone())?;
    let (i, _generics) = many0(parse_identifier)(i)?;
    let (i, _) = Weak(Kind::DoubleCol).parse(i)?;
    let (_i, _func_head) = parse_function_type(i)?;


    todo!()
}

pub enum Expression<'a> {
    Scoped(CodeBlock<'a>),
    Assignment {
        target: Identifier<'a>,
        value: Box<Expression<'a>>,
    }
}

pub struct CodeBlock<'a>(Vec<Expression<'a>>);

pub fn parse_code_block(i: TokenSpan) -> ParResult<CodeBlock> {
    let (i, block) = Weak(Kind::Block(Rc::from([]))).parse(i)?;
    let block_tokens = assert_matches!(block.kind, Kind::Block(tk) => tk);

    let block_span = TokenSpan::new(i.file, block_tokens);

    all_consuming(parse_code_block_inner)(block_span).map(|(_, r)| (i, r))
}

pub fn parse_code_block_inner(_i: TokenSpan) -> ParResult<CodeBlock> {
    todo!()
}
