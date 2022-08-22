use crate::expr::parse_many0;
use crate::expr::path::{parse_path, Path};
use crate::{tag, ParResult, TokenSpan, Weak};
use nom::branch::alt;
use nom::combinator::{all_consuming, map};
use nom::sequence::tuple;
use nom::Parser;
use w_tokenize::Kind;

#[derive(Debug, Clone)]
pub enum Imports<'a> {
    Single(Path<'a>),
    Multiple(Path<'a>, Vec<Imports<'a>>),
}

#[derive(Debug, Clone)]
pub struct ItemImports<'a> {
    pub imports: Vec<Imports<'a>>,
    pub from: Path<'a>,
}

pub fn parse_item_import(i: TokenSpan) -> ParResult<ItemImports> {
    let (i, imports) = parse_imports(i)?;
    let (i, _) = Weak(Kind::DoubleCol).parse(i)?;
    let (i, from) = parse_path(i)?;

    Ok((i, ItemImports {
        imports,
        from
    }))
}

fn parse_imports(i: TokenSpan) -> ParResult<Vec<Imports>> {
    let (i, block) = tag!(Kind::Block(_), Token { kind: Kind::Block(vals), .. } => vals)(i)?;
    let block = TokenSpan::new(i.file, block);

    let (_, imports) = all_consuming(parse_many0(alt((
        map(parse_path, Imports::Single),
        map(
            tuple((parse_path, Weak(Kind::Colon), parse_imports)),
            |(base, _, leaves)| Imports::Multiple(base, leaves),
        ),
    ))))(block)?;

    Ok((i, imports))
}
