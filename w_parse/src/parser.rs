use std::ops::{Deref, DerefMut};
use std::string::ParseError;
use nom::bytes::complete::tag;
use nom::{Compare, CompareResult, Err, InputLength, InputTake, IResult, Parser};
use w_tokenize::{Kind, Token};
use crate::error::{Error, ErrorChain};

pub type ParResult<'a, 'b, T = TokenSpan<'a, 'b>> = IResult<TokenSpan<'a, 'b>, T, ErrorChain<'a, 'b>>;
pub struct TokenSpan<'a, 'b>(pub &'b [Token<'a>]);

impl<'a, 'b> InputTake for TokenSpan<'a, 'b> {
    fn take(&self, count: usize) -> Self {
        Self(&self.0[..count])
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        let (prefix, suffix) = self.0.split_at(count);
        (Self(suffix), Self(prefix))
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

impl<'a, 'b, 'c> Compare<Strong<'c>> for TokenSpan<'a, 'b> {
    fn compare(&self, t: Strong<'c>) -> CompareResult {
        if self.len() == 1 {
            self[0].compare(t)
        } else {
            CompareResult::Error
        }
    }

    fn compare_no_case(&self, t: Strong<'c>) -> CompareResult {
        if self.len() == 1 {
            self[0].compare_no_case(t)
        } else {
            CompareResult::Error
        }
    }
}
impl<'a, 'b, 'c> Compare<Weak<'c>> for TokenSpan<'a, 'b> {
    fn compare(&self, t: Weak<'c>) -> CompareResult {
        if self.len() == 1 {
            self[0].compare(t)
        } else {
            CompareResult::Error
        }
    }

    fn compare_no_case(&self, t: Weak<'c>) -> CompareResult {
        if self.len() == 1 {
            self[0].compare_no_case(t)
        } else {
            CompareResult::Error
        }
    }
}


impl<'a, 'b, 'c> Parser<TokenSpan<'a, 'b>, Token<'a>, ErrorChain<'a, 'b>> for Strong<'c> {
    fn parse(&mut self, input: TokenSpan<'a, 'b>) -> IResult<TokenSpan<'a, 'b>, Token<'a>, ErrorChain<'a, 'b>> {
        if input.is_empty() {
            Err(Err::Error(ErrorChain::from(Error::new(input, "expected token"))))
        } else {
            let (ni, token) = input.take_split(1);
            if Compare::compare(&token[0], self.clone()) == CompareResult::Ok {
                Ok((ni, token[0].clone()))
            } else {
                Err(Err::Error(ErrorChain::from(Error::new(input, "expected token"))))
            }
        }
    }
}
impl<'a, 'b, 'c> Parser<TokenSpan<'a, 'b>, Token<'a>, ErrorChain<'a, 'b>> for Weak<'c> {
    fn parse(&mut self, input: TokenSpan<'a, 'b>) -> IResult<TokenSpan<'a, 'b>, Token<'a>, ErrorChain<'a, 'b>> {
        if input.is_empty() {
            Err(Err::Error(ErrorChain::from(Error::new(input, "expected token"))))
        } else {
            let (ni, token) = input.take_split(1);
            if Compare::compare(&token[0], self.clone()) == CompareResult::Ok {
                Ok((ni, token[0].clone()))
            } else {
                Err(Err::Error(ErrorChain::from(Error::new(input, "expected token"))))
            }
        }
    }
}

impl<'a, 'b> Deref for TokenSpan<'a, 'b> {
    type Target = &'b [Token<'a>];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'b> DerefMut for TokenSpan<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0

    }
}
