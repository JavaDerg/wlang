use w_parse::{Ident, ParsedModule};

pub struct RawModuleInfo<'a> {
    pub parsed: ParsedModule<'a>,
    pub origin: Ident<'a>,
    pub dependants: Vec<Ident<'a>>,
}
