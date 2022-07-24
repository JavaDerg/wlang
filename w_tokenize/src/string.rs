use crate::{bounded, Span, ToTokenError, TokResult, TokenError};
use either::Either;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1, take_while_m_n};
use nom::character::complete::char as char_;
use nom::combinator::{value};
use nom::multi::fold_many0;
use nom::sequence::delimited;
use nom::{Err, Offset, Slice};

pub fn parse_string(i: Span) -> TokResult<(Span, String)> {
    bounded(parse_string_inner, |c| c == '"' || c.is_alphanumeric())(i)
}

fn parse_string_inner(i: Span) -> TokResult<(Span, String)> {
    let (ni, o) = delimited(
        char_('"'),
        fold_many0(parse_partial, String::new, |mut str, frag| {
            match frag {
                Either::Left(s) => str.push_str(s),
                Either::Right(c) => str.push(c),
            }
            str
        }),
        char_('"'),
    )(i)?;
    let span_end = i.offset(&ni);
    Ok((ni, (Span::slice(&i, ..span_end), o)))
}

fn parse_partial(i: Span) -> TokResult<Either<&str, char>> {
    if i.len() == 0 {
        return Err(Err::Failure(TokenError::new(i, "Empty string segment")));
    }
    match i.chars().next().unwrap() {
        '"' => Err(Err::Error(TokenError::new(
            Span::slice(&i, ..1),
            "End of string",
        ))),
        '\\' => parse_escape(i).map(|(s, c)| (s, Either::Right(c))),
        _ => take_while1(|c: char| c != '"' && c != '\\')(i).map(|(i, s)| (i, Either::Left(*s))),
    }
}

fn parse_escape(i: Span) -> TokResult<char> {
    alt((
        parse_byte,
        parse_unicode,
        value('\n', tag("\\n")),
        value('\r', tag("\\r")),
        value('\t', tag("\\t")),
        value('\\', tag("\\\\")),
        value('\0', tag("\\0")),
        value('\"', tag("\\\"")),
        value('\'', tag("\\\'")),
    ))(i)
}

fn parse_unicode(oi: Span) -> TokResult<char> {
    let (i, _) = tag("\\")(oi)?;
    let (i, _) = tag("u")(i)?;
    let (i, _) = tag("{")(i).reason("\\u must be followed by '{'")?;
    let (i, hex) = take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit())(i)
        .reason("\\u{} must contain 1-6 hex characters")?;
    let (i, _) = tag("}")(i).reason("\\u{xxxxxx} must terminate with '\'")?;

    let offset = oi.offset(&i);
    let si = Span::slice(&oi, ..offset);

    let val = u32::from_str_radix(*hex, 16).unwrap();
    let val =
        char::try_from(val).map_err(|err| Err::Failure(TokenError::new(si, err.to_string())))?;

    Ok((i, val))
}

fn parse_byte(oi: Span) -> TokResult<char> {
    let (i, _) = tag("\\")(oi)?;
    let (i, _) = tag("x")(i)?;
    let (i, hex) = take_while_m_n(2, 2, |c: char| c.is_ascii_hexdigit())(i)
        .reason("\\x must be followed by 2 hex characters")?;

    let offset = oi.offset(&i);
    let si = Span::slice(&oi, ..offset);

    let val = u8::from_str_radix(*hex, 16).unwrap();
    if val >= 0x80 {
        return Err(Err::Failure(TokenError::new(
            si,
            "Character must be in range of [\\x00-\\x7f]",
        )));
    }

    Ok((i, char::from(val)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_str_eq};

    #[test]
    pub fn basic_test() {
        let span = Span::new("\"Hello world!\"");
        let (l, (s, o)) = parse_string(span).unwrap();

        assert_eq!(&span, &s);
        assert_str_eq!(&o, "Hello world!");
        assert_eq!(l.len(), 0);
    }

    #[test]
    pub fn basic_escape_test() {
        let span = Span::new("\"Hello\\tworld!\"");
        let (l, (s, o)) = parse_string(span).unwrap();

        assert_eq!(&span, &s);
        assert_str_eq!(&o, "Hello\tworld!");
        assert_eq!(l.len(), 0);
    }

    #[test]
    pub fn byte_escape_test() {
        let span = Span::new("\"Hello\\x20world!\"");
        let (l, (s, o)) = parse_string(span).unwrap();

        assert_eq!(&span, &s);
        assert_str_eq!(&o, "Hello world!");
        assert_eq!(l.len(), 0);
    }

    #[test]
    pub fn unicode_escape_test() {
        let span = Span::new("\"Hello\\u{1f3f3}\\u{fe0f}\\u{200d}\\u{1f308}world!\"");
        let (l, (s, o)) = parse_string(span).unwrap();

        assert_eq!(&span, &s);
        assert_str_eq!(&o, "Hello🏳️‍🌈world!");
        assert_eq!(l.len(), 0);
    }
}
