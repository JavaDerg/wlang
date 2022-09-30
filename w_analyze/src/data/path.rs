use std::borrow::Borrow;
use std::mem::transmute;
use std::ops::{Bound, Deref, RangeBounds};
use std::slice::Iter;
use w_parse::Ident;

#[repr(transparent)]
#[derive(Hash, PartialEq, Eq)]
pub struct Path {
    path: [Ident],
}

#[derive(Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct PathBuf {
    path: Vec<Ident>,
}

impl PathBuf {
    fn new() -> Self {
        Default::default()
    }
}

impl PathBuf {
    pub fn append(&mut self, ident: Ident) {
        self.path.push(ident);
    }

    pub fn append_path(&mut self, path: &Path) {
        self.path.extend(path.path.iter().cloned());
    }
}

impl Path {
    pub fn join(&self, other: Ident) -> PathBuf {
        let mut path = self.to_owned();
        path.append(other);
        path
    }

    pub fn join_path(&self, other: &Path) -> PathBuf {
        let mut path = self.to_owned();
        path.append_path(other);
        path
    }

    pub fn slice(&self, range: impl RangeBounds<usize>) -> &Path {
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

impl ToString for Path {
    fn to_string(&self) -> String {
        let mut buf = String::new();

        let mut iter = self.path.iter();

        if let Some(ident) = iter.next() {
            buf.push_str(&*ident.0);
        }
        for ident in iter {
            buf.push(':');
            buf.push_str(&*ident.0);
        }

        buf
    }
}

impl<'a, V: Into<Vec<Ident>>> From<V> for PathBuf {
    fn from(vec: V) -> Self {
        Self { path: vec.into() }
    }
}

impl Deref for PathBuf {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        unsafe { transmute(self.path.as_slice()) }
    }
}

impl Deref for Path {
    type Target = [Ident];

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}

impl Borrow<Path> for PathBuf {
    fn borrow(&self) -> &Path {
        self
    }
}

impl ToOwned for Path {
    type Owned = PathBuf;

    fn to_owned(&self) -> Self::Owned {
        PathBuf {
            path: self.path.to_vec(),
        }
    }
}

impl<'a, 'b> IntoIterator for &'b Path {
    type Item = &'b Ident;
    type IntoIter = Iter<'b, Ident>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.path).iter()
    }
}
