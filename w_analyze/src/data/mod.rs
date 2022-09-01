// This pass builds the basic type and function system

pub mod err;
pub mod file;
pub mod path;
mod typemap;
pub mod types;

use crate::data::file::FileRef;
use crate::data::path::{Path, PathBuf};
use crate::data::types::TypeRef;
use either::Either;
use std::cell::RefCell;
use std::collections::HashMap;
use typed_arena::Arena;

use w_parse::Ident;

pub struct Module<'a, 'gc> {
    pub types_arena: &'gc Arena<TypeRef<'a, 'gc>>,
    pub modules_arena: &'gc Arena<Self>,

    pub types: RefCell<HashMap<Ident<'a>, &'gc TypeRef<'a, 'gc>>>,
    pub modules: RefCell<HashMap<Ident<'a>, &'gc Self>>,

    // the module is where the item is from and not the item it self
    pub imports: RefCell<HashMap<Ident<'a>, &'gc Self>>,

    pub previous: Option<&'gc Self>,

    pub path: PathBuf<'a>,

    pub owner: ModuleOwner<'a, 'gc>,
}

pub type ModuleOwner<'a, 'gc> = Option<Either<FileRef, Location<'a, 'gc>>>;

#[derive(Clone)]
pub struct Location<'a, 'gc> {
    pub name: Ident<'a>,
    pub home: &'gc Module<'a, 'gc>,
}

pub enum Origin<'a, 'gc> {
    Local(&'gc TypeRef<'a, 'gc>),
    Import(&'gc TypeRef<'a, 'gc>),
}

impl<'a, 'gc> Module<'a, 'gc> {
    pub fn new(
        path: PathBuf<'a>,
        _owner: ModuleOwner,
        modules: &'gc Arena<Self>,
        types: &'gc Arena<TypeRef<'a, 'gc>>,
    ) -> &'gc Self {
        modules.alloc(Module {
            types_arena: types,
            modules_arena: modules,
            types: RefCell::new(HashMap::new()),
            modules: RefCell::new(HashMap::new()),
            imports: RefCell::new(HashMap::new()),
            previous: None,
            path,
            owner: None,
        })
    }

    pub fn previous(&self) -> Option<&'gc Module<'a, 'gc>> {
        self.previous
    }

    pub fn root(&'gc self) -> &'gc Module<'a, 'gc> {
        self.previous.map_or(self, |p| p.root())
    }

    pub fn create_anonymous_type(&self) -> &'gc TypeRef<'a, 'gc> {
        &*self.types_arena.alloc(TypeRef {
            loc: None,
            definition: RefCell::new(None),
        })
    }

    pub fn access_or_create_type(&'gc self, path: &Path<'a>) -> Origin<'a, 'gc> {
        if path.is_empty() {
            panic!("empty path provided");
        }
        let name = path.last().unwrap().clone();

        if path.len() == 1 {
            if let Some(imp_md) = self.imports.borrow().get(&name) {
                return Origin::Import(imp_md.access_or_create_type(path).unwrap());
            }
        }

        if let Some(imp_md) = self.imports.borrow().get(path.first().unwrap()) {
            return Origin::Import(imp_md.access_or_create_type(path).unwrap());
        }

        let md_path = path.slice(..path.len() - 1);
        let md = self.access_or_create_module(md_path);

        Origin::Local(
            *md.types
                .borrow_mut()
                .entry(name.clone())
                .or_insert_with(|| {
                    &*self.types_arena.alloc(TypeRef {
                        loc: Some(Location { name, home: md }),
                        definition: RefCell::new(None),
                    })
                }),
        )
    }

    pub fn access_or_create_module(&'gc self, path: &Path<'a>) -> &'gc Module<'a, 'gc> {
        if path.is_empty() {
            return self;
        }

        if let Some(imp_md) = self.imports.borrow().get(path.first().unwrap()) {
            return imp_md.access_or_create_module(path);
        }

        let next = path.first().unwrap();
        let next = *self
            .modules
            .borrow_mut()
            .entry(next.clone())
            .or_insert_with(|| {
                Module::new(
                    self.path.join(next.clone()),
                    None,
                    self.modules_arena,
                    self.types_arena,
                )
            });

        next.access_or_create_module(path.slice(1..))
    }
}

impl<'a, 'gc> Origin<'a, 'gc> {
    pub fn unwrap(&self) -> &'gc TypeRef<'a, 'gc> {
        match self {
            Origin::Local(t) => t,
            Origin::Import(t) => t,
        }
    }
}
