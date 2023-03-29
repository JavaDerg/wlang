use crate::PathBuf;
use w_parse::ParsedModule;

pub struct RawModuleInfo {
    pub parsed: ParsedModule,
    pub origin: PathBuf,
}
