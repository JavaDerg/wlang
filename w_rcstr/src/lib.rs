#[cfg(test)]
mod tests;

use nom::{
    AsBytes, Compare, CompareResult, ExtendInto, FindToken, InputIter, InputLength, InputTake,
    Needed, Offset, Slice, UnspecializedInput,
};
use std::borrow::Borrow;
use std::collections::Bound;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, Range, RangeBounds};
use std::sync::Arc;

#[derive(Clone)]
pub enum Origin {
    Unknown,
}

#[derive(Clone)]
pub struct RcStr {
    str: Arc<String>,
    frag: Range<usize>,
    origin: Origin,
}

pub struct RcChars(RcCharsIndices);
pub struct RcCharsIndices(RcStr, usize);

impl Iterator for RcChars {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.0.next()?.1)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.0.count()
    }
}

impl Iterator for RcCharsIndices {
    type Item = (usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.frag.start >= self.0.frag.end {
            return None;
        }

        let c = (*self.0).chars().next()?;
        self.0.frag.start += c.len_utf8();

        Some((self.0.frag.start - self.1, c))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.0.len(), Some(self.0.len()))
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.0.len()
    }
}

impl RcStr {
    pub fn new(str: String, origin: Origin) -> Self {
        let str = Arc::new(str);

        Self {
            origin,
            frag: 0..str.len(),
            str,
        }
    }
}

impl Deref for RcStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.str[self.frag.clone()]
    }
}

impl AsBytes for RcStr {
    fn as_bytes(&self) -> &[u8] {
        (**self).as_bytes()
    }
}

impl Compare<RcStr> for RcStr {
    fn compare(&self, t: RcStr) -> CompareResult {
        (&**self).compare(&*t)
    }

    fn compare_no_case(&self, t: RcStr) -> CompareResult {
        (&**self).compare_no_case(&*t)
    }
}

impl ExtendInto for RcStr {
    type Item = char;
    type Extender = String;

    fn new_builder(&self) -> Self::Extender {
        String::new()
    }

    fn extend_into(&self, acc: &mut Self::Extender) {
        acc.push_str(self);
    }
}

impl FindToken<char> for RcStr {
    fn find_token(&self, token: char) -> bool {
        self.contains(token)
    }
}

impl InputIter for RcStr {
    type Item = char;
    type Iter = RcCharsIndices;
    type IterElem = RcChars;

    fn iter_indices(&self) -> Self::Iter {
        RcCharsIndices(self.clone(), self.frag.start)
    }

    fn iter_elements(&self) -> Self::IterElem {
        RcChars(self.iter_indices())
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        (&**self).position(predicate)
    }

    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        (&**self).slice_index(count)
    }
}

impl InputLength for RcStr {
    fn input_len(&self) -> usize {
        self.frag.end - self.frag.start
    }
}

impl InputTake for RcStr {
    fn take(&self, count: usize) -> Self {
        assert!(count <= self.input_len());

        Self {
            origin: self.origin.clone(),
            str: self.str.clone(),
            frag: self.frag.start..self.frag.start + count,
        }
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        assert!(count <= self.input_len());
        let split = self.frag.start + count;

        (
            Self {
                origin: self.origin.clone(),
                str: self.str.clone(),
                frag: self.frag.start..split,
            },
            Self {
                origin: self.origin.clone(),
                str: self.str.clone(),
                frag: split..self.frag.end,
            },
        )
    }
}

impl UnspecializedInput for RcStr {}

impl Offset for RcStr {
    fn offset(&self, second: &Self) -> usize {
        second.frag.start - self.frag.start
    }
}

impl PartialEq<str> for RcStr {
    fn eq(&self, other: &str) -> bool {
        *self == other
    }
}

impl<'a> PartialEq<&'a str> for RcStr {
    fn eq(&self, other: &&'a str) -> bool {
        **self == **other
    }
}

impl<'a> Compare<&'a str> for RcStr {
    fn compare(&self, t: &'a str) -> CompareResult {
        (&**self).compare(t)
    }

    fn compare_no_case(&self, t: &'a str) -> CompareResult {
        (&**self).compare_no_case(t)
    }
}

impl<R: RangeBounds<usize>> Slice<R> for RcStr
where
    &'static str: Slice<R>,
{
    fn slice(&self, range: R) -> Self {
        let start = match range.start_bound() {
            Bound::Included(i) => *i,
            Bound::Excluded(_) => unreachable!("this should be unreachable, right?!"),
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(i) => i
                .checked_add(1)
                .expect("strings cant be larger than the max of its architecture"),
            Bound::Excluded(i) => *i,
            Bound::Unbounded => self.frag.end - self.frag.start,
        };

        assert!(start <= end);
        assert!(self.frag.start + end <= self.frag.end);

        Self {
            origin: self.origin.clone(),
            str: self.str.clone(),
            frag: self.frag.start + start..self.frag.start + end,
        }
    }
}

impl PartialEq for RcStr {
    fn eq(&self, other: &Self) -> bool {
        self.frag == other.frag
    }
}

impl Eq for RcStr {}

impl Display for RcStr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &**self)
    }
}

impl Debug for RcStr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &**self)
    }
}

impl Borrow<str> for RcStr {
    fn borrow(&self) -> &str {
        self.deref()
    }
}

impl From<String> for RcStr {
    fn from(s: String) -> Self {
        Self::new(s, Origin::Unknown)
    }
}

impl From<&str> for RcStr {
    fn from(s: &str) -> Self {
        Self::new(s.to_string(), Origin::Unknown)
    }
}
