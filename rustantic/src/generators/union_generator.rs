use crate::{collector::MetadataCollector, models::ItemMetadata};

use super::generator_base::{GeneratorConfig, PydanticCodeGenerator, PydanticCodeGeneratorFactory};

pub(crate) struct UnionCodeGenerator {}

impl PydanticCodeGeneratorFactory for UnionCodeGenerator {
    fn create() -> Box<dyn PydanticCodeGenerator> {
        Box::new(Self {})
    }
}

impl PydanticCodeGenerator for UnionCodeGenerator {
    fn is_item_supported(&self, meta: &ItemMetadata) -> bool {
        if let ItemMetadata::DiscriminatedUnion(_) = meta {
            true
        } else {
            false
        }
    }

    fn generate(
        &self,
        config: GeneratorConfig,
        collector: &MetadataCollector,
        meta: &ItemMetadata,
    ) -> Result<String, ()> {
        if let ItemMetadata::DiscriminatedUnion(enum_md) = meta {
            Ok("# Discrimintated union".to_string())
        } else {
            Err(())
        }
    }
}
