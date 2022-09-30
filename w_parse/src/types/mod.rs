pub mod array;
pub mod r#enum;
pub mod func;
pub mod never;
pub mod ptr;
pub mod r#struct;
pub mod tuple;

use crate::{ParResult, TokenSpan};

use nom::branch::alt;
use nom::combinator::map;

use crate::expr::path::{parse_path, ExprPath};
use crate::types::array::{parse_ty_array, TyArray};
use crate::types::func::{parse_ty_func, TyFunc};
use crate::types::never::{parse_ty_never, TyNever};
use crate::types::ptr::{parse_ty_ptr, TyPtr};
use crate::types::r#enum::{parse_ty_enum, TyEnum};
use crate::types::r#struct::{parse_ty_struct, TyStruct};
use crate::types::tuple::{parse_ty_tuple, TyTuple};

#[derive(Debug, Clone)]
pub enum ItemTy {
    Referred(ExprPath),
    Struct(TyStruct),
    Enum(TyEnum),
    Tuple(TyTuple),
    Func(TyFunc),
    Array(TyArray),
    Pointer(TyPtr),
    Never(TyNever),
}

pub fn parse_type(i: TokenSpan) -> ParResult<ItemTy> {
    alt((
        map(parse_path, ItemTy::Referred),
        map(parse_ty_struct, ItemTy::Struct),
        map(parse_ty_enum, ItemTy::Enum),
        map(parse_ty_tuple, ItemTy::Tuple),
        map(parse_ty_func, ItemTy::Func),
        map(parse_ty_array, ItemTy::Array),
        map(parse_ty_ptr, ItemTy::Pointer),
        map(parse_ty_never, ItemTy::Never),
    ))(i)
}
