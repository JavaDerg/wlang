use crate::data::err::{DefinitionKind, MultipleDefinitionsError, UnresolvedTypeError};
use crate::data::types::{
    TypeArray, TypeEnum, TypeFunc, TypeInfo, TypeKind, TypeNever, TypePtr, TypeRef, TypeStruct,
    TypeTuple,
};
use crate::{ErrorCollector, Module, PathBuf};
use std::cell::RefCell;
use w_parse::expr::path::ExprPath;
use w_parse::item::import::{Imports, ItemImports};
use w_parse::item::named::NamedKind;
use w_parse::item::Item;
use w_parse::types::array::TyArray;
use w_parse::types::func::TyFunc;
use w_parse::types::never::TyNever;
use w_parse::types::ptr::TyPtr;
use w_parse::types::r#enum::TyEnum;
use w_parse::types::r#struct::TyStruct;
use w_parse::types::tuple::TyTuple;
use w_parse::types::ItemTy;
use w_parse::util::NameTyPair;
use w_parse::ParsedModule;

pub fn run_pass1<'a, 'gc>(
    module: &ParsedModule<'a>,
    tsys: &'gc Module<'a, 'gc>,
    errs: &ErrorCollector<'a>,
) {
    for item in module.items.iter() {
        let def = match item {
            Item::Definer(def) => def,
            Item::Import(_imports) => {
                continue;
            }
        };

        let ty = match &def.kind {
            NamedKind::Type(ty) => ty,
            NamedKind::Func(_) => continue,
        };

        let tref = tsys.access_or_create_type(&PathBuf::from([def.name.clone()]));

        if tref.definition.borrow().is_some() {
            errs.add_error(MultipleDefinitionsError {
                loc: def.name.clone(),
                first: tref
                    .loc
                    .clone()
                    .expect("Type ref should have location if present in type system")
                    .name
                    .clone(),
                kind: DefinitionKind::Type,
            });
            continue;
        }

        let kind = build_type(&ty.ty, tsys, errs);
    }

    tsys.types
        .borrow()
        .iter()
        .filter(|(_, v)| v.definition.borrow().is_none())
        .for_each(|(_, v)| {
            errs.add_error(UnresolvedTypeError(v.loc.as_ref().unwrap().name.clone()))
        })
}

fn resolve_type<'a, 'gc>(
    ty: &ItemTy<'a>,
    tsys: &'gc Module<'a, 'gc>,
    errs: &ErrorCollector<'a>,
) -> &'gc TypeRef<'a, 'gc> {
    match ty {
        ItemTy::Named(name) => {
            let (md, path) = conv_path(tsys, name);
            md.access_or_create_type(&path)
        }
        _ => {
            let ty = build_type(ty, tsys, errs);
            let tref = &*tsys.types_arena.alloc(TypeRef {
                loc: None,
                definition: RefCell::new(Some(TypeInfo { kind: ty })),
            });
            tref
        }
    }
}

fn build_type<'a, 'gc>(
    ty: &ItemTy<'a>,
    tsys: &'gc Module<'a, 'gc>,
    errs: &ErrorCollector<'a>,
) -> TypeKind<'a, 'gc> {
    match ty {
        ItemTy::Named(_) => panic!("Named type not expected"),
        ItemTy::Struct(TyStruct {
            span_struct,
            fields,
        }) => {
            TypeKind::Struct(TypeStruct {
                def: *span_struct,
                fields: fields
                    .iter()
                    .map(|NameTyPair { name, ty }| {
                        match ty {
                            ItemTy::Named(path) => {
                                let (md, path) = conv_path(tsys, path);
                                (name.clone(), md.access_or_create_type(&path))
                            }
                            // anonymous type
                            _ => {
                                let tref = tsys.create_anonymous_type();
                                *tref.definition.borrow_mut() = Some(TypeInfo {
                                    kind: build_type(ty, tsys, errs),
                                });
                                (name.clone(), tref)
                            }
                        }
                    })
                    .collect(),
            })
        }
        ItemTy::Enum(TyEnum {
            span_enum,
            variants,
        }) => TypeKind::Enum(TypeEnum {
            def: *span_enum,
            variants: variants
                .iter()
                .map(|(name, ty)| {
                    (
                        name.clone(),
                        ty.as_ref().map(|tp| conv_tuple(tp, tsys, errs)),
                    )
                })
                .collect(),
        }),
        ItemTy::Tuple(tp) => TypeKind::Tuple(conv_tuple(tp, tsys, errs)),
        ItemTy::Func(TyFunc {
            span_func,
            args,
            ret_ty,
        }) => TypeKind::Func(TypeFunc {
            def: *span_func,
            args: args.iter().map(|ty| resolve_type(ty, tsys, errs)).collect(),
            ret: resolve_type(ret_ty, tsys, errs),
        }),
        ItemTy::Array(TyArray { span, ty, size }) => TypeKind::Array(TypeArray {
            def: *span,
            ty: resolve_type(ty, tsys, errs),
            len: size.clone(),
        }),
        ItemTy::Pointer(TyPtr { span_ptr, ty }) => TypeKind::Ptr(TypePtr {
            def: *span_ptr,
            ty: resolve_type(ty, tsys, errs),
        }),
        ItemTy::Never(TyNever(span)) => TypeKind::Never(TypeNever(*span)),
    }
}

fn conv_tuple<'a, 'gc>(
    TyTuple { span, types }: &TyTuple<'a>,
    tsys: &'gc Module<'a, 'gc>,
    errs: &ErrorCollector<'a>,
) -> TypeTuple<'a, 'gc> {
    TypeTuple {
        def: *span,
        fields: types
            .iter()
            .map(|ty| resolve_type(ty, tsys, errs))
            .collect(),
    }
}

fn import_imports<'a>(
    module: &ParsedModule<'a>,
    tsys: &Module<'a, '_>,
    errs: &ErrorCollector<'a>,
    imports: &ItemImports<'a>,
) -> bool {
    for import in &imports.imports {
        match import {
            Imports::Single(direct) => {}
            Imports::Multiple(_, _) => {}
        }
    }

    todo!()
}

fn conv_path<'a, 'gc>(
    tsys: &'gc Module<'a, 'gc>,
    path: &ExprPath<'a>,
) -> (&'gc Module<'a, 'gc>, PathBuf<'a>) {
    let md = if path.root.is_some() {
        tsys.root()
    } else {
        tsys
    };
    (md, PathBuf::from(path.path.as_slice()))
}
