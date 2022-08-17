use nom::combinator::map;
use w_tokenize::Span;
use crate::expr::{Expr, parse_expression};
use crate::{ParResult, tag, TokenSpan};

#[derive(Debug, Clone)]
pub struct ExprDefine<'a> {
    pub span_op: Span<'a>,
    pub assignee: Box<Expr<'a>>,
    pub value: Box<Expr<'a>>,
}

pub fn parse_define(i: TokenSpan) -> ParResult<ExprDefine> {
    let (i, assignee) = map(parse_expression, Box::new)(i)?;
    let (i, span_op) = tag!(Kind::Define)(i)?;
    let (i, value) = map(parse_expression, Box::new)(i)?;

    Ok((i, ExprDefine {
        span_op,
        assignee,
        value,
    }))
}
