use crate::data::err::fmt::ErrorFormatter;
use crate::data::err::{AnalyzerError, ErrKind};
use w_parse::Ident;

pub struct RecursiveTypeError<'a> {
    pub og: Ident<'a>,
    pub usage: Ident<'a>,
}

impl<'a> AnalyzerError<'a> for RecursiveTypeError<'a> {
    fn kind(&self) -> ErrKind {
        ErrKind::Error
    }

    fn fmt(&self, f: &mut ErrorFormatter) {
        f.err()
            .description("This type may not contain it self directly")
            .location(self.usage.0)
            .add_elaboration()
            .description("The type is defined here")
            .location(self.og.0)
            .build_help()
            .add_note("Types may contain them self but only indirectly through pointers")
            .add_note("References to other types may never point to them self")
            .build();
    }
}
