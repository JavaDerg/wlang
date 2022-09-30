use crate::data::types::{TypeArray, TypeEnum, TypeFunc, TypeKind, TypePtr, TypeStruct, TypeTuple};
use crate::PathBuf;
use std::collections::HashMap;
use w_parse::Ident;
use w_tokenize::Span;

pub struct TypeSystem {
    pub flat: HashMap<PathBuf, ElidedType>,
}

pub struct ElidedType {
    pub loc: PathBuf,

    pub def: Span,
    pub kind: ETyKind,
}

pub enum ETyKind {
    Referred(PathBuf),
    Array(ETyArray),
    Enum(ETyEnum),
    Func(ETyFunc),
    Never,
    Ptr(ETyPtr),
    Struct(ETyStruct),
    Tuple(ETyTuple),
}

pub struct ETyArray {
    pub ty: Box<ETyKind>,
    pub len: Option<u64>,
}

pub struct ETyEnum {
    pub variants: Vec<(Ident, Option<ETyTuple>)>,
}

pub struct ETyFunc {
    pub args: Vec<ETyKind>,
    pub ret: Box<ETyKind>,
}

pub struct ETyPtr {
    pub ty: Box<ETyKind>,
}

pub struct ETyStruct {
    pub fields: Vec<(Ident, ETyKind)>,
}

pub struct ETyTuple {
    pub fields: Vec<ETyKind>,
}

pub fn elide_type_kind<'gc>(ty: TypeKind<'gc>) -> ETyKind {
    match ty {
        TypeKind::Referred(_, pb) => ETyKind::Referred(pb),
        TypeKind::Array(TypeArray { ty, len, .. }) => ETyKind::Array(ETyArray {
            ty: Box::new(elide_type_kind(*ty)),
            len,
        }),
        TypeKind::Enum(TypeEnum { variants, .. }) => ETyKind::Enum(ETyEnum {
            variants: variants
                .into_iter()
                .map(|(i, t)| (i, t.map(elide_tuple_kind)))
                .collect(),
        }),
        TypeKind::Func(TypeFunc { args, ret, .. }) => ETyKind::Func(ETyFunc {
            args: args.into_iter().map(elide_type_kind).collect(),
            ret: Box::new(elide_type_kind(*ret)),
        }),
        TypeKind::Never(_) => ETyKind::Never,
        TypeKind::Ptr(TypePtr { ty, .. }) => ETyKind::Ptr(ETyPtr {
            ty: Box::new(elide_type_kind(*ty)),
        }),
        TypeKind::Struct(TypeStruct { fields, .. }) => ETyKind::Struct(ETyStruct {
            fields: fields
                .into_iter()
                .map(|(i, t)| (i, elide_type_kind(t)))
                .collect(),
        }),
        TypeKind::Tuple(t) => ETyKind::Tuple(elide_tuple_kind(t)),
    }
}

pub fn elide_tuple_kind<'gc>(ty: TypeTuple<'gc>) -> ETyTuple {
    ETyTuple {
        fields: ty.fields.into_iter().map(elide_type_kind).collect(),
    }
}
