use crate::data::err::fmt::ErrorFormatter;
use crate::data::err::{AnalyzerError, ErrKind};

use w_tokenize::Span;

pub struct DuplicateImport {
    pub original: Span,
    pub new: Span,
}

impl AnalyzerError for DuplicateImport {
    fn kind(&self) -> ErrKind {
        ErrKind::Error
    }

    fn fmt(&self, f: &mut ErrorFormatter) {
        f.err()
            .description("Item imported more than once")
            .location(self.new.clone())
            .add_elaboration()
            .description("It was originally imported here")
            .location(self.original.clone())
            .build_help()
            .submit();
    }
}
