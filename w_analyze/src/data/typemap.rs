use crate::data::types::{
    TypeArray, TypeEnum, TypeFunc, TypeInfo, TypeKind, TypePtr, TypeRef, TypeStruct, TypeTuple,
};
use crate::Module;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Default)]
pub struct AnonTypeMap<'a, 'gc> {
    map: HashMap<Wrapper<'a, 'gc>, &'gc TypeRef<'a, 'gc>>,
}

struct Wrapper<'a, 'gc> {
    pub inner: TypeKind<'a, 'gc>,
    pub owner: &'gc Module<'a, 'gc>,
}

impl<'a, 'gc> AnonTypeMap<'a, 'gc> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a, 'gc> Hash for Wrapper<'a, 'gc> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self.inner {
            TypeKind::Named(tref) => Wrapper {
                inner: tref
                    .definition
                    .borrow()
                    .expect("only defined types can be hashed")
                    .kind
                    .clone(),
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
                len.hash(state);
            }
            TypeKind::Enum(TypeEnum { variants, .. }) => {
                for (ident, tuple) in variants {
                    (**ident.0).hash(state);
                    if let Some(tuple) = tuple {
                        for ty in &tuple.fields {
                            Wrapper {
                                inner: *ty,
                                owner: self.owner,
                            }
                            .hash(state);
                        }
                    }
                }
            }
            TypeKind::Func(TypeFunc { args, ret, .. }) => {
                for ty in args {
                    Wrapper {
                        inner: *ty,
                        owner: self.owner,
                    }
                    .hash(state);
                }
                Wrapper {
                    inner: ret,
                    owner: self.owner,
                }
                .hash(state);
            }
            // Never is always the same
            TypeKind::Never(_) => (),
            TypeKind::Ptr(TypePtr { ty, .. }) => Wrapper {
                inner: ty,
                owner: self.owner,
            }
            .hash(state),
            TypeKind::Struct(TypeStruct { fields, .. }) => {
                for (ident, ty) in fields {
                    (**ident.0).hash(state);
                    Wrapper {
                        inner: ty,
                        owner: self.owner,
                    }
                    .hash(state);
                }
            }
            TypeKind::Tuple(TypeTuple { fields, .. }) => {
                for ty in fields {
                    Wrapper {
                        inner: ty,
                        owner: self.owner,
                    }
                    .hash(state);
                }
            }
        }
    }
}

macro_rules! match_or_die {
    ($val:expr, $pt:pat_param => $ex:expr) => {
        (match $val {
            $pt => $ex,
            _ => return false,
        })
    };
}

impl<'a, 'gc> PartialEq for Wrapper<'a, 'gc> {
    fn eq(&self, other: &Self) -> bool {
        let ty1 = self.inner.definition.borrow();
        let ty2 = other.inner.definition.borrow();

        if ty1.is_none() || ty2.is_none() {
            return false;
        }

        let ty1 = ty1.as_ref().unwrap();
        let ty2 = ty2.as_ref().unwrap();

        match &ty1.kind {
            TypeKind::Named(ty1) => {
                let ty2 = match_or_die!(&ty2.kind, TypeKind::Named(ty2) => ty2);
                Wrapper {
                    inner: ty1,
                    owner: self.owner,
                }
                .eq(&Wrapper {
                    inner: ty2,
                    owner: other.owner,
                })
            }
            TypeKind::Referred(ty1) => {
                let ty2 = match_or_die!(&ty2.kind, TypeKind::Referred(ty2) => ty2);
                Wrapper {
                    inner: ty1,
                    owner: self.owner,
                }
                .eq(&Wrapper {
                    inner: ty2,
                    owner: other.owner,
                })
            }
            TypeKind::Import(ty1) => {
                let ty2 = match_or_die!(&ty2.kind, TypeKind::Import(ty2) => ty2);
                Wrapper {
                    inner: ty1,
                    owner: self.owner,
                }
                .eq(&Wrapper {
                    inner: ty2,
                    owner: other.owner,
                })
            }
            TypeKind::Array(ty1) => {
                let ty2 = match_or_die!(&ty2.kind, TypeKind::Array(ty2) => ty2);
                Wrapper {
                    inner: ty1.ty,
                    owner: self.owner,
                }
                .eq(&Wrapper {
                    inner: ty2.ty,
                    owner: other.owner,
                }) && ty1.len == ty2.len
            }
            TypeKind::Enum(ty1) => {
                let ty2 = match_or_die!(&ty2.kind, TypeKind::Enum(ty2) => ty2);
                ty1.variants.len() == ty2.variants.len()
                    && ty1.variants.iter().zip(ty2.variants.iter()).all(
                        |((ident1, tuple1), (ident2, tuple2))| {
                            (**ident1.0) == (**ident2.0)
                                && match (tuple1, tuple2) {
                                    (Some(tuple1), Some(tuple2)) => {
                                        tuple1.fields.iter().zip(tuple2.fields.iter()).all(
                                            |(ty1, ty2)| {
                                                Wrapper {
                                                    inner: ty1,
                                                    owner: self.owner,
                                                }
                                                .eq(&Wrapper {
                                                    inner: ty2,
                                                    owner: other.owner,
                                                })
                                            },
                                        )
                                    }
                                    (None, None) => true,
                                    _ => false,
                                }
                        },
                    )
            }
            TypeKind::Func(ty1) => {
                let ty2 = match_or_die!(&ty2.kind, TypeKind::Func(ty2) => ty2);
                ty1.args.len() == ty2.args.len()
                    && ty1.args.iter().zip(ty2.args.iter()).all(|(ty1, ty2)| {
                        Wrapper {
                            inner: ty1,
                            owner: self.owner,
                        }
                        .eq(&Wrapper {
                            inner: ty2,
                            owner: other.owner,
                        })
                    })
                    && Wrapper {
                        inner: ty1.ret,
                        owner: self.owner,
                    }
                    .eq(&Wrapper {
                        inner: ty2.ret,
                        owner: other.owner,
                    })
            }
            TypeKind::Never(_) => matches!(ty2.kind, TypeKind::Never(_)),
            TypeKind::Ptr(ty1) => {
                let ty2 = match_or_die!(&ty2.kind, TypeKind::Ptr(ty2) => ty2);
                Wrapper {
                    inner: ty1.ty,
                    owner: self.owner,
                }
                .eq(&Wrapper {
                    inner: ty2.ty,
                    owner: other.owner,
                })
            }
            TypeKind::Struct(ty1) => {
                let ty2 = match_or_die!(&ty2.kind, TypeKind::Struct(ty2) => ty2);
                ty1.fields.len() == ty2.fields.len()
                    && ty1.fields.iter().zip(ty2.fields.iter()).all(
                        |((ident1, ty1), (ident2, ty2))| {
                            (**ident1.0) == (**ident2.0)
                                && Wrapper {
                                    inner: ty1,
                                    owner: self.owner,
                                }
                                .eq(&Wrapper {
                                    inner: ty2,
                                    owner: other.owner,
                                })
                        },
                    )
            }
            TypeKind::Tuple(ty1) => {
                let ty2 = match_or_die!(&ty2.kind, TypeKind::Tuple(ty2) => ty2);
                ty1.fields.len() == ty2.fields.len()
                    && ty1.fields.iter().zip(ty2.fields.iter()).all(|(ty1, ty2)| {
                        Wrapper {
                            inner: ty1,
                            owner: self.owner,
                        }
                        .eq(&Wrapper {
                            inner: ty2,
                            owner: other.owner,
                        })
                    })
            }
        }
    }
}
