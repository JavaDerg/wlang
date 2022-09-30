use nom::error::{ErrorKind, ParseError};
use nom::{
    AsBytes, Compare, CompareResult, ExtendInto, FindToken, IResult, InputIter, InputLength,
    InputTake, InputTakeAtPosition, Needed, Offset, Slice, UnspecializedInput,
};
use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};
use std::iter::Enumerate;
use std::mem::{transmute, MaybeUninit};
use std::ops::{Deref, RangeBounds, RangeFrom};
use std::pin::Pin;
use std::rc::Rc;
use std::str::{CharIndices, Chars};
use std::sync::Arc;

#[derive(Clone)]
pub struct RcStr {
    /// may not change ever
    _origin: Arc<String>,
    fragment: &'static str,
}

impl RcStr {
    pub fn new(str: String) -> Self {
        let origin = Arc::new(str);

        let fragment = origin.as_str();

        Self {
            fragment: unsafe { transmute(fragment) },
            _origin: origin,
        }
    }
}

impl Deref for RcStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.fragment
    }
}

impl AsBytes for RcStr {
    fn as_bytes(&self) -> &[u8] {
        self.fragment.as_bytes()
    }
}

impl Compare<RcStr> for RcStr {
    fn compare(&self, t: RcStr) -> nom::CompareResult {
        self.fragment.compare(t.fragment)
    }

    fn compare_no_case(&self, t: RcStr) -> nom::CompareResult {
        self.fragment.compare_no_case(t.fragment)
    }
}

impl ExtendInto for RcStr {
    type Item = char;
    type Extender = String;

    fn new_builder(&self) -> Self::Extender {
        String::new()
    }

    fn extend_into(&self, acc: &mut Self::Extender) {
        acc.push_str(self.fragment);
    }
}

impl FindToken<char> for RcStr {
    fn find_token(&self, token: char) -> bool {
        self.fragment.contains(token)
    }
}

// This is not sound and may cause UB
impl InputIter for RcStr {
    type Item = char;
    type Iter = CharIndices<'static>;
    type IterElem = Chars<'static>;

    fn iter_indices(&self) -> Self::Iter {
        self.fragment.char_indices()
    }

    fn iter_elements(&self) -> Self::IterElem {
        self.fragment.chars()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.fragment.position(predicate)
    }

    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        self.fragment.slice_index(count)
    }
}

impl InputLength for RcStr {
    fn input_len(&self) -> usize {
        self.fragment.len()
    }
}

impl InputTake for RcStr {
    fn take(&self, count: usize) -> Self {
        Self {
            _origin: self._origin.clone(),
            fragment: self.fragment.take(count),
        }
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        let (left, right) = self.fragment.take_split(count);

        (
            Self {
                _origin: self._origin.clone(),
                fragment: left,
            },
            Self {
                _origin: self._origin.clone(),
                fragment: right,
            },
        )
    }
}

impl UnspecializedInput for RcStr {}

impl Offset for RcStr {
    fn offset(&self, second: &Self) -> usize {
        self.fragment.offset(second.fragment)
    }
}

impl RcStr {
    pub fn offset(&self, second: &str) -> usize {
        self.fragment.offset(second)
    }
}

impl PartialEq<str> for RcStr {
    fn eq(&self, other: &str) -> bool {
        self.fragment == other
    }
}

impl<'a> PartialEq<&'a str> for RcStr {
    fn eq(&self, other: &&'a str) -> bool {
        self.fragment == *other
    }
}

impl<'a> Compare<&'a str> for RcStr {
    fn compare(&self, t: &'a str) -> CompareResult {
        self.fragment.compare(t)
    }

    fn compare_no_case(&self, t: &'a str) -> CompareResult {
        self.fragment.compare_no_case(t)
    }
}

impl<R: RangeBounds<usize>> Slice<R> for RcStr
where
    &'static str: Slice<R>,
{
    fn slice(&self, range: R) -> Self {
        Self {
            _origin: self._origin.clone(),
            fragment: self.fragment.slice(range),
        }
    }
}

impl PartialEq for RcStr {
    fn eq(&self, other: &Self) -> bool {
        self.fragment == other.fragment
    }
}

impl Eq for RcStr {}

impl Display for RcStr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fragment)
    }
}

impl Debug for RcStr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fragment)
    }
}

impl Borrow<str> for RcStr {
    fn borrow(&self) -> &str {
        self.fragment
    }
}
