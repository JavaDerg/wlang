use crate::PathBuf;
use w_parse::{Ident, ParsedModule};

pub struct RawModuleInfo {
    pub parsed: ParsedModule,
    pub origin: PathBuf,
}
