use crate::data::err::fmt::builder::StepDesc;
use crate::data::err::ErrKind;
use w_tokenize::Span;

pub struct ErrorFormatter {}

pub struct Error<'a> {
    kind: ErrKind,

    description: String,
    location: Span<'a>,

    help: Vec<Error<'a>>,
    notes: Vec<String>,
}

pub struct ErrorBuilder<'a, 'b> {
    fmt: Option<&'b mut ErrorFormatter>,
    kind: ErrKind,

    description: Option<String>,
    location: Option<Span<'a>>,

    help: Vec<Error<'a>>,
    notes: Vec<String>,
}

impl<'a, 'b> ErrorBuilder<'a, 'b> {
    fn derive_new(&self) -> Self {
        Self {
            fmt: None,
            kind: self.kind,
            description: None,
            location: None,
            help: vec![],
            notes: vec![],
        }
    }
}

macro_rules! define_builder_steps {
    {
        $name:ident $((0, $($dty:ty),*))? => { $(fn $fn_name:ident ($($args:tt)*) -> $next:ty $fn_body:block)* } $(,)?
    } => {
        pub struct $name<'a, 'b>(pub(super) ErrorBuilder<'a, 'b>, $($($dty),*)?);
        impl<'a, 'b> $name<'a, 'b> {
            $(
                pub fn $fn_name ($($args)*) -> $next $fn_body
            )*
        }
    };

    {
        $(
            $name:ident $((0, $($dty:ty),*))? => { $(fn $fn_name:ident ($($args:tt)*) -> $next:ty $fn_body:block)* }
        ),* $(,)?
    } => {
        $(
            define_builder_steps! {
                $name $((0, $($dty),*))? => { $(fn $fn_name ($($args)*) -> $next $fn_body)* }
            }
        )*
    };
}

impl ErrorFormatter {
    pub fn err(&mut self) -> StepDesc {
        StepDesc(ErrorBuilder {
            fmt: Some(self),
            kind: ErrKind::Error,
            description: None,
            location: None,
            help: vec![],
            notes: vec![],
        })
    }

    pub fn warn(&mut self) -> StepDesc {
        StepDesc(ErrorBuilder {
            fmt: Some(self),
            kind: ErrKind::Warning,
            description: None,
            location: None,
            help: vec![],
            notes: vec![],
        })
    }

    pub fn submit(&mut self, _error: Error) {
        todo!()
    }
}

mod builder {
    use super::{Error, ErrorBuilder, ErrorFormatter};
    use w_tokenize::Span;

    define_builder_steps! {
        StepDesc => { fn description(mut self, text: impl Into<String>) -> StepLoc<'a, 'b> {
            self.0.description = Some(text.into());
            StepLoc(self.0)
        } },
        StepLoc => { fn location(mut self, loc: impl Into<Span<'a>>) -> StepDone<'a, 'b> {
            self.0.location = Some(loc.into());
            StepDone(self.0)
        } },
        StepDone => {
            fn build(self) -> Error<'a> {
                Error {
                    kind: self.0.kind,

                    description: self.0.description.unwrap(),
                    location: self.0.location.unwrap(),

                    help: self.0.help,
                    notes: self.0.notes,
                }
            }
            fn submit(mut self) -> &'b mut ErrorFormatter {
                let fmt = self.0.fmt.take().unwrap();
                let err = self.build();
                fmt.submit(err);
                fmt
            }
            fn add_help(self) -> StepHelpDesc<'a, 'b> {
                let help = self.0.derive_new();
                StepHelpDesc(self.0, help)
            }
            fn add_note(mut self, note: impl Into<String>) -> Self {
                self.0.notes.push(note.into());
                self
            }
        },
        StepHelpDesc (0, ErrorBuilder<'a, 'b>) => { fn description(mut self, text: impl Into<String>) -> StepHelpLoc<'a, 'b> {
            self.1.description = Some(text.into());
            StepHelpLoc(self.0, self.1)
        } },
        StepHelpLoc (0, ErrorBuilder<'a, 'b>) => { fn location(mut self, loc: impl Into<Span<'a>>) -> StepHelpDone<'a, 'b> {
            self.1.location = Some(loc.into());
            StepHelpDone(self.0, self.1)
        } },
        StepHelpDone (0, ErrorBuilder<'a, 'b>) => {
            fn build_help(mut self) -> StepDone<'a, 'b> {
                self.0.help.push(StepDone(self.1).build());
                StepDone(self.0)
            }
            fn add_note(mut self, note: impl Into<String>) -> Self {
                self.1.notes.push(note.into());
                self
            }
        }
    }
}
