use crate::expr::many::{parse_array, parse_tuple, ExprArray, ExprTuple};
use crate::expr::Expr;
use crate::{ParResult, TokenSpan};
use nom::sequence::pair;

pub struct ExprIndex<'a> {
    pub base: Box<Expr<'a>>,
    pub args: ExprArray<'a>,
}

pub fn parse_index_wrapper<'a>(
    i: TokenSpan<'a>,
) -> ParResult<'a, Box<dyn FnOnce(Expr<'a>) -> Expr<'a> + 'a>> {
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
