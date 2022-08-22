#![cfg_attr(debug_assertions, allow(dead_code))]

use crate::data::err::ErrorCollector;
use crate::data::Module;
use typed_arena::Arena;
use w_parse::ParsedModule;

mod data;
mod pass1;

pub fn analyze<'a>(module: &[ParsedModule<'a>]) -> Result<(), ErrorCollector<'a>> {
    let collector = ErrorCollector::default();

    let types_arena = Arena::new();
    let modules_arena = Arena::new();
    let module = Module::new(&types_arena);

    pass1::run_pass1(&module, &types, &collector);
    if collector.has_errors() {
        return Err(collector);
    }

    todo!()
}
