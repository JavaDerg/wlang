use crate::data::err::fmt::ErrorFormatter;
use crate::data::err::{AnalyzerError, ErrKind};
use w_parse::Ident;
use w_tokenize::Span;

pub struct DuplicateImport<'a> {
    pub original: Span<'a>,
    pub new: Span<'a>,
}

impl<'a> AnalyzerError<'a> for DuplicateImport<'a> {
    fn kind(&self) -> ErrKind {
        ErrKind::Error
    }

    fn fmt(&self, f: &mut ErrorFormatter) {
        f.err()
            .description("Item imported more than once")
            .location(self.new)
            .add_elaboration()
            .description("It was originally imported here")
            .location(self.original)
            .build_help()
            .submit();
    }
}
