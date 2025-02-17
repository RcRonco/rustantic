use syn::{Fields, FieldsNamed, Type};

pub enum ItemKind {
    Request,
    Response,
}

#[derive(Clone)]
pub struct ConstructorMetadata {
    pub args: Vec<(String, Type)>,
}

#[derive(Clone)]
pub struct StructMetadata {
    pub ident: String,
    pub constructor: Option<ConstructorMetadata>,
    pub fields: Fields,
}

impl StructMetadata {
    pub fn set_ctor(&mut self, ctor: ConstructorMetadata) {
        self.constructor = Some(ctor);
    }
}

pub struct UnitEnumMetadata {
    pub ident: String,
    pub variants: Vec<(String, Option<String>)>,
}

pub struct UnionVariantMetadata {
    pub ident: String,
    pub ty: Option<Type>,
    #[allow(unused)]
    pub named_fields: Option<FieldsNamed>,
}

pub struct DiscriminatedUnionMetadata {
    pub ident: String,
    pub variants: Vec<UnionVariantMetadata>,
}

pub enum ItemMetadata {
    Struct(StructMetadata),
    UnitEnum(UnitEnumMetadata),
    DiscriminatedUnion(DiscriminatedUnionMetadata),
}

impl ItemMetadata {
    pub fn ident(&self) -> &str {
        match self {
            ItemMetadata::Struct(struct_md) => &struct_md.ident,
            ItemMetadata::UnitEnum(enum_md) => &enum_md.ident,
            ItemMetadata::DiscriminatedUnion(union_md) => &union_md.ident,
        }
    }
}
