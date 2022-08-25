use crate::expr::path::ExprPath;
use crate::item::{parse_item, Item};
use crate::{ParResult, TokenSpan};

// Leaving this open for mode things in the future like imports

pub struct ParsedModule<'a> {
    pub path: ExprPath<'a>,
    pub items: Vec<Item<'a>>,
}

pub fn parse_module<'a>(
    mut i: TokenSpan<'a>,
    path: ExprPath<'a>,
) -> ParResult<'a, ParsedModule<'a>> {
    let mut items = vec![];

    while !i.is_empty() {
        let (ni, item) = parse_item(i)?;

        items.push(item);

        i = ni;
    }

    Ok((i, ParsedModule { path, items }))
}
