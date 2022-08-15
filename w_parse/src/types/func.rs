use nom::combinator::{all_consuming, map};
use w_tokenize::Span;
use crate::{ItemTy, ParResult, parse_keyword, parse_type, tag, TokenSpan};
use crate::expr::parse_many0;
use crate::util::{NameTyPair, parse_name_ty_pair};

pub struct TyFunc<'a> {
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

pub fn parse_func_args(i: TokenSpan) -> ParResult<Vec<NameTyPair>> {
    let (i, tuple) = tag!(Kind::Tuple(_), Token { kind: Kind::Tuple(vals), .. } => vals)(i)?;
    let tks = TokenSpan::new(i.file.clone(), tuple);
    let (_, args) = all_consuming(parse_many0(parse_name_ty_pair))(tks)?;

    Ok((i, args))
}
