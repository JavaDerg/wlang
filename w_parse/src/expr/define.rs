use crate::expr::{parse_expression, Expr};
use crate::{tag, ParResult, TokenSpan};
use nom::combinator::map;
use w_tokenize::Span;

#[derive(Debug, Clone)]
pub struct ExprDefine {
    pub span_op: Span,
    pub assignee: Box<Expr>,
    pub value: Box<Expr>,
}

pub fn parse_define(i: TokenSpan) -> ParResult<ExprDefine> {
    let (i, assignee) = map(parse_expression, Box::new)(i)?;
    let (i, span_op) = tag!(Kind::Define)(i)?;
    let (i, value) = map(parse_expression, Box::new)(i)?;

    Ok((
        i,
        ExprDefine {
            span_op,
            assignee,
            value,
        },
    ))
}
