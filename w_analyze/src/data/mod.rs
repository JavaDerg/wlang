// This pass builds the basic type and function system

pub mod err;
pub mod types;

use crate::data::types::TypeRef;
use std::cell::RefCell;
use std::collections::HashMap;
use typed_arena::Arena;
use w_parse::expr::path::Path;
use w_parse::Ident;

pub struct SimpleTypeSystem<'a, 'gc> {
    pub types_arena: &'gc Arena<TypeRef<'a, 'gc>>,
    pub types: RefCell<HashMap<Path<'a>, &'gc TypeRef<'a, 'gc>>>,
}

#[derive(Clone)]
pub struct Location<'a> {
    pub name: Ident<'a>,
    pub path: Path<'a>,
}

impl<'a, 'gc> SimpleTypeSystem<'a, 'gc> {
    pub fn new(types_arena: &'gc Arena<TypeRef<'a, 'gc>>) -> Self {
        Self {
            types_arena,
            types: Default::default(),
        }
    }
}
