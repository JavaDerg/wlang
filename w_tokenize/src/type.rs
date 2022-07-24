use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::IResult;
use crate::Span;

pub fn type_<X>(i: Span<X>) -> IResult<Span<X>, Span<X>> {
    let (i, ext) = opt(extern_)(i)?;

}

fn extern_<X>(i: Span<X>) -> IResult<Span<X>, Span<X>> {
    tag("extern")(i)
}
