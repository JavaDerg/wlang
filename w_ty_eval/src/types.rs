use bevy_ecs::component::Component;

#[derive(Component)]
pub enum TypeKind {
    Product { fields: Vec<(String, TypeKind)> },
    Sum { variants: Vec<(String, TypeKind)> },
    Primitive(Primitive),
}

#[derive(Eq, PartialEq, Clone)]
pub enum Primitive {
    Boolean,
    Int8(bool),
    Int16(bool),
    Int32(bool),
    Int64(bool),
    IntSize(bool),
}

impl PartialEq for TypeKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TypeKind::Product { fields: sf, .. }, TypeKind::Product { fields: of, .. }) => {
                sf == of
            }
            (TypeKind::Sum { variants: sv, .. }, TypeKind::Sum { variants: ov, .. }) => sv == ov,
            (TypeKind::Primitive(p1), TypeKind::Primitive(p2)) => p1 == p2,
            (_, _) => false,
        }
    }
}

impl Eq for TypeKind {}
