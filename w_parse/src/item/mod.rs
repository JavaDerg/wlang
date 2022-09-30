use crate::item::import::ItemImports;
use crate::item::named::ItemNamed;
use crate::{ParResult, TokenSpan};
use nom::branch::alt;
use nom::combinator::map;

pub mod func;
pub mod import;
pub mod named;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum Item {
    Definer(ItemNamed),
    Import(ItemImports),
}

pub fn parse_item(i: TokenSpan) -> ParResult<Item> {
    alt((
        map(named::parse_named, Item::Definer),
        map(import::parse_item_import, Item::Import),
    ))(i)
}
