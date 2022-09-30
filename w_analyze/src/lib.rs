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

pub mod data;
pub mod elided;
pub mod pass1_tsys;
pub mod vmod;

pub struct AnalyzerOptions {
    dependencies: Vec<Ident>,
}

pub fn build_tsys(
    vmd: &mut dyn ModuleProvider,
    opt: AnalyzerOptions,
) -> Result<(), ErrorCollector> {
    let collector = ErrorCollector::default();

    let types_arena = Arena::new();
    let modules_arena = Arena::new();

    let root_module = Module::new_root(&modules_arena, &types_arena);

    for dep in &opt.dependencies {
        // vmd.
    }

    if collector.has_errors() {
        return Err(collector);
    }

    todo!()
}
