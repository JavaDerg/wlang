use crate::expr::Expr;
use crate::{ParResult, TokenSpan, Weak};
use nom::branch::alt;
use nom::combinator::map;
use w_tokenize::{Kind, Span};

#[derive(Debug, Clone)]
pub struct ExprUnary {
    pub op: UnOp,
    pub expr: Box<Expr>,
}

#[derive(Debug, Clone)]
pub enum UnOp {
    Ref(Span),
    Deref(Span),
    Not(Span),
}

pub fn parse_unary(i: TokenSpan) -> ParResult<UnOp> {
    map(
        alt((Weak(Kind::And), Weak(Kind::Mul), Weak(Kind::Not))),
        |tk| {
            (match tk.kind {
                Kind::And => UnOp::Ref,
                Kind::Mul => UnOp::Deref,
                Kind::Not => UnOp::Not,
                _ => unreachable!(),
            })(tk.span)
        },
    )(i)
}
