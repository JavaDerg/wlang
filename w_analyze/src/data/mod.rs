// This pass builds the basic type and function system

pub mod err;
pub mod types;

use crate::data::types::TypeRef;
use std::cell::RefCell;
use std::collections::HashMap;
use either::Either;
use typed_arena::Arena;
use w_parse::expr::path::Path;
use w_parse::Ident;

pub struct Module<'a, 'gc> {
    pub types_arena: &'gc Arena<TypeRef<'a, 'gc>>,
    pub modules_arena: &'gc Arena<Self>,

    pub types: RefCell<HashMap<Ident<'a>, TypeRef<'a, 'gc>>>,
    pub modules: RefCell<HashMap<Ident<'a>, Self>>,

    pub previous: Option<&'gc Self>,
}

#[derive(Clone)]
pub struct Location<'a> {
    pub name: Ident<'a>,
    pub path: Path<'a>,
}

impl<'a, 'gc> Module<'a, 'gc> {
    pub fn new(modules: &'gc Arena<Self>, types: &'gc Arena<TypeRef<'a, 'gc>>) -> &'gc Self {
        modules.alloc(Module {
            types_arena: types,
            modules_arena: modules,
            types: RefCell::new(HashMap::new()),
            modules: RefCell::new(HashMap::new()),
            previous: None,
        })
    }

    pub fn previous(&self) -> Option<&'gc Module<'a, 'gc>> {
        self.previous
    }

    pub fn root(&self) -> &'gc Module<'a, 'gc> {
        self.previous.map_or(self, |p| p.root())
    }

    pub fn access_or_create_type(&self, path: &[&Path<'a>]) -> &'gc TypeRef<'a, 'gc> {

        todo!()
    }

    pub fn access_or_create_module(&self, path: &[&Path<'a>]) -> &'gc Module<'a, 'gc> {
        todo!()
    }
}
