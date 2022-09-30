use crate::expr::block::{parse_block, ExprBlock};
use crate::expr::{parse_expression, Expr};
use crate::{parse_keyword, ParResult, TokenSpan};
use nom::combinator::map;
use w_tokenize::Span;

#[derive(Debug, Clone)]
pub struct ExprWhile {
    pub span_while: Span,
    pub cond: Box<Expr>,
    pub body: ExprBlock,
}

pub fn parse_while(i: TokenSpan) -> ParResult<ExprWhile> {
    let (i, span_while) = parse_keyword("while")(i)?;
    let (i, cond) = map(parse_expression, Box::new)(i)?;
    let (i, body) = parse_block(i)?;

    Ok((
        i,
        ExprWhile {
            span_while,
            cond,
            body,
        },
    ))
}
