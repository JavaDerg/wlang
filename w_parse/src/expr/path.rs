use crate::{parse_name, tag, Ident, ParResult, TokenSpan, Weak};
use nom::combinator::{map, opt};
use nom::multi::separated_list1;
use nom::sequence::pair;
use std::hash::{Hash, Hasher};
use w_tokenize::{Kind, Span};

#[derive(Debug, Clone)]
pub struct ExprPath<'a> {
    pub root: Option<Span<'a>>,
    pub path: Vec<Ident<'a>>,
}

pub fn parse_path(i: TokenSpan) -> ParResult<ExprPath> {
    map(
        pair(
            opt(tag!(Kind::Colon)),
            separated_list1(Weak(Kind::Colon), parse_name),
        ),
        |(root, path)| ExprPath { root, path },
    )(i)
}

impl<'a> Hash for ExprPath<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.root.is_some().hash(state);
        for ident in &self.path {
            (*ident.0).hash(state);
        }
    }
}

impl<'a> PartialEq for ExprPath<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.path.len() != other.path.len() {
            return false;
        }
        self.root == other.root
            && self
                .path
                .iter()
                .zip(other.path.iter())
                .all(|(a, b)| *a.0 == *b.0)
    }
}

impl<'a> Eq for ExprPath<'a> {}
