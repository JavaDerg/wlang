#![cfg_attr(debug_assertions, allow(dead_code))]

extern crate core;

use crate::data::err::ErrorCollector;
use crate::data::md_raw::RawModuleInfo;
use crate::data::path::PathBuf;
use crate::data::{Module, Origin, TypeOrigin};
use std::collections::HashMap;
use typed_arena::Arena;
use w_parse::{Ident, ParsedModule};

mod data;
mod pass1;

pub fn build_tsys<'a>(
    root: Ident<'a>,
    modules: &HashMap<PathBuf<'a>, RawModuleInfo<'a>>,
) -> Result<(), ErrorCollector<'a>> {
    let collector = ErrorCollector::default();

    let types_arena = Arena::new();
    let modules_arena = Arena::new();

    let root_module = Module::new_root(&modules_arena, &types_arena);

    let target = modules.get(&PathBuf::from(vec![root])).unwrap();
    build_tsys_recursive(target, &modules, &root_module, &collector, vec![]);

    if collector.has_errors() {
        return Err(collector);
    }

    todo!()
}

fn build_tsys_recursive<'a, 'gc>(
    target: &RawModuleInfo<'a>,
    modules: &HashMap<PathBuf<'a>, RawModuleInfo<'a>>,
    root: &'gc Module<'a, 'gc>,
    errs: &ErrorCollector<'a>,
    recursion_fix: Vec<Ident<'a>>,
) {
}
