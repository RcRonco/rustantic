use crate::{collector::MetadataCollector, models::ItemMetadata};
use std::collections::HashSet;

#[derive(Default)]
pub(crate) struct GenerationResult {
    pub code: String,
    pub additional_imports: HashSet<String>,
}

impl GenerationResult {
    pub(crate) fn add_any_import(&mut self) {
        self.additional_imports
            .insert("from typing import Any".to_string());
    }

    pub(crate) fn add_optional_import(&mut self) {
        self.additional_imports
            .insert("from typing import Optional".to_string());
    }

    pub(crate) fn add_pydantic_field_import(&mut self) {
        self.additional_imports
            .insert("from pydantic import Field".to_string());
    }
}

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
