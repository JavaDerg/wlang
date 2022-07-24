use nom::bytes::complete::tag;
use nom::IResult;

mod identifier;
mod r#type;
mod string;

type Span<'a, X> = nom_locate::LocatedSpan<&'a str, X>;

pub struct Token<'a, X> {
    pub span: Span<'a, X>,
    pub kind: Kind,
}

pub enum Kind {
    Ident,
    Defines,
}

pub fn defines<X>(i: Span<X>) -> IResult<Span<X>, ()> {
    tag("::")(i).map(|(o, _)| (o, ()))
}
