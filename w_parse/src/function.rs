use crate::expr::parse_many0;
use crate::{parse_keyword, parse_name, parse_type, tag, Ident, ParResult, TokenSpan, Type};
use nom::combinator::{all_consuming, map};
use w_tokenize::{Kind, Span};
use crate::expr::block::{ExprBlock, parse_block};

pub struct ItemFunc<'a> {
    pub span_func: Span<'a>,
    pub args: Vec<FuncArg<'a>>,
    pub ret_ty: Type<'a>,
    pub body: ExprBlock<'a>,
}

pub struct FuncArg<'a> {
    pub name: Ident<'a>,
    pub ty: Type<'a>,
}

pub fn parse_func(i: TokenSpan) -> ParResult<ItemFunc> {
    let (i, span_func) = map(parse_keyword("func"), |id| id.0)(i)?;

    let (i, args) = parse_func_args(i)?;
    let (i, ret_ty) = parse_type(i)?;

    let (i, body) = parse_block(i)?;

    Ok((
        i,
        ItemFunc {
            span_func,
            args,
            ret_ty,
            body,
        },
    ))
}

fn parse_func_args(i: TokenSpan) -> ParResult<Vec<FuncArg>> {
    let (i, tuple) = tag!(Kind::Tuple(_), Token { kind: Kind::Tuple(vals), .. } => vals)(i)?;
    let tks = TokenSpan::new(i.file.clone(), tuple);
    let (_, args) = all_consuming(parse_many0(parse_func_arg))(tks)?;

    Ok((i, args))
}

fn parse_func_arg(i: TokenSpan) -> ParResult<FuncArg> {
    let (i, name) = parse_name(i)?;
    let (i, ty) = parse_type(i)?;

    Ok((i, FuncArg { name, ty }))
}
