use crate::data::types::{TypeArray, TypeKind, TypeRef};
use crate::Module;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub struct AnonTypeMap<'a, 'gc> {
    map: HashMap<Wrapper<'a, 'gc>, &'gc TypeRef<'a, 'gc>>,
}

struct Wrapper<'a, 'gc> {
    pub inner: &'gc TypeRef<'a, 'gc>,
    pub owner: &'gc Module<'a, 'gc>,
}

impl<'a, 'gc> Hash for Wrapper<'a, 'gc> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let ty = self.inner.definition.borrow();
        if let Some(ty) = &*ty {
            match &ty.kind {
                TypeKind::Named(tref) => Wrapper {
                    inner: *tref,
                    owner: self.owner,
                }
                .hash(state),
                TypeKind::Referred(tref) => Wrapper {
                    inner: *tref,
                    owner: self.owner,
                }
                .hash(state),
                TypeKind::Import(tref) => Wrapper {
                    inner: *tref,
                    owner: self.owner,
                }
                .hash(state),
                TypeKind::Array(TypeArray { ty, len, .. }) => {
                    Wrapper {
                        inner: ty,
                        owner: self.owner,
                    }
                    .hash(state);
                }
                TypeKind::Enum(_) => {}
                TypeKind::Func(_) => {}
                TypeKind::Never(_) => {}
                TypeKind::Ptr(_) => {}
                TypeKind::Struct(_) => {}
                TypeKind::Tuple(_) => {}
            }
        }
    }
}
