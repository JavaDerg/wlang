use crate::item::definer::ItemDefiner;
use crate::{ParResult, TokenSpan};
use nom::branch::alt;
use nom::combinator::map;

pub mod definer;
pub mod func;

pub enum Item<'a> {
    Definer(ItemDefiner<'a>),
}

pub fn parse_item(i: TokenSpan) -> ParResult<Item> {
    alt((map(definer::parse_definer, Item::Definer),))(i)
}
