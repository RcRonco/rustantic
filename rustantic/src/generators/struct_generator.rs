use std::collections::HashSet;

use super::field_generator::FieldGenerator;
use super::generator_base::GenerationResult;
use super::generator_base::GeneratorConfig;
use super::generator_base::PydanticCodeGenerator;
use super::generator_base::PydanticCodeGeneratorFactory;
use crate::collector::MetadataCollector;
use crate::models::ConstructorMetadata;
use crate::models::ItemMetadata;
use crate::models::StructMetadata;
use itertools::sorted;
use itertools::Itertools;

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
        let field_generator = FieldGenerator::new(config.clone(), collector.entities());

        let class_declaration = format!("class {}(BaseModel):", &meta.ident);
        let class_definition = self.generate_definition(&config, &field_generator, meta);
        let import_code =
            self.generate_import(config.package_name, &class_definition.additional_imports);

        format!(
            "{}\n{}\n\n{}\n{}",
            config.header_comment, import_code, class_declaration, class_definition.code
        )
    }

    fn generate_import(&self, package_name: &str, additional_imports: &HashSet<String>) -> String {
        let mut imports: HashSet<String> = vec![
            "from pydantic import BaseModel, Field".to_owned(),
            format!("import {}", package_name),
        ]
        .into_iter()
        .collect();

        imports.extend(additional_imports.iter().map(|i| i.to_owned()));

        sorted(imports).join("\n")
    }

    fn generate_definition(
        &self,
        config: &GeneratorConfig,
        field_generator: &FieldGenerator,
        meta: &StructMetadata,
    ) -> GenerationResult {
        if let Some(ref ctor) = meta.constructor {
            self.generate_ctor_based_definition(config, field_generator, &meta.ident, ctor)
        } else {
            self.generate_fields_based_definition(field_generator, meta)
        }
    }

    fn generate_ctor_based_definition(
        &self,
        config: &GeneratorConfig,
        field_generator: &FieldGenerator,
        ident: &str,
        ctor: &ConstructorMetadata,
    ) -> GenerationResult {
        let mut result = GenerationResult::default();
        for (arg_name, arg_ty) in ctor.args.iter() {
            let field_result = field_generator.generate(&arg_name, arg_ty);
            result
                .additional_imports
                .extend(field_result.additional_imports);
            result
                .code
                .push_str(&format!("    {}\n", field_result.code));
        }
        result.code.push_str("\n");
        result.code.push_str(&self.generate_to_pyo3(
            &field_generator,
            config.package_name,
            &ident,
            ctor,
        ));
        result
    }

    fn generate_fields_based_definition(
        &self,
        field_generator: &FieldGenerator,
        meta: &StructMetadata,
    ) -> GenerationResult {
        let mut result = GenerationResult::default();
        if let syn::Fields::Named(ref fields_named) = meta.fields {
            for field in fields_named.named.iter() {
                let field_ident = field
                    .ident
                    .as_ref()
                    .expect("Identifier cannot be empty for named fields")
                    .to_string();
                let field_result = field_generator.generate(&field_ident, &field.ty);
                result
                    .additional_imports
                    .extend(field_result.additional_imports);
                result
                    .code
                    .push_str(&format!("    {}\n", field_result.code));
            }
        } else {
            result.code = self.generate_invalid_model_body(&meta.ident, "No pyo3 constructor");
        }

        result
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
    ) -> String {
        let indent = "    ";
        let mut code = format!("{0}def to_rs(self):\n", indent);
        code.push_str(&format!(
            "{0}{0}return {1}.{2}(\n",
            indent, package_name, struct_name
        ));
        for (arg_name, arg_ty) in ctor.args.iter() {
            let to_rs_field =
                field_generator.generate_to_pyo3(&format!("self.{}", arg_name), arg_ty);
            code.push_str(&format!(
                "{0}{0}{0}{1}={2},\n",
                indent, arg_name, to_rs_field
            ));
        }
        code.push_str(&format!("{0}{0})", indent));

        code
    }
}
