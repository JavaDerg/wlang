use std::borrow::Cow;
use std::fmt::{Debug, Formatter};
use nom::error::{ErrorKind, ParseError};
use w_tokenize::{Span, Token};
use crate::parser::TokenSpan;

pub struct ErrorChain<'token, 'slice> {
    err_acc: Vec<Error<'token, 'slice>>,
}

pub struct Error<'a, 'b> {
    pub location: TokenSpan<'a, 'b>,
    pub reason: Cow<'static, str>,
}

impl<'a, 'b> Error<'a, 'b> {
    pub fn new(location: TokenSpan<'a, 'b>, reason: impl Into<Cow<'static, str>>) -> Self {
        Self {
            location,
            reason: reason.into(),
        }
    }
}

impl<'a, 'b> ErrorChain<'a, 'b> {
    pub fn has_errs(&self) -> bool {
        !self.err_acc.is_empty()
    }

    pub fn put_errs(&mut self, mut other: ErrorChain<'a, 'b>) {
        self.err_acc.append(&mut other.err_acc);
    }
}

impl<'a, 'b> From<Error<'a, 'b>> for ErrorChain<'a, 'b> {
    fn from(err: Error<'a, 'b>) -> Self {
        Self {
            err_acc: vec![err],
        }
    }
}

impl<'a, 'b> ParseError<TokenSpan<'a, 'b>> for ErrorChain<'a, 'b> {
    fn from_error_kind(input: TokenSpan<'a, 'b>, kind: ErrorKind) -> Self {
        Self {
            err_acc: vec![Error::new(input, format!("{:?}", kind))],
        }
    }

    fn append(input: TokenSpan<'a, 'b>, kind: ErrorKind, mut other: Self) -> Self {
        other.err_acc.push(Error::new(input, format!("{:?}", kind)));
        other
    }
}

impl<'a, 'b> Debug for ErrorChain<'a, 'b> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for err in &self.err_acc {
            writeln!(f, "{:?}", err)?;
        }
        Ok(())
    }
}

impl<'a, 'b> Debug for Error<'a, 'b> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Error: {}", self.reason)?;
        writeln!(f, "At {} {}:\n---------------------------\n{}\n---------------------------",
            "???",
            "???",
            "???",
        )?;
        Ok(())
    }
}
