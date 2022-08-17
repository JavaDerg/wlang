use crate::item::{parse_item, Item};
use crate::{ParResult, TokenSpan};

// Leaving this open for mode things in the future like imports

pub struct Module<'a> {
    pub items: Vec<Item<'a>>,
}

pub fn parse_module(mut i: TokenSpan) -> ParResult<Module> {
    let mut items = vec![];

    while !i.is_empty() {
        let (ni, item) = parse_item(i)?;

        items.push(item);

        i = ni;
    }

    Ok((i, Module { items }))
}
