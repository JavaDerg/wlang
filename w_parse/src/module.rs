use crate::expr::path::ExprPath;
use crate::item::{parse_item, Item};
use crate::{Ident, ParResult, TokenSpan};

// Leaving this open for mode things in the future like imports

pub struct ParsedModule {
    pub name: Ident,
    pub items: Vec<Item>,
}

pub fn parse_module(mut i: TokenSpan, name: Ident) -> ParResult<ParsedModule> {
    let mut items = vec![];

    while !i.is_empty() {
        let (ni, item) = parse_item(i)?;

        items.push(item);

        i = ni;
    }

    Ok((i, ParsedModule { name, items }))
}
