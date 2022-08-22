use crate::{parse_name, Ident, ParResult, TokenSpan, Weak};
use nom::combinator::map;
use nom::multi::separated_list1;
use std::hash::{Hash, Hasher};
use w_tokenize::Kind;

#[derive(Debug, Clone)]
pub struct Path<'a> {
    pub path: Vec<Ident<'a>>,
}

pub fn parse_path(i: TokenSpan) -> ParResult<Path> {
    map(separated_list1(Weak(Kind::Colon), parse_name), |path| {
        Path { path }
    })(i)
}

impl<'a> Hash for Path<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for ident in &self.path {
            (*ident.0).hash(state);
        }
    }
}

impl<'a> Path<'a> {
    pub fn join_s(&self, other: &Ident<'a>) -> Path<'a> {
        Path {
            path: self
                .path
                .clone()
                .into_iter()
                .chain(vec![other.clone()])
                .collect(),
        }
    }

    pub fn join_p(&self, other: &Path<'a>) -> Path<'a> {
        Path {
            path: self
                .path
                .clone()
                .into_iter()
                .chain(other.path.iter().cloned())
                .collect(),
        }
    }
}

impl<'a> PartialEq for Path<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.path.len() != other.path.len() {
            return false;
        }
        self.path
            .iter()
            .zip(other.path.iter())
            .all(|(a, b)| *a.0 == *b.0)
    }
}

impl<'a> Eq for Path<'a> {}
