use crate::data::err::UnresolvedTypeError;
use crate::data::types::{TypeInfo, TypeKind, TypeRef};
use crate::data::Location;
use crate::{ErrorCollector, SimpleTypeSystem};
use std::cell::RefCell;
use w_parse::item::import::{Imports, ItemImports};
use w_parse::item::named::NamedKind;
use w_parse::item::Item;
use w_parse::types::ItemTy;
use w_parse::Module;

pub fn run_pass1<'a>(
    module: &Module<'a>,
    tsys: &SimpleTypeSystem<'a, '_>,
    errs: &ErrorCollector<'a>,
) {
    let mut progress = true;
    while progress {
        progress = false;

        for item in module
            .items
            .iter()
        {
            let def = match item {
                Item::Definer(def) => def,
                Item::Import(imports) => {
                    continue;
                }
            };
            let ty = match &def.kind {
                NamedKind::Type(ty) => ty,
                NamedKind::Func(_) => continue,
            };

            let lpath = module.path.join_s(&def.name);

            if let Some(&tref) = tsys.types.borrow().get(&lpath) {
                if tref.definition.borrow().is_some() {
                    continue;
                }
            }

            let kind = match ty.ty.clone() {
                ItemTy::Named(path) => {
                    if let Some(&tref) = tsys.types.borrow().get(&path) {
                        TypeKind::Nested(tref)
                    } else {
                        let ident = path.path[path.path.len() - 1].clone();
                        let t = TypeRef {
                            loc: Location { name: ident, path },
                            definition: RefCell::new(None),
                        };
                        let tref = &*tsys.types_arena.alloc(t);
                        TypeKind::Nested(tref)
                    }
                }
                ItemTy::Struct(st) => TypeKind::Struct(st),
                ItemTy::Enum(en) => TypeKind::Enum(en),
                ItemTy::Tuple(tp) => TypeKind::Tuple(tp),
                ItemTy::Func(func) => TypeKind::Func(func),
                ItemTy::Array(ar) => TypeKind::Array(ar),
                ItemTy::Pointer(ptr) => TypeKind::Ptr(ptr),
                ItemTy::Never(nv) => TypeKind::Never(nv),
            };

            tsys.types
                .borrow_mut()
                .entry(lpath.clone())
                .or_insert_with(move || {
                    &*tsys.types_arena.alloc(TypeRef {
                        loc: Location {
                            name: def.name.clone(),
                            path: lpath,
                        },
                        definition: RefCell::new(None),
                    })
                })
                .definition
                .borrow_mut()
                .replace(TypeInfo { kind });

            progress = true;
        }
    }

    tsys.types
        .borrow()
        .iter()
        .filter(|(_, v)| v.definition.borrow().is_none())
        .for_each(|(_, v)| errs.add_error(UnresolvedTypeError(v.loc.clone())))
}

fn import_imports<'a>(module: &Module<'a>, tsys: &SimpleTypeSystem<'a, '_>, errs: &ErrorCollector<'a>, imports: &ItemImports<'a>) -> bool {
    for import in &imports.imports {
        match import {
            Imports::Single(direct) => {

            }
            Imports::Multiple(_, _) => {}
        }
    }
}
