use crate::expr::many::{parse_array, ExprArray};
use crate::expr::Expr;
use crate::{ParResult, TokenSpan};

#[derive(Debug, Clone)]
pub struct ExprIndex {
    pub base: Box<Expr>,
    pub args: ExprArray,
}

pub fn parse_index_wrapper(i: TokenSpan) -> ParResult<Box<dyn FnOnce(Expr) -> Expr>> {
    let (i, args) = parse_array(i)?;
    Ok((
        i,
        Box::new(move |expr| {
            Expr::Index(ExprIndex {
                base: Box::new(expr),
                args,
            })
        }),
    ))
}
