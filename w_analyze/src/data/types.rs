use crate::data::Location;
use either::Either;
use std::cell::RefCell;

use w_tokenize::{Number, Span};

use w_parse::expr::Expr;
use w_parse::Ident;

pub struct TypeRef<'a, 'gc> {
    pub loc: Option<Location<'a, 'gc>>,
    pub definition: RefCell<Option<TypeInfo<'a, 'gc>>>,
}

pub struct TypeInfo<'a, 'gc> {
    pub kind: TypeKind<'a, 'gc>,
}

pub enum TypeKind<'a, 'gc> {
    Named(&'gc TypeRef<'a, 'gc>),
    Referred(&'gc TypeRef<'a, 'gc>),
    Import(&'gc TypeRef<'a, 'gc>),
    Array(TypeArray<'a, 'gc>),
    Enum(TypeEnum<'a, 'gc>),
    Func(TypeFunc<'a, 'gc>),
    Never(TypeNever<'a>),
    Ptr(TypePtr<'a, 'gc>),
    Struct(TypeStruct<'a, 'gc>),
    Tuple(TypeTuple<'a, 'gc>),
}

pub struct TypeArray<'a, 'gc> {
    pub def: Span<'a>,
    pub ty: &'gc TypeRef<'a, 'gc>,
    pub len: Option<Number<'a>>,
}

pub struct TypeEnum<'a, 'gc> {
    pub def: Span<'a>,
    pub variants: Vec<(Ident<'a>, Option<TypeTuple<'a, 'gc>>)>,
}

pub struct TypeFunc<'a, 'gc> {
    pub def: Span<'a>,
    pub args: Vec<&'gc TypeRef<'a, 'gc>>,
    pub ret: &'gc TypeRef<'a, 'gc>,
}

pub struct TypePtr<'a, 'gc> {
    pub def: Span<'a>,
    pub ty: &'gc TypeRef<'a, 'gc>,
}

pub struct TypeStruct<'a, 'gc> {
    pub def: Span<'a>,
    pub fields: Vec<(Ident<'a>, &'gc TypeRef<'a, 'gc>)>,
}

pub struct TypeTuple<'a, 'gc> {
    pub def: Span<'a>,
    pub fields: Vec<&'gc TypeRef<'a, 'gc>>,
}

pub struct TypeNever<'a>(pub Span<'a>);
