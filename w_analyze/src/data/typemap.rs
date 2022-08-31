use crate::data::types::TypeRef;
use crate::Module;
use std::collections::HashMap;

pub struct TypeMap<'a, 'gc> {
    map: HashMap<Wrapper<'a, 'gc>, &'gc TypeRef<'a, 'gc>>,
}

struct Wrapper<'a, 'gc> {
    pub inner: &'gc TypeRef<'a, 'gc>,
    pub owner: &'gc Module<'a, 'gc>,
}
