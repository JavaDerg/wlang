use crate::data::Location;
use std::cell::RefCell;
use w_parse::types::array::TyArray;
use w_parse::types::func::TyFunc;
use w_parse::types::never::TyNever;
use w_parse::types::ptr::TyPtr;
use w_parse::types::r#enum::TyEnum;
use w_parse::types::r#struct::TyStruct;
use w_parse::types::tuple::TyTuple;

pub struct TypeRef<'a, 'gc> {
    pub loc: Location<'a>,
    pub definition: RefCell<Option<TypeInfo<'a, 'gc>>>,
}

pub struct TypeInfo<'a, 'gc> {
    pub kind: TypeKind<'a, 'gc>,
}

pub enum TypeKind<'a, 'gc> {
    Nested(&'gc TypeRef<'a, 'gc>),
    Array(TyArray<'a>),
    Enum(TyEnum<'a>),
    Func(TyFunc<'a>),
    Never(TyNever<'a>),
    Ptr(TyPtr<'a>),
    Struct(TyStruct<'a>),
    Tuple(TyTuple<'a>),
}
