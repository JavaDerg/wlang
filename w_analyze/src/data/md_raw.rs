use crate::PathBuf;
use w_parse::{Ident, ParsedModule};

pub struct RawModuleInfo<'a> {
    pub parsed: ParsedModule<'a>,
    pub origin: PathBuf<'a>,
}
