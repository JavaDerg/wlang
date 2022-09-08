use crate::data::err::{
    ArrayNumberFix, DefinitionKind, MultipleDefinitionsError, UnresolvedTypeError,
};
use crate::data::types::{
    TypeArray, TypeEnum, TypeFunc, TypeInfo, TypeKind, TypeNever, TypePtr, TypeStruct, TypeTuple,
};
use crate::{ErrorCollector, Module, PathBuf};
use std::borrow::Cow;
use w_parse::expr::path::ExprPath;
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
use w_tokenize::Number;

pub fn run_pass1<'a, 'gc>(
    module: &ParsedModule<'a>,
    tsys: &'gc Module<'a, 'gc>,
    errs: &ErrorCollector<'a>,
) {
    // Imports
    // for item in module.items.iter() {
    //     let def = match item {
    //         Item::Import(def) => def,
    //         Item::Definer(_) => continue,
    //     };
    //
    //     let (root, base) = conv_path(tsys, &def.from);
    //     let root = root.access_or_create_module(&base);
    //
    //     resolve_imports(&def.imports, root, tsys, errs);
    // }

    // Type definitions
    for item in module.items.iter() {
        let def = match item {
            Item::Definer(def) => def,
            Item::Import(_) => continue,
        };

        let ty = match &def.kind {
            NamedKind::Type(ty) => ty,
            NamedKind::Func(_) => continue,
        };

        let tref = tsys.access_or_create_type(&PathBuf::from([def.name.clone()]));

        if tref.definition.borrow().is_some() {
            errs.add_error(MultipleDefinitionsError {
                loc: def.name.clone(),
                first: tref.loc.name.clone(),
                kind: DefinitionKind::Type,
            });
            continue;
        }

        let kind = build_type(&ty.ty, tsys, errs);

        *tref.definition.borrow_mut() = Some(TypeInfo::Owned {
            kind: TypeKind::Named(Box::new(kind)),
        });
    }

    tsys.types
        .borrow()
        .iter()
        .filter(|(_, v)| v.definition.borrow().is_none())
        .for_each(|(_, v)| errs.add_error(UnresolvedTypeError(v.loc.name.clone())))
}

// fn resolve_imports<'a, 'gc>(
//     imps: &[Imports<'a>],
//     root: &'gc Module<'a, 'gc>,
//     tsys: &'gc Module<'a, 'gc>,
//     errs: &ErrorCollector<'a>,
// ) {
//     for imp in imps {
//         match imp {
//             Imports::Single(single) => {
//                 // import paths can not be absolute
//                 let (_, path) = conv_path(tsys, single);
//
//                 let name = path.last().expect("imported paths may not be empty");
//                 let path = path.slice(0..path.len() - 1);
//
//                 let md = root.access_or_create_module(path);
//                 tsys.imports.borrow_mut().insert(name.clone(), md);
//             }
//             Imports::Multiple(offset, imps) => {
//                 let (_, path) = conv_path(tsys, offset);
//                 let rel_root = root.access_or_create_module(&path);
//                 resolve_imports(imps, rel_root, tsys, errs);
//             }
//         }
//     }
// }

fn build_type<'a, 'gc>(
    ty: &ItemTy<'a>,
    tsys: &'gc Module<'a, 'gc>,
    errs: &ErrorCollector<'a>,
) -> TypeKind<'a, 'gc> {
    match ty {
        ItemTy::Referred(reference) => {
            let (root, path) = conv_path(tsys, reference);
            TypeKind::Referred(root.access_or_create_type(&path))
        }
        ItemTy::Struct(TyStruct {
            span_struct,
            fields,
        }) => TypeKind::Struct(TypeStruct {
            def: *span_struct,
            fields: fields
                .iter()
                .map(|NameTyPair { name, ty }| (name.clone(), build_type(ty, tsys, errs)))
                .collect(),
        }),
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
            args: args.iter().map(|ty| build_type(ty, tsys, errs)).collect(),
            ret: Box::new(build_type(ret_ty, tsys, errs)),
        }),
        ItemTy::Array(TyArray { span, ty, size }) => TypeKind::Array(TypeArray {
            def: *span,
            ty: Box::new(build_type(ty, tsys, errs)),
            len: if let Some(num) = size {
                match array_num_to_sized(&*num) {
                    Ok(n) => Some(n),
                    Err(err) => {
                        errs.add_error(ArrayNumberFix {
                            loc: num.number,
                            msg: err,
                        });
                        None
                    }
                }
            } else {
                None
            },
        }),
        ItemTy::Pointer(TyPtr { span_ptr, ty }) => TypeKind::Ptr(TypePtr {
            def: *span_ptr,
            ty: Box::new(build_type(ty, tsys, errs)),
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
        fields: types.iter().map(|ty| build_type(ty, tsys, errs)).collect(),
    }
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

fn array_num_to_sized(num: &Number) -> Result<u64, Cow<'static, str>> {
    if let Some(sign) = &num.sign {
        if **sign != "+" {
            return Err("Unsigned integers must be positive".into());
        }
    }
    if let Some(suffix) = &num.suffix {
        if **suffix != "usize" {
            return Err("Only usize numbers are allowed as array size".into());
        }
    }

    let base = num
        .base
        .as_ref()
        .map(|span| match **span {
            "0x" => 16,
            "0o" => 8,
            "0b" => 2,
            _ => unreachable!("allowed bases exceeded"),
        })
        .unwrap_or(10);

    let num = if num.number.find('_').is_some() {
        Cow::Owned(num.number.replace('_', ""))
    } else {
        Cow::Borrowed(*num.number)
    };

    u64::from_str_radix(&num, base).map_err(|err| format!("Number out of scope: {err}").into())
}
