use crate::Span;
use nom::error::{Error, ErrorKind, ParseError};
use nom::{Err, IResult};
use std::borrow::Cow;
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct TokenError<'a> {
    pub span: Span<'a>,
    pub kind: TokenErrorKind<'a>,
    pub reason: Option<Cow<'static, str>>,
}

#[derive(Clone)]
pub enum TokenErrorKind<'a> {
    Nom(ErrorKind),
    Other(Box<TokenError<'a>>),
    NomAndOther(ErrorKind, Box<TokenError<'a>>),
    None,
}

pub trait ToTokenError<Err> {
    type Res;

    fn reason(self, msg: impl Into<Cow<'static, str>>) -> Self::Res;
    fn reason_fn(self, f: impl FnOnce(Err) -> Cow<'static, str>) -> Self::Res;
}

impl<'a> TokenError<'a> {
    pub fn new(input: Span<'a>, msg: impl Into<Cow<'static, str>>) -> Self {
        Self {
            span: input,
            kind: TokenErrorKind::None,
            reason: Some(msg.into()),
        }
    }
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

impl<'a, T> ToTokenError<TokenError<'a>> for IResult<Span<'a>, T, TokenError<'a>> {
    type Res = IResult<Span<'a>, T, TokenError<'a>>;

    fn reason(self, msg: impl Into<Cow<'static, str>>) -> Self::Res {
        if self.is_ok() {
            return self;
        }
        // SAFETY: We check if self is ok before hand
        // This is required as we do not require T to implement Debug
        match unsafe { self.unwrap_err_unchecked() } {
            Err::Incomplete(n) => Err(Err::Incomplete(n)),
            Err::Error(err) => Err(Err::Error(TokenError {
                span: err.span,
                kind: TokenErrorKind::Other(Box::new(err)),
                reason: Some(msg.into()),
            })),
            Err::Failure(err) => Err(Err::Failure(TokenError {
                span: err.span,
                kind: TokenErrorKind::Other(Box::new(err)),
                reason: Some(msg.into()),
            })),
        }
    }

    fn reason_fn(self, f: impl FnOnce(TokenError<'a>) -> Cow<'static, str>) -> Self::Res {
        if self.is_ok() {
            return self;
        }
        // SAFETY: We check if self is ok before hand
        // This is required as we do not require T to implement Debug
        match unsafe { self.unwrap_err_unchecked() } {
            Err::Incomplete(n) => Err(Err::Incomplete(n)),
            Err::Error(err) => Err(Err::Error(TokenError {
                span: err.span,
                kind: TokenErrorKind::Other(Box::new(err.clone())),
                reason: Some(f(err)),
            })),
            Err::Failure(err) => Err(Err::Failure(TokenError {
                span: err.span,
                kind: TokenErrorKind::Other(Box::new(err.clone())),
                reason: Some(f(err)),
            })),
        }
    }
}

impl<'a> From<Error<Span<'a>>> for TokenError<'a> {
    fn from(err: Error<Span<'a>>) -> Self {
        TokenError {
            span: err.input,
            kind: TokenErrorKind::Nom(err.code),
            reason: None,
        }
    }
}

impl<'a> ParseError<Span<'a>> for TokenError<'a> {
    fn from_error_kind(input: Span<'a>, kind: ErrorKind) -> Self {
        Self {
            span: input,
            kind: TokenErrorKind::Nom(kind),
            reason: None,
        }
    }

    fn append(input: Span<'a>, _kind: ErrorKind, other: Self) -> Self {
        Self {
            span: input,
            kind: TokenErrorKind::Other(Box::new(other)),
            reason: None,
        }
    }
}

impl<'a> Debug for TokenError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(r) = &self.reason {
            writeln!(f, "Error {}", r)?;
        }
        writeln!(f, "At {}", self.span)?;

        match &self.kind {
            TokenErrorKind::Nom(err) => write!(f, "Cause: {err:?}")?,
            TokenErrorKind::Other(err) => write!(f, "Cause:\n{:>4?}", &*err)?,
            TokenErrorKind::NomAndOther(nom, err) => {
                write!(f, "Cause: {nom:?}\nAnd:\n{:>4?}", &*err)?
            }
            TokenErrorKind::None => f.write_str("Cause: Unknown")?,
        }

        Ok(())
    }
}
