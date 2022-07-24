use nom::character::complete::char;
use nom::combinator::recognize;
use nom::{IResult, Offset, Slice};
use nom::multi::fold_many0;
use nom::sequence::delimited;
use crate::Span;

pub fn parse_string<X>(i: Span<X>) -> IResult<Span<X>, (Span<X>, String)> {
    let (ni, o) = delimited(char('"'), fold_many0(
        parse_partial,
        String::new,
        |mut str, frag| {
            todo!()
            str
        }
    ), char('"'))(i)?;
    let span_end = i.offset(&ni);
    Ok((ni, (Span::slice(&i, ..span_end), o)))
}

fn parse_partial<X>(i: Span<X>) -> IResult<Span<X>, > {

}
