use std::borrow::Cow;
use w_tokenize::Span;

pub struct ErrorChain<'a> {
    err_acc: Vec<Error<'a>>,
}

pub struct Error<'a> {
    pub location: Span<'a>,
    pub reason: Cow<'static, str>,
}

impl<'a> ErrorChain<'a> {
    pub fn has_errs(&self) -> bool {
        !self.err_acc.is_empty()
    }

    pub fn put_errs(&mut self, mut other: ErrorChain<'a>) {
        self.err_acc.append(&mut other.err_acc);
    }
}
