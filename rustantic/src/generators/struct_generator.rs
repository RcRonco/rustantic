use std::collections::HashSet;

use super::field_generator::FieldGenerator;
use super::generator_base::GeneratorConfig;
use super::generator_base::PydanticCodeGenerator;
use super::generator_base::PydanticCodeGeneratorFactory;
use crate::collector::MetadataCollector;
use crate::models::ConstructorMetadata;
use crate::models::ItemMetadata;
use crate::models::StructMetadata;
use itertools::sorted;

pub(crate) struct StructCodeGenerator {}

impl PydanticCodeGeneratorFactory for StructCodeGenerator {
    fn create() -> Box<dyn PydanticCodeGenerator> {
        Box::new(Self {})
    }
}

impl PydanticCodeGenerator for StructCodeGenerator {
    fn is_item_supported(&self, meta: &ItemMetadata) -> bool {
        matches!(meta, ItemMetadata::Struct(_))
    }

    fn generate(
        &self,
        config: GeneratorConfig,
        collector: &MetadataCollector,
        meta: &ItemMetadata,
    ) -> Result<String, ()> {
        if let ItemMetadata::Struct(struct_md) = meta {
            Ok(self.generate_code(config, collector, struct_md))
        } else {
            Err(())
        }
    }
}

impl StructCodeGenerator {
    fn generate_code(
        &self,
        config: GeneratorConfig,
        collector: &MetadataCollector,
        meta: &StructMetadata,
    ) -> String {
        let mut import_code = format!(
            "from pydantic import BaseModel\nimport {}\n",
            config.package_name
        );
        let mut code = format!("class {}(BaseModel):\n", &meta.ident);
        let mut additional_imports: HashSet<String> = HashSet::new();
        let field_generator = FieldGenerator::new(config.clone(), collector.entities());

        match meta.constructor {
            Some(ref ctor) => {
                for (arg_name, arg_ty) in ctor.args.iter() {
                    let field_result = field_generator.generate(&arg_name, arg_ty);
                    additional_imports.extend(field_result.additional_imports);
                    code.push_str(&format!("    {}\n", field_result.code));
                }
                code.push_str("\n");
                code.push_str(&self.generate_to_pyo3(
                    &field_generator,
                    config.package_name,
                    &meta.ident,
                    ctor,
                    collector,
                ));
            }
            None => {
                code.push_str(
                    &self.generate_invalid_model_body(&meta.ident, "No pyo3 constructor"),
                );
            }
        };

        sorted(additional_imports).for_each(|s| import_code.push_str(&format!("{}\n", s)));

        format!("{}\n{}\n{}", config.header_comment, import_code, code)
    }

    fn generate_invalid_model_body(&self, ident: &str, error: &str) -> String {
        println!(
            "cargo::warning=Rustantic '{}' generation failed, error: {}",
            ident, error
        );
        let mut code = format!("    # Invalid model definition: {}\n", error);
        code.push_str("    pass\n");
        code
    }

    fn generate_to_pyo3(
        &self,
        field_generator: &FieldGenerator,
        package_name: &str,
        struct_name: &str,
        ctor: &ConstructorMetadata,
        collector: &MetadataCollector,
    ) -> String {
        let indent = "    ";
        let mut code = format!("{0}def to_rs(self):\n", indent);
        code.push_str(&format!(
            "{0}{0}return {1}.{2}(\n",
            indent, package_name, struct_name
        ));
        for (arg_name, arg_ty) in ctor.args.iter() {
            if let Some(ident_str) = field_generator.get_type_ident(arg_ty) {
                if collector.contains_ident(&ident_str) {
                    code.push_str(&format!(
                        "{0}{0}{0}{1}=self.{1}.to_rs(),\n",
                        indent, arg_name
                    ));
                } else {
                    code.push_str(&format!("{0}{0}{0}{1}=self.{1},\n", indent, arg_name));
                }
            } else {
                code.push_str(&format!("{0}{0}{0}{1}=self.{1},\n", indent, arg_name));
            }
        }
        code.push_str(&format!("{0}{0})", indent));

        code
    }
}
