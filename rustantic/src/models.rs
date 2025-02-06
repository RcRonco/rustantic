use syn::Type;

#[derive(Clone)]
pub struct ConstructorMetadata {
    pub args: Vec<(String, Type)>,
}

#[derive(Clone)]
pub(crate) struct StructMetadata {
    pub ident: String,
    pub constructor: Option<ConstructorMetadata>,
}

impl StructMetadata {
    pub fn set_ctor(&mut self, ctor: ConstructorMetadata) {
        self.constructor = Some(ctor);
    }
}

pub(crate) struct UnitEnumMetadata {
    pub ident: String,
    pub variants: Vec<(String, Option<String>)>,
}
