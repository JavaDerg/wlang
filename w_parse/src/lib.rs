mod types;

use std::collections::HashMap;
use w_tokenize::Span;

pub struct Identifier<'a>(pub Span<'a>);

pub struct ModuleInfo<'a> {
    functions: HashMap<String, FunctionInfo<'a>>,
}
