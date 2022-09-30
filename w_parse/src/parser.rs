use crate::error::{Error, ErrorChain};
use nom::{Compare, CompareResult, Err, IResult, InputLength, InputTake, Offset, Parser, Slice};

use std::ops::{Deref, Range, RangeTo};
use std::rc::Rc;

use w_tokenize::{Kind, Span, Token};

pub type ParResult<T = TokenSpan> = IResult<TokenSpan, T, ErrorChain>;

#[derive(Debug, Clone)]
pub struct TokenSpan {
    pub(crate) file: Span,
    pub(crate) local: Range<usize>,
    pub(crate) tokens: Rc<[Token]>,
}

impl TokenSpan {
    pub fn new(file: Span, tokens: Rc<[Token]>) -> TokenSpan {
        TokenSpan {
            file,
            local: 0..tokens.len(),
            tokens,
        }
    }

    pub fn as_span(&self) -> Span {
        let diff = self.local.end - self.local.start;
        match diff {
            0 => {
                let last = &self.tokens[self.local.end];
                let offset = last.span.location_offset() + last.span.len();
                self.file.slice(offset..)
            }
            1 => self.tokens[self.local.start].span.clone(),
            _ => {
                let start = self.tokens[self.local.start].span.clone();
                let end = self.tokens[self.local.end].span.clone();

                let so = start.location_offset();
                let eo = end.location_offset() + end.len();

                self.file.slice(so..eo)
            }
        }
    }
}

impl From<&TokenSpan> for Span {
    fn from(tks: &TokenSpan) -> Self {
        tks.as_span()
    }
}

impl Offset for TokenSpan {
    fn offset(&self, second: &Self) -> usize {
        second.local.start - self.local.start
    }
}

impl Slice<Range<usize>> for TokenSpan {
    fn slice(&self, range: Range<usize>) -> Self {
        let offset_start = self.local.start + range.start;
        let offset_end = self.local.start + range.end;

        if offset_start > self.local.end || offset_end > self.local.end {
            panic!("out of range");
        }

        TokenSpan {
            file: self.file.clone(),
            local: offset_start..offset_end,
            tokens: self.tokens.clone(),
        }
    }
}

impl Slice<RangeTo<usize>> for TokenSpan {
    fn slice(&self, range: RangeTo<usize>) -> Self {
        let offset_end = self.local.start + range.end;

        if offset_end > self.local.end {
            panic!("out of range");
        }

        TokenSpan {
            file: self.file.clone(),
            local: self.local.start..offset_end,
            tokens: self.tokens.clone(),
        }
    }
}

impl InputTake for TokenSpan {
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
                file: self.file.clone(),
                local: mid..right,
                tokens: self.tokens.clone(),
            },
            Self {
                file: self.file.clone(),
                local: left..mid,
                tokens: self.tokens.clone(),
            },
        )
    }
}

impl InputLength for TokenSpan {
    fn input_len(&self) -> usize {
        self.local.len()
    }
}

#[derive(Clone)]
pub struct Strong(pub Kind);
#[derive(Clone)]
pub struct Weak(pub Kind);

impl InputLength for Strong {
    fn input_len(&self) -> usize {
        1
    }
}
impl InputLength for Weak {
    fn input_len(&self) -> usize {
        1
    }
}

impl Compare<Strong> for Token {
    fn compare(&self, t: Strong) -> CompareResult {
        if self.kind == t.0 {
            CompareResult::Ok
        } else {
            CompareResult::Error
        }
    }

    fn compare_no_case(&self, t: Strong) -> CompareResult {
        self.compare(t)
    }
}
impl Compare<Weak> for Token {
    fn compare(&self, t: Weak) -> CompareResult {
        if self.kind.cmp_id() == t.0.cmp_id() {
            CompareResult::Ok
        } else {
            CompareResult::Error
        }
    }

    fn compare_no_case(&self, t: Weak) -> CompareResult {
        self.compare(t)
    }
}

impl Compare<Strong> for TokenSpan {
    fn compare(&self, t: Strong) -> CompareResult {
        if self.len() >= 1 {
            self[0].compare(t)
        } else {
            CompareResult::Error
        }
    }

    fn compare_no_case(&self, t: Strong) -> CompareResult {
        if self.len() >= 1 {
            self[0].compare_no_case(t)
        } else {
            CompareResult::Error
        }
    }
}
impl Compare<Weak> for TokenSpan {
    fn compare(&self, t: Weak) -> CompareResult {
        if self.len() >= 1 {
            self[0].compare(t)
        } else {
            CompareResult::Error
        }
    }

    fn compare_no_case(&self, t: Weak) -> CompareResult {
        if self.len() >= 1 {
            self[0].compare_no_case(t)
        } else {
            CompareResult::Error
        }
    }
}

impl Parser<TokenSpan, Token, ErrorChain> for Strong {
    fn parse(&mut self, input: TokenSpan) -> IResult<TokenSpan, Token, ErrorChain> {
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
impl Parser<TokenSpan, Token, ErrorChain> for Weak {
    fn parse(&mut self, input: TokenSpan) -> IResult<TokenSpan, Token, ErrorChain> {
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

impl Deref for TokenSpan {
    type Target = [Token];
    fn deref(&self) -> &Self::Target {
        &self.tokens[self.local.clone()]
    }
}
