use crate::{Span, TokResult};
use nom::error::{Error, ErrorKind, ParseError};
use nom::{Err, IResult, Parser};
use std::borrow::Cow;
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct TokenError {
    pub span: Span,
    pub kind: TokenErrorKind,
    pub reason: Option<Cow<'static, str>>,
}

#[derive(Clone)]
pub enum TokenErrorKind {
    Nom(ErrorKind),
    Other(Box<TokenError>),
    NomAndOther(ErrorKind, Box<TokenError>),
    None,
}

pub trait ToTokenError<Err> {
    type Res;

    fn reason(self, msg: impl Into<Cow<'static, str>>) -> Self::Res;
    fn reason_fn(self, f: impl FnOnce(Err) -> Cow<'static, str>) -> Self::Res;
}

impl TokenError {
    pub fn new(input: Span, msg: impl Into<Cow<'static, str>>) -> Self {
        Self {
            span: input,
            kind: TokenErrorKind::None,
            reason: Some(msg.into()),
        }
    }
}

pub fn reason<F, O>(
    mut parser: F,
    msg: impl Into<Cow<'static, str>> + Clone,
) -> impl FnMut(Span) -> TokResult<O>
where
    F: Parser<Span, O, TokenError>,
{
    move |i| parser.parse(i).reason(msg.clone())
}

// impl<'a, T> ToTokenError<Error<Span<'a>>> for IResult<Span<'a>, T> {
//     type Res = IResult<Span<'a>, T, TokenError<'a>>;
//
//     fn reason(self, msg: impl Into<Cow<'static, str>>) -> Self::Res {
//         if self.is_ok() {
//             return self.map_err(|_| unreachable!());
//         }
//         // SAFETY: We check if self is ok before hand
//         // This is required as we do not require T to implement Debug
//         let err = unsafe { self.unwrap_err_unchecked() };
//         match err {
//             Err::Incomplete(n) => Err(Err::Incomplete(n)),
//             Err::Error(err) => Err(Err::Error(TokenError {
//                 span: err.input,
//                 kind: TokenErrorKind::Nom(err.code),
//                 reason: Some(msg.into()),
//             })),
//             Err::Failure(err) => Err(Err::Failure(TokenError {
//                 span: err.input,
//                 kind: TokenErrorKind::Nom(err.code),
//                 reason: Some(msg.into()),
//             })),
//         }
//     }
//
//     fn reason_fn(self, f: impl FnOnce(Error<Span<'a>>) -> (Cow<'static, str>)) -> Self::Res {
//         if self.is_ok() {
//             return self.map_err(|_| unreachable!());
//         }
//         // SAFETY: We check if self is ok before hand
//         // This is required as we do not require T to implement Debug
//         match unsafe { self.unwrap_err_unchecked() } {
//             Err::Incomplete(n) => Err(Err::Incomplete(n)),
//             Err::Error(err) => Err(Err::Error(TokenError {
//                 span: err.input.clone(),
//                 kind: TokenErrorKind::Nom(err.code.clone()),
//                 reason: Some(f(err)),
//             })),
//             Err::Failure(err) => Err(Err::Failure(TokenError {
//                 span: err.input.clone(),
//                 kind: TokenErrorKind::Nom(err.code.clone()),
//                 reason: Some(f(err)),
//             })),
//         }
//     }
// }

impl<T> ToTokenError<TokenError> for IResult<Span, T, TokenError> {
    type Res = IResult<Span, T, TokenError>;

    fn reason(self, msg: impl Into<Cow<'static, str>>) -> Self::Res {
        if self.is_ok() {
            return self;
        }
        // SAFETY: We check if self is ok before hand
        // This is required as we do not require T to implement Debug
        match unsafe { self.unwrap_err_unchecked() } {
            Err::Incomplete(n) => Err(Err::Incomplete(n)),
            Err::Error(err) => Err(Err::Error(TokenError {
                span: err.span.clone(),
                kind: TokenErrorKind::Other(Box::new(err)),
                reason: Some(msg.into()),
            })),
            Err::Failure(err) => Err(Err::Failure(TokenError {
                span: err.span.clone(),
                kind: TokenErrorKind::Other(Box::new(err)),
                reason: Some(msg.into()),
            })),
        }
    }

    fn reason_fn(self, f: impl FnOnce(TokenError) -> Cow<'static, str>) -> Self::Res {
        if self.is_ok() {
            return self;
        }
        // SAFETY: We check if self is ok before hand
        // This is required as we do not require T to implement Debug
        match unsafe { self.unwrap_err_unchecked() } {
            Err::Incomplete(n) => Err(Err::Incomplete(n)),
            Err::Error(err) => Err(Err::Error(TokenError {
                span: err.span.clone(),
                kind: TokenErrorKind::Other(Box::new(err.clone())),
                reason: Some(f(err)),
            })),
            Err::Failure(err) => Err(Err::Failure(TokenError {
                span: err.span.clone(),
                kind: TokenErrorKind::Other(Box::new(err.clone())),
                reason: Some(f(err)),
            })),
        }
    }
}

impl From<Error<Span>> for TokenError {
    fn from(err: Error<Span>) -> Self {
        TokenError {
            span: err.input,
            kind: TokenErrorKind::Nom(err.code),
            reason: Some("Failed to obey noms parsing rules :(".into()),
        }
    }
}

impl ParseError<Span> for TokenError {
    fn from_error_kind(input: Span, kind: ErrorKind) -> Self {
        Self {
            span: input,
            kind: TokenErrorKind::Nom(kind),
            reason: Some("Failed to obey noms parsing rules :(".into()),
        }
    }

    fn append(input: Span, _kind: ErrorKind, other: Self) -> Self {
        if other.span == input {
            other
        } else {
            Self {
                span: input,
                kind: TokenErrorKind::Other(Box::new(other)),
                reason: None,
            }
        }
    }
}

impl Debug for TokenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            TokenErrorKind::Nom(err) => writeln!(f, "Error: {err:?}\nCaused in:")?,
            TokenErrorKind::Other(err) => writeln!(f, "Error: {:?}\nCaused by:", err)?,
            TokenErrorKind::NomAndOther(nom, err) => {
                write!(f, "Error: {nom:?}\nCaused by:\n{:?}", err)?
            }
            TokenErrorKind::None => f.write_str("Cause: Unknown")?,
        }
        if let Some(r) = &self.reason {
            writeln!(f, "Error {}", r)?;
        }
        writeln!(
            f,
            "At {} {}:\n---------------------------\n{}\n---------------------------",
            self.span.location_line(),
            self.span.location_offset(),
            self.span
        )?;

        Ok(())
    }
}
