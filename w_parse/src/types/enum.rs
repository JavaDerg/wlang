use crate::expr::parse_many0;
use crate::types::tuple::{parse_ty_tuple, TyTuple};
use crate::{parse_keyword, parse_name, tag, Ident, ParResult, TokenSpan};
use nom::combinator::{all_consuming, opt};
use nom::sequence::pair;
use w_tokenize::Span;

pub struct TyEnum<'a> {
    pub span_enum: Span<'a>,
    pub variants: Vec<(Ident<'a>, Option<TyTuple<'a>>)>,
}

pub fn parse_ty_enum(i: TokenSpan) -> ParResult<TyEnum> {
    let (i, span_enum) = parse_keyword("enum")(i)?;

    let (i, block) = tag!(Kind::Block(_), Token { kind: Kind::Block(vals), .. } => vals)(i)?;
    let block = TokenSpan::new(i.file, block);

    let (_, variants) = all_consuming(parse_many0(pair(parse_name, opt(parse_ty_tuple))))(block)?;

    Ok((
        i,
        TyEnum {
            span_enum,
            variants,
        },
    ))
}
