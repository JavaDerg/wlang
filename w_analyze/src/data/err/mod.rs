mod array_fix;
mod duplicate_import;
mod fmt;
mod multiple_definitions;
mod recursive_type;
mod unresolved_type;

use crate::data::err::fmt::ErrorFormatter;
use std::cell::RefCell;

pub use array_fix::*;
pub use duplicate_import::*;
pub use multiple_definitions::*;
pub use recursive_type::*;
pub use unresolved_type::*;

#[derive(Default)]
pub struct ErrorCollector {
    errors: RefCell<Vec<Box<dyn AnalyzerError>>>,
    has_errors: RefCell<bool>,
}

#[derive(Debug, Copy, Clone)]
pub enum ErrKind {
    Error,
    Warning,
}

pub trait AnalyzerError {
    fn kind(&self) -> ErrKind;
    fn fmt(&self, f: &mut ErrorFormatter);
}

impl ErrorCollector {
    pub fn add_error(&self, error: impl AnalyzerError + 'static) {
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
