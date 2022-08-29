use std::borrow::Borrow;
use std::mem::transmute;
use std::ops::{Bound, Deref, RangeBounds};
use std::slice::Iter;
use w_parse::Ident;

#[repr(transparent)]
pub struct Path<'a> {
    path: [Ident<'a>],
}

#[derive(Clone, Debug, Default)]
pub struct PathBuf<'a> {
    path: Vec<Ident<'a>>,
}

impl<'a> PathBuf<'a> {
    fn new() -> Self {
        Default::default()
    }
}

impl<'a> PathBuf<'a> {
    pub fn append(&mut self, ident: Ident<'a>) {
        self.path.push(ident);
    }

    pub fn append_path(&mut self, path: &Path<'a>) {
        self.path.extend(path.path.iter().cloned());
    }
}

impl<'a> Path<'a> {
    pub fn join(&self, other: Ident<'a>) -> PathBuf<'a> {
        let mut path = self.to_owned();
        path.append(other);
        path
    }

    pub fn join_path(&self, other: &Path<'a>) -> PathBuf<'a> {
        let mut path = self.to_owned();
        path.append_path(other);
        path
    }

    pub fn slice(&self, range: impl RangeBounds<usize>) -> &Path<'a> {
        let start = match range.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&n) => n + 1,
            Bound::Excluded(&n) => n,
            Bound::Unbounded => self.path.len(),
        };

        let slice = &self.path[start..end];

        unsafe { transmute(slice) }
    }
}

impl<'a, V: Into<Vec<Ident<'a>>>> From<V> for PathBuf<'a> {
    fn from(vec: V) -> Self {
        Self { path: vec.into() }
    }
}

impl<'a> Deref for PathBuf<'a> {
    type Target = Path<'a>;

    fn deref(&self) -> &Self::Target {
        unsafe { transmute(self.path.as_slice()) }
    }
}

impl<'a> Deref for Path<'a> {
    type Target = [Ident<'a>];

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl<'a> Borrow<Path<'a>> for PathBuf<'a> {
    fn borrow(&self) -> &Path<'a> {
        self
    }
}

impl<'a> ToOwned for Path<'a> {
    type Owned = PathBuf<'a>;

    fn to_owned(&self) -> Self::Owned {
        PathBuf {
            path: self.path.to_vec(),
        }
    }
}

impl<'a, 'b> IntoIterator for &'b Path<'a> {
    type Item = &'b Ident<'a>;
    type IntoIter = Iter<'b, Ident<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.path).iter()
    }
}
