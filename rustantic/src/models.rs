use syn::{Fields, Type};

#[derive(Clone)]
pub struct ConstructorMetadata {
    pub args: Vec<(String, Type)>,
}

#[derive(Clone)]
pub struct PydanticMetadata {
    pub ident: String,
    pub fields: Fields,
    pub constructor: Option<ConstructorMetadata>,
}

impl PydanticMetadata {
    pub fn set_ctor(&mut self, ctor: ConstructorMetadata) {
        self.constructor = Some(ctor);
    }
}

unsafe impl Sync for PydanticMetadata {}
