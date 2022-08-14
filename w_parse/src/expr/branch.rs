use crate::expr::{parse_expression, Expr};
use crate::{parse_keyword, Ident, ParResult, TokenSpan};
use nom::combinator::{map, opt};
use nom::sequence::pair;
use w_tokenize::Span;

pub struct ExprBranch<'a> {
    pub span_if: Span<'a>,
    pub cond: Box<Expr<'a>>,
    pub body: Box<Expr<'a>>,

    pub span_else: Option<Span<'a>>,
    pub body_else: Option<Box<Expr<'a>>>,
}

pub fn parse_branch(i: TokenSpan) -> ParResult<ExprBranch> {
    let (i, span_if) = map(parse_keyword("if"), |id| id.0)(i)?;

    let (i, cond) = parse_expression(i)?;
    let (i, body) = parse_expression(i)?;

    let (i, opt_else) = opt(parse_else)(i)?;

    let (span_else, body_else) =
        opt_else.map_or_else(|| (None, None), |(ie, be)| (Some(ie.0), Some(Box::new(be))));

    Ok((
        i,
        ExprBranch {
            span_if,
            cond: Box::new(cond),
            body: Box::new(body),
            span_else,
            body_else,
        },
    ))
}

fn parse_else(i: TokenSpan) -> ParResult<(Ident, Expr)> {
    pair(parse_keyword("else"), parse_expression)(i)
}
