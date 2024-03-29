pub mod ctor;
pub mod types;

use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;

use bevy_ecs::world::World;

use std::collections::HashMap;

use w_analyze::data::err::{DuplicateImport, ErrorCollector};
use w_analyze::data::md_raw::RawModuleInfo;
use w_analyze::data::path::PathBuf;

use w_parse::expr::path::ExprPath;
use w_parse::item::import::{Imports, ItemImports};
use w_parse::item::named::{ItemNamed, ItemNamedType, NamedKind};
use w_parse::item::Item;
use w_parse::types::r#struct::TyStruct;
use w_parse::types::ItemTy;
use w_parse::Ident;

pub struct VmState {
    types: HashMap<PathBuf, Entity>,

    root: Ident,
    missing: Vec<PathBuf>,

    errs: ErrorCollector,

    world: World,
}

impl VmState {
    pub fn run(&mut self, module: RawModuleInfo) -> Option<PathBuf> {
        let mut scope = Scope {
            current: module.origin.join(module.parsed.name),
            imports: HashMap::new(),
        };

        for item in &module.parsed.items {
            if let Item::Import(ItemImports { imports, from }) = item {
                for imp in imports {
                    flatten_imports(
                        &mut scope.imports,
                        module.origin.clone(),
                        Some(from),
                        imp,
                        &self.errs,
                    );
                }
            }
        }

        for item in module.parsed.items {
            match item {
                Item::Definer(ItemNamed {
                    name,
                    kind: NamedKind::Type(named_ty),
                }) => {
                    let mut entity = self.world.spawn();

                    let entity_path = scope.current.join(name);
                    entity.insert(WithPath(entity_path.clone()));

                    self.types.insert(entity_path, entity.id());

                    let entity = entity.id();

                    self.analyze_type(entity, named_ty, &scope);
                }
                _ => todo!("functions lol"),
            }
        }
        todo!()
    }

    fn analyze_type(
        &mut self,
        ety: Entity,
        ItemNamedType { ty, .. }: ItemNamedType,
        scope: &Scope,
    ) {
        match ty {
            ItemTy::Referred(other) => {
                let other = self.resolve_path(other, scope);

                let path = self.world.get::<WithPath>(ety).unwrap().0.to_string();

                self.world
                    .get_entity_mut(ety)
                    .unwrap()
                    .insert(IncompleteType::Product {
                        fields: vec![other],
                        ids: None,
                        meta: Some(path),
                    });
            }
            ItemTy::Struct(TyStruct { fields, .. }) => {
                let _names = fields
                    .iter()
                    .map(|pair| pair.name.clone())
                    .collect::<Vec<_>>();
            }
            ItemTy::Enum(_) => todo!(),
            ItemTy::Tuple(_) => {}
            ItemTy::Func(_) => {}
            ItemTy::Array(_) => {}
            ItemTy::Pointer(_) => {}
            ItemTy::Never(_) => {}
        }
    }

    fn resolve_path(&mut self, path: ExprPath, scope: &Scope) -> PathBuf {
        let buf = PathBuf::from(path.path);
        let buf = if path.root.is_some() {
            buf
        } else if let Some(import) = scope.imports.get(buf.first().unwrap()) {
            import.join_path(&buf)
        } else {
            scope.current.join_path(&buf)
        };
        buf
    }
}

struct Scope {
    current: PathBuf,
    imports: HashMap<Ident, PathBuf>,
}

#[derive(Component)]
struct WithPath(pub PathBuf);

#[derive(Component)]
pub struct ModuleScope {
    pub location: PathBuf,
    pub imports: Vec<Entity>,
}

#[derive(Component)]
pub struct Dependencies {
    pub dependencies: Vec<PathBuf>,
}

#[derive(Component)]
pub enum IncompleteType {
    Product {
        fields: Vec<PathBuf>,
        ids: Option<Vec<String>>,
        meta: Option<String>,
    },
    Sum {
        variants: Vec<PathBuf>,
        ids: Vec<String>,
    },
}

mod marker {
    use bevy_ecs::component::Component;

    #[derive(Component)]
    pub struct Incomplete;
}

fn flatten_imports(
    out: &mut HashMap<Ident, PathBuf>,
    rel_root: PathBuf,
    base: Option<&ExprPath>,
    imp: &Imports,
    errs: &ErrorCollector,
) {
    let base = base
        .map(|base| {
            if base.root.is_some() {
                PathBuf::from(base.path.clone())
            } else {
                rel_root.join_path(&PathBuf::from(base.path.clone()))
            }
        })
        .unwrap_or_else(|| rel_root.to_owned());

    match imp {
        Imports::Single(pt) => {
            let tp = PathBuf::from(pt.path.clone());
            let imp = base.join_path(&tp);
            if let Some(og) = out.get(imp.last().unwrap()) {
                errs.add_error(DuplicateImport {
                    original: og.last().unwrap().0.clone(),
                    new: imp.last().unwrap().0.clone(),
                });
                return;
            }
            out.insert(imp.last().unwrap().clone(), imp);
        }
        Imports::Multiple(sub_base, other) => {
            let base = base.join_path(&PathBuf::from(sub_base.path.clone()));
            for imp in other {
                flatten_imports(out, base.clone(), None, imp, errs);
            }
        }
    }
}
