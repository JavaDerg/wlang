mod types;
mod error;
mod parser;

use std::collections::HashMap;
use std::rc::Rc;
use w_tokenize::Span;

pub type SVec<T> = Rc<[T]>;

pub struct Identifier<'a>(pub Span<'a>);

pub struct Name<'a> {
    pub main: Identifier<'a>,
    pub generic_params: SVec<Identifier<'a>>,
}

fn svconv<T>(v: Vec<T>) -> Rc<[T]> {
    Rc::from(v.into_boxed_slice())
}
