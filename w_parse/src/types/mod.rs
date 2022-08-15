pub mod array;
pub mod r#enum;
pub mod never;
pub mod ptr;
pub mod r#struct;
pub mod tuple;
pub mod func;

use crate::{ParResult, TokenSpan};

use nom::branch::alt;
use nom::combinator::map;

use crate::expr::path::{parse_path, Path};
use crate::types::array::{parse_ty_array, TyArray};
use crate::types::func::{parse_ty_func, TyFunc};
use crate::types::never::{parse_ty_never, TyNever};
use crate::types::ptr::{parse_ty_ptr, TyPtr};
use crate::types::r#enum::{parse_ty_enum, TyEnum};
use crate::types::r#struct::{parse_ty_struct, TyStruct};
use crate::types::tuple::{parse_ty_tuple, TyTuple};

pub enum ItemTy<'a> {
    Named(Path<'a>),
    Struct(TyStruct<'a>),
    Enum(TyEnum<'a>),
    Tuple(TyTuple<'a>),
    Func(TyFunc<'a>),
    Array(TyArray<'a>),
    Pointer(TyPtr<'a>),
    Never(TyNever<'a>),
}

pub fn parse_type(i: TokenSpan) -> ParResult<ItemTy> {
    alt((
        map(parse_path, ItemTy::Named),
        map(parse_ty_struct, ItemTy::Struct),
        map(parse_ty_enum, ItemTy::Enum),
        map(parse_ty_tuple, ItemTy::Tuple),
        map(parse_ty_func, ItemTy::Func),
        map(parse_ty_array, ItemTy::Array),
        map(parse_ty_ptr, ItemTy::Pointer),
        map(parse_ty_never, ItemTy::Never),
    ))(i)
}
