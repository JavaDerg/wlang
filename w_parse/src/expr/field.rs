use crate::expr::{parse_expression, Expr};
use crate::{parse_identifier, parse_name, Ident, ParResult, TokenSpan, Weak};
use either::Either;
use nom::combinator::opt;
use nom::sequence::pair;
use w_tokenize::{Kind, Span};

pub struct ExprField<'a> {
    pub base: Box<Expr<'a>>,
    pub dot: Span<'a>,
    pub field: Ident<'a>,
}

pub fn parse_field_wrapper<'a>(
    i: TokenSpan<'a>,
) -> ParResult<'a, Box<dyn FnOnce(Expr<'a>) -> Expr<'a> + 'a>> {
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
