use crate::expr::parse_expression;
use crate::{parse_type, tag, Error, ErrorChain, ItemTy, ParResult, TokenSpan};
use either::Either;
use nom::branch::alt;
use nom::combinator::{all_consuming, map, opt, recognize};
use nom::{Offset, Slice};
use w_tokenize::{Number, Span};

#[derive(Debug, Clone)]
pub struct TyArray {
    pub span: Span,
    pub ty: Box<ItemTy>,
    pub size: Option<Box<Number>>,
}

pub fn parse_ty_array(oi: TokenSpan) -> ParResult<TyArray> {
    let (i, array) =
        tag!(Kind::Array(_), Token { kind: Kind::Array(vals), .. } => vals)(oi.clone())?;
    let array = TokenSpan::new(i.file.clone(), array);

    let (_, size) = all_consuming(opt(alt((
        map(
            tag!(Kind::Number(_), Token { kind: Kind::Number(n), .. } => n),
            Either::Left,
        ),
        map(recognize(parse_expression), Either::Right),
    ))))(array)?;
    let size = match size {
        None => None,
        Some(Either::Left(n)) => Some(n),
        Some(Either::Right(expr)) => {
            return Err(nom::Err::Failure(ErrorChain::from(Error::new(
                expr,
                "Constant time expressions are currently not yet supported",
            ))))
        }
    };

    let (i, ty) = map(parse_type, Box::new)(i)?;

    let offset = oi.offset(&i);
    let span = oi.slice(..offset);

    Ok((
        i,
        TyArray {
            span: Span::from(&span),
            ty,
            size,
        },
    ))
}
