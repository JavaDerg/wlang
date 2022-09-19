#![cfg_attr(debug_assertions, allow(dead_code))]

extern crate core;

use crate::data::err::ErrorCollector;
use crate::data::md_raw::RawModuleInfo;
use crate::data::path::PathBuf;
use crate::data::Module;
use crate::vmod::ModuleProvider;
use std::collections::HashMap;
use typed_arena::Arena;
use w_parse::Ident;

mod data;
mod elided;
mod pass1_tsys;
mod vmod;

pub struct AnalyzerOptions<'a> {
    dependencies: Vec<Ident<'a>>,
}

pub fn build_tsys<'a>(
    vmd: &mut dyn ModuleProvider<'a>,
    opt: AnalyzerOptions,
) -> Result<(), ErrorCollector<'a>> {
    let collector = ErrorCollector::default();

    let types_arena = Arena::new();
    let modules_arena = Arena::new();

    let root_module = Module::new_root(&modules_arena, &types_arena);

    for dep in &opt.dependencies {
        vmd.
    }

    if collector.has_errors() {
        return Err(collector);
    }

    todo!()
}
