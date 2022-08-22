mod unresolved_type;
mod fmt;

use std::cell::RefCell;
use crate::data::err::fmt::ErrorFormatter;

pub use unresolved_type::UnresolvedTypeError;

#[derive(Default)]
pub struct ErrorCollector<'a> {
    errors: RefCell<Vec<Box<dyn AnalyzerError<'a>>>>,
    has_errors: RefCell<bool>,
}

#[derive(Debug, Copy, Clone)]
pub enum ErrKind {
    Error,
    Warning,
}

pub trait AnalyzerError<'a>: 'a {
    fn kind(&self) -> ErrKind;
    fn fmt(&self, f: &mut ErrorFormatter);
}

impl<'a> ErrorCollector<'a> {
    pub fn add_error(&self, error: impl AnalyzerError<'a> + 'a) {
        let err = matches!(error.kind(), ErrKind::Error);

        self.errors.borrow_mut().push(Box::new(error));

        if err {
            *self.has_errors.borrow_mut() = true;
        }
    }

    pub fn has_errors(&self) -> bool {
        *self.has_errors.borrow()
    }
}
