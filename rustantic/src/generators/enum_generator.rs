use crate::{
    collector::MetadataCollector,
    models::{ItemMetadata, UnitEnumMetadata},
};

use super::generator_base::{GeneratorConfig, PydanticCodeGenerator, PydanticCodeGeneratorFactory};

pub(crate) struct EnumCodeGenerator {}

impl PydanticCodeGeneratorFactory for EnumCodeGenerator {
    fn create() -> Box<dyn PydanticCodeGenerator> {
        Box::new(Self {})
    }
}

impl PydanticCodeGenerator for EnumCodeGenerator {
    fn is_item_supported(&self, meta: &ItemMetadata) -> bool {
        if let ItemMetadata::UnitEnum(_) = meta {
            true
        } else {
            false
        }
    }

    fn generate(
        &self,
        config: GeneratorConfig,
        _: &MetadataCollector,
        meta: &ItemMetadata,
    ) -> Result<String, ()> {
        if let ItemMetadata::UnitEnum(enum_md) = meta {
            Ok(self.generate_code(config, enum_md))
        } else {
            Err(())
        }
    }
}

impl EnumCodeGenerator {
    fn generate_code(&self, config: GeneratorConfig, meta: &UnitEnumMetadata) -> String {
        let import_code = format!("import enum\nimport {}\n", config.package_name);
        let mut code = format!("class {}(enum.Enum):\n", &meta.ident);
        for (var_name, discriminant) in meta.variants.iter() {
            code.push_str(&format!("    {}", &var_name));
            if let Some(disc_val) = discriminant {
                code.push_str(&format!(" = {}\n", disc_val));
            } else {
                code.push_str(" = enum.auto()\n");
            }
        }
        code.push_str("\n");
        code.push_str(&self.generate_to_pyo3(config.package_name, meta));

        format!("{}\n{}\n{}", config.header_comment, import_code, code)
    }

    fn generate_to_pyo3(&self, package_name: &str, meta: &UnitEnumMetadata) -> String {
        let indent = "    ";
        let mut code = format!("{0}def to_rs(self):\n", indent);
        code.push_str(&format!("{0}{0}match self:\n", indent));

        for (ref var_name, _) in meta.variants.iter() {
            code.push_str(&format!(
                "{0}{0}{0}case {1}.{2}:\n{0}{0}{0}{0}return {3}.{1}.{2}\n",
                indent, &meta.ident, var_name, package_name
            ));
        }

        code.push_str(&format!(
            "{0}{0}{0}case _:\n{0}{0}{0}{0}raise ValueError(f\"Unsupported value '{{self}}'\")\n",
            indent
        ));

        code
    }
}
