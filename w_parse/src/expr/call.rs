use crate::expr::many::{parse_tuple, ExprTuple};
use crate::expr::Expr;
use crate::{ParResult, TokenSpan};

#[derive(Debug, Clone)]
pub struct ExprCall<'a> {
    pub base: Box<Expr<'a>>,
    pub args: ExprTuple<'a>,
}

pub fn parse_call_wrapper<'a>(
    i: TokenSpan<'a>,
) -> ParResult<'a, Box<dyn FnOnce(Expr<'a>) -> Expr<'a> + 'a>> {
    let (i, args) = parse_tuple(i)?;
    Ok((
        i,
        Box::new(move |expr| {
            Expr::Call(ExprCall {
                base: Box::new(expr),
                args,
            })
        }),
    ))
}
