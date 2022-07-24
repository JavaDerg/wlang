use crate::string::parse_string;
use crate::{bounded, Span, TokResult};
use nom::bytes::complete::tag;
use nom::combinator::{cond, opt};
use nom::IResult;

pub fn parse_type(i: Span) -> TokResult {
    let (i, ext) = opt(extern_)(i)?;
    let (i, mode) = cond(ext.is_some(), opt(parse_string))(i).map(|(i, s)| (i, s.flatten()))?;



    todo!()
}

pub struct FunctionInfo<'a> {
    pub func: Span<'a>,
    pub args: Vec<(Span<'a>, Span<'a>)>,
    pub returns: Vec<Span<'a>>,
}

fn parse_function(i: Span) -> TokResult {

}

fn extern_(i: Span) -> TokResult {
    bounded(tag("extern"), char::is_alphanumeric)(i)
}
