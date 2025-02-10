use crate::{collector::MetadataCollector, models::ItemMetadata};
use std::collections::HashSet;

#[derive(Default)]
pub(crate) struct GenerationResult {
    pub code: String,
    pub additional_imports: HashSet<String>,
}

#[derive(Clone)]
pub(crate) struct GeneratorConfig<'a> {
    pub header_comment: &'a str,
    pub package_name: &'a str,
    pub models_package_name: &'a str,
}

pub(crate) trait PydanticCodeGenerator {
    fn is_item_supported(&self, meta: &ItemMetadata) -> bool;
    fn generate(
        &self,
        config: GeneratorConfig,
        collector: &MetadataCollector,
        meta: &ItemMetadata,
    ) -> Result<String, ()>;
}

pub(crate) trait PydanticCodeGeneratorFactory {
    fn create() -> Box<dyn PydanticCodeGenerator>;
}
