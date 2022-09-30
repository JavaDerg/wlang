use crate::data::err::fmt::ErrorFormatter;
use crate::data::err::{AnalyzerError, ErrKind};
use w_parse::Ident;

pub struct RecursiveTypeError {
    pub og: Ident,
    pub usage: Ident,
}

impl AnalyzerError for RecursiveTypeError {
    fn kind(&self) -> ErrKind {
        ErrKind::Error
    }

    fn fmt(&self, f: &mut ErrorFormatter) {
        f.err()
            .description("This type may not contain it self directly")
            .location(self.usage.0.clone())
            .add_elaboration()
            .description("The type is defined here")
            .location(self.og.0.clone())
            .build_help()
            .add_note("Types may contain them self but only indirectly through pointers")
            .add_note("References to other types may never point to them self")
            .build();
    }
}
