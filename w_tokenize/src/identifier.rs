use crate::{bounded, Span, TokResult};
use nom::bytes::complete::{take_while, take_while1};
use nom::combinator::{recognize, verify};

pub fn parse_ident(i: Span) -> TokResult {
    bounded(
        verify(recognize(inner_span), |s| **s != "_"),
        char::is_alphanumeric,
    )(i)
}

fn inner_span(i: Span) -> TokResult<()> {
    let (i, _) = take_while1(|c: char| c.is_alphabetic() || c == '_')(i)?;
    let (i, _) = take_while(|c: char| c.is_alphanumeric() || c == '_')(i)?;

    Ok((i, ()))
}
