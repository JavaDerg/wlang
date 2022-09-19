use crate::data::path::Path;
use w_parse::{Ident, ParsedModule};

pub trait ModuleProvider<'a> {
    fn get(&self, path: &Path) -> Option<ParsedModule<'a>>;
    fn submit(&mut self, path: &Path, md: ParsedModule<'a>) -> Result<(), ()>;

    fn root(&self) -> Ident<'a>;
}
