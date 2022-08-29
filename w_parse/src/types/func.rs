use crate::expr::parse_many0;
use crate::util::{parse_name_ty_pair, NameTyPair};
use crate::{parse_keyword, parse_type, tag, ItemTy, ParResult, TokenSpan};
use nom::combinator::{all_consuming, map};
use w_tokenize::Span;

#[derive(Debug, Clone)]
pub struct TyFunc<'a> {
    pub span_func: Span<'a>,
    pub args: Vec<ItemTy<'a>>,
    pub ret_ty: Box<ItemTy<'a>>,
}

#[derive(Debug, Clone)]
pub struct TyNamedFunc<'a> {
    pub span_func: Span<'a>,
    pub args: Vec<NameTyPair<'a>>,
    pub ret_ty: Box<ItemTy<'a>>,
}

pub fn parse_ty_func(i: TokenSpan) -> ParResult<TyFunc> {
    let (i, span_func) = parse_keyword("func")(i)?;

    let (i, args) = parse_func_args(i)?;
    let (i, ret_ty) = map(parse_type, Box::new)(i)?;

    Ok((
        i,
        TyFunc {
            span_func,
            args,
            ret_ty,
        },
    ))
}

pub fn parse_ty_named_func(i: TokenSpan) -> ParResult<TyNamedFunc> {
    let (i, span_func) = parse_keyword("func")(i)?;

    let (i, args) = parse_func_named_args(i)?;
    let (i, ret_ty) = map(parse_type, Box::new)(i)?;

    Ok((
        i,
        TyNamedFunc {
            span_func,
            args,
            ret_ty,
        },
    ))
}

pub fn parse_func_args(i: TokenSpan) -> ParResult<Vec<ItemTy>> {
    let (i, tuple) = tag!(Kind::Tuple(_), Token { kind: Kind::Tuple(vals), .. } => vals)(i)?;
    let tks = TokenSpan::new(i.file, tuple);
    let (_, args) = all_consuming(parse_many0(parse_type))(tks)?;

    Ok((i, args))
}

pub fn parse_func_named_args(i: TokenSpan) -> ParResult<Vec<NameTyPair>> {
    let (i, tuple) = tag!(Kind::Tuple(_), Token { kind: Kind::Tuple(vals), .. } => vals)(i)?;
    let tks = TokenSpan::new(i.file, tuple);
    let (_, args) = all_consuming(parse_many0(parse_name_ty_pair))(tks)?;

    Ok((i, args))
}
