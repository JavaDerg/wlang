use crate::error::{Error, ErrorChain};
use nom::bytes::complete::tag;
use nom::{Compare, CompareResult, Err, IResult, InputLength, InputTake, Parser};
use std::ops::{Deref, DerefMut, Range};
use std::rc::Rc;
use std::string::ParseError;
use w_tokenize::{Kind, Span, Token};

pub type ParResult<'a, T = TokenSpan<'a>> = IResult<TokenSpan<'a>, T, ErrorChain<'a>>;

#[derive(Clone)]
pub struct TokenSpan<'a> {
    pub(crate) file: Span<'a>,
    pub(crate) local: Range<usize>,
    pub(crate) tokens: Rc<[Token<'a>]>,
}

impl<'a> TokenSpan<'a> {
    pub fn new(file: Span<'a>, tokens: Rc<[Token<'a>]>) -> TokenSpan<'a> {
        TokenSpan {
            file,
            local: 0..tokens.len(),
            tokens,
        }
    }
}

impl<'a> InputTake for TokenSpan<'a> {
    fn take(&self, count: usize) -> Self {
        if self.local.start + count > self.tokens.len() {
            panic!("TokenSpan::take: out of bounds");
        }
        Self {
            file: self.file.clone(),
            local: self.local.start..self.local.end + count,
            tokens: self.tokens.clone(),
        }
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        if self.local.start + count > self.tokens.len() {
            panic!("TokenSpan::take: out of bounds");
        }

        let left = self.local.start;
        let mid = self.local.start + count;
        let right = self.tokens.len();

        (
            Self {
                file: self.file,
                local: mid..right,
                tokens: self.tokens.clone(),
            },
            Self {
                file: self.file,
                local: left..mid,
                tokens: self.tokens.clone(),
            },
        )
    }
}

impl<'a> InputLength for TokenSpan<'a> {
    fn input_len(&self) -> usize {
        self.local.len()
    }
}

#[derive(Clone)]
pub struct Strong<'a>(pub Kind<'a>);
#[derive(Clone)]
pub struct Weak<'a>(pub Kind<'a>);

impl<'a> InputLength for Strong<'a> {
    fn input_len(&self) -> usize {
        1
    }
}
impl<'a> InputLength for Weak<'a> {
    fn input_len(&self) -> usize {
        1
    }
}

impl<'a, 'b> Compare<Strong<'a>> for Token<'b> {
    fn compare(&self, t: Strong<'a>) -> CompareResult {
        if self.kind == t.0 {
            CompareResult::Ok
        } else {
            CompareResult::Error
        }
    }

    fn compare_no_case(&self, t: Strong<'a>) -> CompareResult {
        self.compare(t)
    }
}
impl<'a, 'b> Compare<Weak<'a>> for Token<'b> {
    fn compare(&self, t: Weak<'a>) -> CompareResult {
        if self.kind.cmp_id() == t.0.cmp_id() {
            CompareResult::Ok
        } else {
            CompareResult::Error
        }
    }

    fn compare_no_case(&self, t: Weak<'a>) -> CompareResult {
        self.compare(t)
    }
}

impl<'a, 'b> Compare<Strong<'b>> for TokenSpan<'a> {
    fn compare(&self, t: Strong<'b>) -> CompareResult {
        if self.len() >= 1 {
            self[0].compare(t)
        } else {
            CompareResult::Error
        }
    }

    fn compare_no_case(&self, t: Strong<'b>) -> CompareResult {
        if self.len() >= 1 {
            self[0].compare_no_case(t)
        } else {
            CompareResult::Error
        }
    }
}
impl<'a, 'c> Compare<Weak<'c>> for TokenSpan<'a> {
    fn compare(&self, t: Weak<'c>) -> CompareResult {
        if self.len() >= 1 {
            self[0].compare(t)
        } else {
            CompareResult::Error
        }
    }

    fn compare_no_case(&self, t: Weak<'c>) -> CompareResult {
        if self.len() >= 1 {
            self[0].compare_no_case(t)
        } else {
            CompareResult::Error
        }
    }
}

impl<'a, 'b> Parser<TokenSpan<'a>, Token<'a>, ErrorChain<'a>> for Strong<'b> {
    fn parse(&mut self, input: TokenSpan<'a>) -> IResult<TokenSpan<'a>, Token<'a>, ErrorChain<'a>> {
        if input.is_empty() {
            Err(Err::Error(ErrorChain::from(Error::new(
                input,
                "expected token",
            ))))
        } else {
            let (ni, token) = input.take_split(1);
            if Compare::compare(&token[0], self.clone()) == CompareResult::Ok {
                Ok((ni, token[0].clone()))
            } else {
                Err(Err::Error(ErrorChain::from(Error::new(
                    input,
                    "expected token",
                ))))
            }
        }
    }
}
impl<'a, 'b> Parser<TokenSpan<'a>, Token<'a>, ErrorChain<'a>> for Weak<'b> {
    fn parse(&mut self, input: TokenSpan<'a>) -> IResult<TokenSpan<'a>, Token<'a>, ErrorChain<'a>> {
        if input.is_empty() {
            Err(Err::Error(ErrorChain::from(Error::new(
                input,
                "expected token",
            ))))
        } else {
            let (ni, token) = input.take_split(1);
            if Compare::compare(&token[0], self.clone()) == CompareResult::Ok {
                Ok((ni, token[0].clone()))
            } else {
                Err(Err::Error(ErrorChain::from(Error::new(
                    input,
                    "expected token",
                ))))
            }
        }
    }
}

impl<'a> Deref for TokenSpan<'a> {
    type Target = [Token<'a>];
    fn deref(&self) -> &Self::Target {
        &self.tokens[self.local.clone()]
    }
}
