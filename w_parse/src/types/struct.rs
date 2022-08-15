use nom::combinator::{all_consuming, map};
use w_tokenize::Span;
use crate::{ParResult, parse_keyword, tag, TokenSpan};
use crate::expr::parse_many0;
use crate::util::{NameTyPair, parse_name_ty_pair};

pub struct TyStruct<'a> {
    pub span_struct: Span<'a>,
    pub fields: Vec<NameTyPair<'a>>,
}

pub fn parse_ty_struct(i: TokenSpan) -> ParResult<TyStruct> {
    let (i, span_struct) = map(parse_keyword("struct"), |id| id.0)(i)?;

    let (i, block) = tag!(Kind::Block(_), Token { kind: Kind::Block(vals), .. } => vals)(i)?;
    let block = TokenSpan::new(i.file.clone(), block);

    let (_, fields) = all_consuming(parse_many0(parse_name_ty_pair))(block)?;

    Ok((i, TyStruct { span_struct, fields }))
}
