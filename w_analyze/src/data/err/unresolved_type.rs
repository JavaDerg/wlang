use crate::data::err::fmt::ErrorFormatter;
use crate::data::err::{AnalyzerError, ErrKind};
use crate::data::Location;
use w_parse::Ident;

pub struct UnresolvedTypeError<'a>(pub Ident<'a>);

impl<'a> AnalyzerError<'a> for UnresolvedTypeError<'a> {
    fn kind(&self) -> ErrKind {
        ErrKind::Error
    }

    fn fmt(&self, f: &mut ErrorFormatter) {
        f.err()
            .description("Unable to resolve type")
            .location(self.0 .0)
            .add_note("Try defining the type")
            .add_note("Try importing the type")
            .submit();
    }
}
