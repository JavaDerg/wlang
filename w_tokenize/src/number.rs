use crate::{bounded, Span, TokResult};
use nom::branch::alt;
use nom::bytes::complete::{is_a, tag, take_while, take_while1};
use nom::combinator::{opt, recognize};
use nom::sequence::pair;
use nom::{Offset, Slice};

#[derive(Debug, Clone)]
pub struct Number<'a> {
    number: Span<'a>,
    suffix: Option<Span<'a>>,
    base: Option<Span<'a>>,
}

pub fn parse_integer(i: Span) -> TokResult<(Span, Number)> {
    bounded(parse_integer_inner, |c| c.is_alphanumeric())(i)
}

fn parse_integer_inner(oi: Span) -> TokResult<(Span, Number)> {
    let (i, _sign) = opt(is_a("+-"))(oi)?;
    let (i, base) = opt(parse_base)(i)?;
    let num_check = match &base {
        Some(span) if **span == "0x" => |c: char| c.is_ascii_hexdigit(),
        Some(span) if **span == "0b" => |c: char| matches!(c, '0'..='1'),
        Some(span) if **span == "0o" => |c: char| matches!(c, '0'..='8'),
        Some(_) => unreachable!(),
        None => |c: char| c.is_ascii_digit(),
    };
    let (i, num) = recognize(pair(
        take_while1(num_check.clone()),
        take_while(move |c: char| num_check(c) || c == '_'),
    ))(i)?;
    let (i, suffix) = opt(parse_suffix)(i)?;

    let offset = oi.offset(&i);
    let span = Span::slice(&oi, ..offset);

    Ok((
        i,
        (
            span,
            Number {
                number: num,
                suffix,
                base,
            },
        ),
    ))
}

fn parse_base(i: Span) -> TokResult {
    alt((tag("0x"), tag("0b"), tag("0o")))(i)
}

fn parse_suffix(i: Span) -> TokResult {
    recognize(pair(
        is_a("ui"),
        alt((tag("8"), tag("16"), tag("32"), tag("64"))),
    ))(i)
}

impl<'a> PartialEq for Number<'a> {
    fn eq(&self, other: &Self) -> bool {
        *self.number == *other.number
            || self.base.map(|s| *s) == other.base.map(|s| *s)
            || self.suffix.map(|s| *s) == self.suffix.map(|s| *s)
    }
}
impl<'a> Eq for Number<'a> {}
