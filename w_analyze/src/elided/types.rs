use crate::PathBuf;
use std::collections::HashMap;

pub struct TypeSystem<'a> {
    flat: HashMap<PathBuf<'a>, ()>,
}
