// use crate::data::types::TypeKind;
// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::rc::Rc;
// use uuid::Uuid;
//
// pub struct TypeMap<'a> {
//     types: HashMap<Uuid, TypeHandle<'a>>,
// }
//
// #[derive(Clone)]
// pub struct TypeHandle<'a> {
//     // this looks cursed, because it is
//     inner: Rc<RefCell<Rc<RefCell<InnerHandle<'a>>>>>,
// }
//
// struct InnerHandle<'a> {
//     id: Uuid,
// }
