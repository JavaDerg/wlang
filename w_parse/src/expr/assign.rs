use crate::expr::{parse_expression, Expr};
use crate::{tag, ParResult, TokenSpan};
use nom::combinator::map;
use w_tokenize::Span;

#[derive(Debug, Clone)]
pub struct ExprAssignment {
    pub span_op: Span,
    pub assignee: Box<Expr>,
    pub value: Box<Expr>,
}

pub fn parse_assignment(i: TokenSpan) -> ParResult<ExprAssignment> {
    let (i, assignee) = map(parse_expression, Box::new)(i)?;
    let (i, span_op) = tag!(Kind::Assign)(i)?;
    let (i, value) = map(parse_expression, Box::new)(i)?;

    Ok((
        i,
        ExprAssignment {
            span_op,
            assignee,
            value,
        },
    ))
}
