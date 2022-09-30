use crate::data::path::Path;
use w_parse::{Ident, ParsedModule};

pub trait ModuleProvider {
    fn get(&self, path: &Path) -> Option<ParsedModule>;
    fn submit(&mut self, path: &Path, md: ParsedModule) -> Result<(), ()>;

    fn root(&self) -> Ident;
}
