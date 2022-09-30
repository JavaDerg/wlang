use std::cell::RefCell;

use w_tokenize::Span;

use crate::data::Location;
use crate::PathBuf;
use w_parse::Ident;

pub struct TypeRef<'gc> {
    pub loc: Location<'gc>,
    pub definition: RefCell<Option<TypeInfo<'gc>>>,
}

pub enum TypeInfo<'gc> {
    Owned { kind: TypeKind<'gc> },
    Proxy(&'gc TypeRef<'gc>),
}

#[derive(Clone)]
pub enum TypeKind<'gc> {
    Referred(&'gc TypeRef<'gc>, PathBuf),
    Array(TypeArray<'gc>),
    Enum(TypeEnum<'gc>),
    Func(TypeFunc<'gc>),
    Never(TypeNever),
    Ptr(TypePtr<'gc>),
    Struct(TypeStruct<'gc>),
    Tuple(TypeTuple<'gc>),
}

#[derive(Clone)]
pub struct TypeArray<'gc> {
    pub def: Span,
    pub ty: Box<TypeKind<'gc>>,
    pub len: Option<u64>,
}

#[derive(Clone)]
pub struct TypeEnum<'gc> {
    pub def: Span,
    pub variants: Vec<(Ident, Option<TypeTuple<'gc>>)>,
}

#[derive(Clone)]
pub struct TypeFunc<'gc> {
    pub def: Span,
    pub args: Vec<TypeKind<'gc>>,
    pub ret: Box<TypeKind<'gc>>,
}

#[derive(Clone)]
pub struct TypePtr<'gc> {
    pub def: Span,
    pub ty: Box<TypeKind<'gc>>,
}

#[derive(Clone)]
pub struct TypeStruct<'gc> {
    pub def: Span,
    pub fields: Vec<(Ident, TypeKind<'gc>)>,
}

#[derive(Clone)]
pub struct TypeTuple<'gc> {
    pub def: Span,
    pub fields: Vec<TypeKind<'gc>>,
}

#[derive(Clone)]
pub struct TypeNever(pub Span);
