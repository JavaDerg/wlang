use crate::expr::block::{parse_block, ExprBlock};
use crate::expr::{parse_expression, Expr};
use crate::{parse_keyword, ParResult, TokenSpan};
use nom::combinator::opt;
use nom::sequence::pair;
use w_tokenize::Span;

#[derive(Debug, Clone)]
pub struct ExprBranch {
    pub span_if: Span,
    pub cond: Box<Expr>,
    pub body: ExprBlock,

    pub span_else: Option<Span>,
    pub body_else: Option<ExprBlock>,
}

pub fn parse_branch(i: TokenSpan) -> ParResult<ExprBranch> {
    let (i, span_if) = parse_keyword("if")(i)?;

    let (i, cond) = parse_expression(i)?;
    let (i, body) = parse_block(i)?;

    let (i, opt_else) = opt(parse_else)(i)?;

    let (span_else, body_else) =
        opt_else.map_or_else(|| (None, None), |(ie, be)| (Some(ie), Some(be)));

    Ok((
        i,
        ExprBranch {
            span_if,
            cond: Box::new(cond),
            body,
            span_else,
            body_else,
        },
    ))
}

fn parse_else(i: TokenSpan) -> ParResult<(Span, ExprBlock)> {
    pair(parse_keyword("else"), parse_block)(i)
}
