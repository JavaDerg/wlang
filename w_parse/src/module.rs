use crate::expr::path::ExprPath;
use crate::item::{parse_item, Item};
use crate::{Ident, ParResult, TokenSpan};

// Leaving this open for mode things in the future like imports

pub struct ParsedModule<'a> {
    pub name: Ident<'a>,
    pub items: Vec<Item<'a>>,
}

pub fn parse_module<'a>(mut i: TokenSpan<'a>, name: Ident<'a>) -> ParResult<'a, ParsedModule<'a>> {
    let mut items = vec![];

    while !i.is_empty() {
        let (ni, item) = parse_item(i)?;

        items.push(item);

        i = ni;
    }

    Ok((i, ParsedModule { name, items }))
}
