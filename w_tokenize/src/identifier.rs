use nom::bytes::complete::{take_while, take_while1};
use nom::combinator::{opt, recognize};
use nom::IResult;
use crate::Span;

pub fn ident<X>(i: Span<X>) -> IResult<Span<X>, Span<X>> {
    recognize(inner_span)(i)
}

fn inner_span<X>(i: Span<X>) -> IResult<Span<X>, ()> {
    let (i, _) = take_while1(char::is_alphabetic)(i)?;
    let (i, _) = take_while(char::is_alphanumeric)(i)?;

    Ok((i, ()))
}
