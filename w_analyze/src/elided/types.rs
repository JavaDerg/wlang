use crate::data::types::{TypeArray, TypeEnum, TypeFunc, TypeKind, TypePtr, TypeStruct, TypeTuple};
use crate::PathBuf;
use std::collections::HashMap;
use w_parse::Ident;
use w_tokenize::Span;

pub struct TypeSystem<'a> {
    pub flat: HashMap<PathBuf<'a>, ElidedType<'a>>,
}

pub struct ElidedType<'a> {
    pub loc: PathBuf<'a>,

    pub def: Span<'a>,
    pub kind: ETyKind<'a>,
}

pub enum ETyKind<'a> {
    Referred(PathBuf<'a>),
    Array(ETyArray<'a>),
    Enum(ETyEnum<'a>),
    Func(ETyFunc<'a>),
    Never,
    Ptr(ETyPtr<'a>),
    Struct(ETyStruct<'a>),
    Tuple(ETyTuple<'a>),
}

pub struct ETyArray<'a> {
    pub ty: Box<ETyKind<'a>>,
    pub len: Option<u64>,
}

pub struct ETyEnum<'a> {
    pub variants: Vec<(Ident<'a>, Option<ETyTuple<'a>>)>,
}

pub struct ETyFunc<'a> {
    pub args: Vec<ETyKind<'a>>,
    pub ret: Box<ETyKind<'a>>,
}

pub struct ETyPtr<'a> {
    pub ty: Box<ETyKind<'a>>,
}

pub struct ETyStruct<'a> {
    pub fields: Vec<(Ident<'a>, ETyKind<'a>)>,
}

pub struct ETyTuple<'a> {
    pub fields: Vec<ETyKind<'a>>,
}

pub fn elide_type_kind<'a, 'gc>(ty: TypeKind<'a, 'gc>) -> ETyKind<'a> {
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

pub fn elide_tuple_kind<'a, 'gc>(ty: TypeTuple<'a, 'gc>) -> ETyTuple<'a> {
    ETyTuple {
        fields: ty.fields.into_iter().map(elide_type_kind).collect(),
    }
}
