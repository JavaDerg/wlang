use std::cell::RefCell;

use w_tokenize::Span;

use crate::data::Location;
use crate::PathBuf;
use w_parse::Ident;

pub struct TypeRef<'a, 'gc> {
    pub loc: Location<'a, 'gc>,
    pub definition: RefCell<Option<TypeInfo<'a, 'gc>>>,
}

pub enum TypeInfo<'a, 'gc> {
    Owned { kind: TypeKind<'a, 'gc> },
    Proxy(&'gc TypeRef<'a, 'gc>),
}

#[derive(Clone)]
pub enum TypeKind<'a, 'gc> {
    Named(Box<TypeKind<'a, 'gc>>),
    Referred(&'gc TypeRef<'a, 'gc>, PathBuf<'a>),
    Array(TypeArray<'a, 'gc>),
    Enum(TypeEnum<'a, 'gc>),
    Func(TypeFunc<'a, 'gc>),
    Never(TypeNever<'a>),
    Ptr(TypePtr<'a, 'gc>),
    Struct(TypeStruct<'a, 'gc>),
    Tuple(TypeTuple<'a, 'gc>),
}

#[derive(Clone)]
pub struct TypeArray<'a, 'gc> {
    pub def: Span<'a>,
    pub ty: Box<TypeKind<'a, 'gc>>,
    pub len: Option<u64>,
}

#[derive(Clone)]
pub struct TypeEnum<'a, 'gc> {
    pub def: Span<'a>,
    pub variants: Vec<(Ident<'a>, Option<TypeTuple<'a, 'gc>>)>,
}

#[derive(Clone)]
pub struct TypeFunc<'a, 'gc> {
    pub def: Span<'a>,
    pub args: Vec<TypeKind<'a, 'gc>>,
    pub ret: Box<TypeKind<'a, 'gc>>,
}

#[derive(Clone)]
pub struct TypePtr<'a, 'gc> {
    pub def: Span<'a>,
    pub ty: Box<TypeKind<'a, 'gc>>,
}

#[derive(Clone)]
pub struct TypeStruct<'a, 'gc> {
    pub def: Span<'a>,
    pub fields: Vec<(Ident<'a>, TypeKind<'a, 'gc>)>,
}

#[derive(Clone)]
pub struct TypeTuple<'a, 'gc> {
    pub def: Span<'a>,
    pub fields: Vec<TypeKind<'a, 'gc>>,
}

#[derive(Clone)]
pub struct TypeNever<'a>(pub Span<'a>);
