use crate::expr::Expr;
use crate::{parse_name, Ident, ParResult, TokenSpan, Weak};

use nom::sequence::pair;
use w_tokenize::{Kind, Span};

#[derive(Debug, Clone)]
pub struct ExprField {
    pub base: Box<Expr>,
    pub dot: Span,
    pub field: Ident,
}

pub fn parse_field_wrapper(i: TokenSpan) -> ParResult<Box<dyn FnOnce(Expr) -> Expr>> {
    let (i, (tk, ident)) = pair(Weak(Kind::Dot), parse_name)(i)?;
    Ok((
        i,
        Box::new(move |expr| {
            Expr::Field(ExprField {
                base: Box::new(expr),
                dot: tk.span,
                field: ident,
            })
        }),
    ))
}
