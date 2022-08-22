#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::data::SimpleTypeSystem;
use typed_arena::Arena;
use w_parse::Module;
use crate::data::err::ErrorCollector;

mod data;
mod pass1;

pub fn analyze(module: Module) -> Result<(), ErrorCollector> {
    let collector = ErrorCollector::default();

    let types_arena = Arena::new();
    let types = SimpleTypeSystem::new(&types_arena);

    pass1::run_pass1(&module, &types, &collector);
    if collector.has_errors() {
        return Err(collector);
    }

    todo!()
}
