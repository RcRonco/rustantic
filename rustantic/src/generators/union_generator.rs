use std::collections::HashSet;

use crate::{
    collector::MetadataCollector,
    models::{DiscriminatedUnionMetadata, ItemMetadata, UnionVariantMetadata},
};

use super::generator_base::{
    GenerationResult, GeneratorConfig, PydanticCodeGenerator, PydanticCodeGeneratorFactory,
};

pub(crate) struct UnionCodeGenerator {}

impl PydanticCodeGeneratorFactory for UnionCodeGenerator {
    fn create() -> Box<dyn PydanticCodeGenerator> {
        Box::new(Self {})
    }
}

impl PydanticCodeGenerator for UnionCodeGenerator {
    fn is_item_supported(&self, meta: &ItemMetadata) -> bool {
        matches!(meta, ItemMetadata::DiscriminatedUnion(_))
    }

    fn generate(
        &self,
        config: GeneratorConfig,
        collector: &MetadataCollector,
        meta: &ItemMetadata,
    ) -> Result<String, ()> {
        if let ItemMetadata::DiscriminatedUnion(union_md) = meta {
            Ok(self.generate_code(&config, collector, union_md))
        } else {
            Err(())
        }
    }
}

impl UnionCodeGenerator {
    fn generate_code(
        &self,
        config: &GeneratorConfig,
        collector: &MetadataCollector,
        meta: &DiscriminatedUnionMetadata,
    ) -> String {
        let discriminator = self.generate_discriminator(meta);
        let variants = self.generate_union_variants(config, collector, meta);
        let definition = self.generate_type_definitions(meta);
        let imports = self.generate_import(&variants.additional_imports);
        format!(
            "# Missing to_rs function\n{}\n\n{}\n{}\n{}",
            imports, discriminator, variants.code, definition
        )
    }

    fn generate_import(&self, additional_imports: &HashSet<String>) -> String {
        let mut code = "import enum\n\
         from typing import Annotated, Literal, Union\n\
         from pydantic import BaseModel, Discriminator, Field, RootModel"
            .to_string();

        additional_imports
            .iter()
            .for_each(|s| code.push_str(&format!("{}\n", s)));
        code
    }

    fn generate_discriminator(&self, meta: &DiscriminatedUnionMetadata) -> String {
        let mut code = format!(
            "class {}(enum.Enum):\n",
            self.generate_discriminator_name(&meta.ident)
        );
        for variant in meta.variants.iter() {
            code.push_str(&format!("    {} = enum.auto()\n", &variant.ident));
        }
        code.push_str("\n");
        code
    }

    fn generate_union_variants(
        &self,
        config: &GeneratorConfig,
        collector: &MetadataCollector,
        meta: &DiscriminatedUnionMetadata,
    ) -> GenerationResult {
        let mut variants = vec![];
        let mut result = GenerationResult::default();
        for variant in meta.variants.iter() {
            let variant_code = self.generate_union_variant(config, collector, meta, variant);
            result
                .additional_imports
                .extend(variant_code.additional_imports);
            variants.push(variant_code.code);
        }
        result.code = variants.join("\n");
        result
    }

    fn generate_union_variant(
        &self,
        _: &GeneratorConfig,
        _: &MetadataCollector,
        meta: &DiscriminatedUnionMetadata,
        variant: &UnionVariantMetadata,
    ) -> GenerationResult {
        let mut result = GenerationResult::default();
        let code = vec![
            format!("class {0}{1}(BaseModel):", &meta.ident, &variant.ident),
            format!(
                "    kind: Literal[{0}.{1}] = Field(default={0}.{1}, init=False, frozen=True)",
                self.generate_discriminator_name(&meta.ident),
                &variant.ident
            ),
            format!(
                "    value: {0} # Type unresolved!",
                variant
                    .ty_ident
                    .as_ref()
                    .expect("Only unnamed variants supported")
            ),
            "\n".to_string(),
        ];
        result.code = code.join("\n");
        result
    }

    fn generate_type_definitions(&self, meta: &DiscriminatedUnionMetadata) -> String {
        let variants: Vec<String> = meta
            .variants
            .iter()
            .map(|v| format!("{}{}", &meta.ident, &v.ident))
            .collect();

        vec![
            format!("{}Type = Union[{}]", &meta.ident, variants.join(",")),
            format!(
                "{0} = Annotated[{0}Type, Discriminator(\"kind\")]",
                &meta.ident
            ),
            format!("\nclass {0}RootModel(RootModel[{0}Type]):", &meta.ident),
            format!(
                "    root: {}Type = Field(..., discriminator=\"kind\")",
                &meta.ident
            ),
        ]
        .join("\n")
    }

    fn generate_discriminator_name(&self, ident: &str) -> String {
        format!("{}Discriminator", ident)
    }
}
