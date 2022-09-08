use crate::data::err::fmt::ErrorFormatter;
use crate::data::err::{AnalyzerError, ErrKind};
use std::borrow::Cow;

use w_parse::Ident;
use w_tokenize::Span;

pub struct ArrayNumberFix<'a> {
    pub loc: Span<'a>,
    pub msg: Cow<'static, str>,
}

impl<'a> AnalyzerError<'a> for ArrayNumberFix<'a> {
    fn kind(&self) -> ErrKind {
        ErrKind::Error
    }

    fn fmt(&self, f: &mut ErrorFormatter) {
        f.err()
            .description(&*self.msg)
            .location(self.loc)
            .add_note(
                "Due to compiler limitations, array sizes must be a number literal of usize kind",
            )
            .add_note("Try contributing to the compiler")
            .submit();
    }
}
