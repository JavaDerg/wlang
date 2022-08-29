use crate::data::err::fmt::ErrorFormatter;
use crate::data::err::{AnalyzerError, ErrKind};
use w_parse::Ident;

pub struct MultipleDefinitionsError<'a> {
    pub loc: Ident<'a>,
    pub first: Ident<'a>,
    pub kind: DefinitionKind,
}

#[derive(Copy, Clone)]
pub enum DefinitionKind {
    Type,
    Func,
    Import,
}

impl<'a> AnalyzerError<'a> for MultipleDefinitionsError<'a> {
    fn kind(&self) -> ErrKind {
        ErrKind::Error
    }

    fn fmt(&self, f: &mut ErrorFormatter) {
        let (msg1, msg2) = match self.kind {
            DefinitionKind::Type => (
                "The type name is conflicting with another previous definition",
                "Conflicting type defined here",
            ),
            DefinitionKind::Func => (
                "The func name is conflicting with another previous definition",
                "Conflicting func defined here",
            ),
            DefinitionKind::Import => (
                "A import under that name is already defined at a previous location",
                "Other import location",
            ),
        };

        f.err()
            .description(msg1)
            .location(self.loc.0)
            .add_elaboration()
            .description(msg2)
            .location(self.first.0)
            .build_help()
            .submit();
    }
}
