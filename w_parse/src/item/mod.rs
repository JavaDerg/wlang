use crate::item::named::ItemNamed;
use crate::{ParResult, TokenSpan};
use nom::branch::alt;
use nom::combinator::map;

pub mod func;
pub mod named;

pub enum Item<'a> {
    Definer(ItemNamed<'a>),
}

pub fn parse_item(i: TokenSpan) -> ParResult<Item> {
    alt((map(named::parse_named, Item::Definer),))(i)
}
