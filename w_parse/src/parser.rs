use nom::bytes::complete::tag;
use nom::{Compare, CompareResult, InputTake, IResult};
use w_tokenize::{Kind, Token};
use crate::error::ErrorChain;

pub type ParResult<'a, T> = IResult<&'a [Token<'a>], T, ErrorChain<'a>>;
pub struct TokenSpan<'a>(&'a [Token<'a>]);

impl<'a> InputTake for TokenSpan<'a> {
    fn take(&self, count: usize) -> Self {
        Self(&self.0[..count])
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        let (prefix, suffix) = self.0.split_at(count);
        (Self(suffix), Self(prefix))
    }
}

pub struct Strong<'a>(pub Kind<'a>);
pub struct Weak<'a>(pub Kind<'a>);

impl<'a> Compare<Strong<'a>> for Token<'a> {
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

impl<'a> Compare<Weak<'a>> for Token<'a> {
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
