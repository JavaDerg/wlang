#![cfg_attr(debug_assertions, allow(dead_code))]

extern crate core;

use crate::data::err::ErrorCollector;
use crate::data::md_raw::RawModuleInfo;
use crate::data::path::PathBuf;
use crate::data::Module;
use std::collections::HashMap;
use typed_arena::Arena;
use w_parse::Ident;

mod data;
mod elided;
mod pass1_tsys;

pub fn build_tsys<'a>(
    root: Ident<'a>,
    modules: &HashMap<PathBuf<'a>, RawModuleInfo<'a>>,
) -> Result<(), ErrorCollector<'a>> {
    let collector = ErrorCollector::default();

    let types_arena = Arena::new();
    let modules_arena = Arena::new();

    let root_module = Module::new_root(&modules_arena, &types_arena);

    let target = modules.get(&PathBuf::from(vec![root])).unwrap();

    if collector.has_errors() {
        return Err(collector);
    }

    todo!()
}
