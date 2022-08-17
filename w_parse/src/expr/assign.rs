use crate::expr::{parse_expression, Expr};
use crate::{tag, ParResult, TokenSpan};
use nom::combinator::map;
use w_tokenize::Span;

#[derive(Debug, Clone)]
pub struct ExprAssignment<'a> {
    pub span_op: Span<'a>,
    pub assignee: Box<Expr<'a>>,
    pub value: Box<Expr<'a>>,
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
