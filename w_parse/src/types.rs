use std::collections::HashMap;
use crate::Identifier;

pub struct TypeDescriptor<'a> {
    pub generics: Vec<Identifier<'a>>,
}