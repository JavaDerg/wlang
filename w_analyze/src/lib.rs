#![cfg_attr(debug_assertions, allow(dead_code))]

extern crate core;

use crate::data::err::ErrorCollector;
use crate::data::path::PathBuf;
use crate::data::Module;
use std::collections::HashMap;
use typed_arena::Arena;
use w_parse::{Ident, ParsedModule};

mod data;
mod pass1;

pub fn analyze<'a>(
    _root: Ident<'a>,
    modules: &HashMap<PathBuf<'a>, ParsedModule<'a>>,
) -> Result<(), ErrorCollector<'a>> {
    let collector = ErrorCollector::default();

    let types_arena = Arena::new();
    let modules_arena = Arena::new();
    // FIXME: Module owner needs to be set properly
    let root_module = Module::new(PathBuf::default(), None, &modules_arena, &types_arena);

    for (buf, md) in modules {
        let target_md = root_module.access_or_create_module(buf);
        pass1::run_pass1(md, target_md, &collector);
    }
    if collector.has_errors() {
        return Err(collector);
    }

    todo!()
}
