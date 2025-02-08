use std::collections::HashSet;

use crate::collector::MetadataCollector;
use crate::models::ConstructorMetadata;
use crate::models::ItemMetadata;
use crate::models::StructMetadata;
use convert_case::{Case, Casing};
use syn::GenericArgument;
use syn::PathArguments;
use syn::Type;
use syn::TypePath;

use super::generator_base::GenerationResult;
use super::generator_base::GeneratorConfig;
use super::generator_base::PydanticCodeGenerator;
use super::generator_base::PydanticCodeGeneratorFactory;

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
            Ok(self.generate_code(&config, collector, struct_md))
        } else {
            Err(())
        }
    }
}

impl StructCodeGenerator {
    fn generate_code(
        &self,
        config: &GeneratorConfig,
        collector: &MetadataCollector,
        meta: &StructMetadata,
    ) -> String {
        let mut import_code = format!(
            "from pydantic import BaseModel\nimport {}\n",
            config.package_name
        );
        let mut code = format!("class {}(BaseModel):\n", &meta.ident);
        let mut additional_imports: HashSet<String> = HashSet::new();
        match meta.constructor {
            Some(ref ctor) => {
                for (arg_name, arg_ty) in ctor.args.iter() {
                    let field_result =
                        self.generate_pydantic_named_field(config, collector, &arg_name, arg_ty);
                    additional_imports.extend(field_result.additional_imports);
                    code.push_str(&format!("    {}\n", field_result.code));
                }
                code.push_str("\n");
                code.push_str(&self.generate_to_pyo3(
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

        additional_imports
            .iter()
            .for_each(|s| import_code.push_str(&format!("{}\n", s)));

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

    fn generate_pydantic_named_field(
        &self,
        config: &GeneratorConfig,
        collector: &MetadataCollector,
        name: &str,
        ty: &Type,
    ) -> GenerationResult {
        let mut result = GenerationResult::default();
        let ty_def = self.rust_type_to_pydantic(config, collector, ty);
        result.code = format!("{}: {}", name, ty_def.code);
        result.additional_imports = ty_def.additional_imports.clone();
        result
    }

    fn generate_to_pyo3(
        &self,
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
            if let Some(ident_str) = self.get_type_ident(arg_ty) {
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

    fn get_type_ident(&self, ty: &Type) -> Option<String> {
        if let Type::Path(path_ty) = ty {
            if path_ty.qself.is_none() {
                let segment = path_ty.path.segments.last().unwrap();
                Some(segment.ident.to_string())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn rust_type_to_pydantic(
        &self,
        config: &GeneratorConfig,
        collector: &MetadataCollector,
        ty: &Type,
    ) -> GenerationResult {
        let mut result = GenerationResult::default();
        match ty {
            // Handle common types
            Type::Path(type_path) => {
                result = self.path_to_pydantic(config, collector, type_path);
            }
            // Handle references like `&str` if you want
            Type::Reference(ref_type) => {
                // If it's `&str`, treat like `String`
                result.code = if let Type::Path(inner_path) = &*ref_type.elem {
                    let last_seg = inner_path.path.segments.last().unwrap().ident.to_string();
                    if last_seg == "str" {
                        "str".to_string()
                    } else {
                        result.add_any_import();
                        format!("Any  # Reference to {last_seg}")
                    }
                } else {
                    result.add_any_import();
                    format!("Any # Unknown ref type")
                };
            }

            // Catch-all for function pointers, never types, tuples, etc.
            _ => {
                result.add_any_import();
                result.code = format!("Any # Unknown type kind");
            }
        };

        result
    }

    fn path_to_pydantic(
        &self,
        config: &GeneratorConfig,
        collector: &MetadataCollector,
        type_path: &TypePath,
    ) -> GenerationResult {
        let mut result = GenerationResult::default();
        if type_path.qself.is_none() {
            // Get the last segment of the path, e.g. "Vec" for "std::vec::Vec"
            let segment = type_path.path.segments.last().unwrap();
            let ident_str = segment.ident.to_string();

            result.code = match ident_str.as_str() {
                "i8" | "i16" | "i32" | "i64" => "int".to_string(),
                "u8" | "u16" | "u32" | "u64" | "isize" | "usize" => {
                    result.add_pydantic_field_import();
                    "int = Field(..., ge=0)".to_string()
                }
                "f32" | "f64" => "float".to_string(),
                "bool" => "bool".to_string(),
                "String" => "str".to_string(),
                "Option" => {
                    result.add_optional_import();
                    if let PathArguments::AngleBracketed(angle_args) = &segment.arguments {
                        if let Some(GenericArgument::Type(inner_ty)) = angle_args.args.first() {
                            let inner_py = self.rust_type_to_pydantic(config, collector, inner_ty);
                            format!("Optional[{}]", inner_py.code)
                        } else {
                            result.add_any_import();
                            "Optional[Any]".to_string()
                        }
                    } else {
                        result.add_any_import();
                        "Optional[Any]".to_string()
                    }
                }
                "Vec" => {
                    if let PathArguments::AngleBracketed(angle_args) = &segment.arguments {
                        if let Some(GenericArgument::Type(inner_ty)) = angle_args.args.first() {
                            let inner_py = self.rust_type_to_pydantic(config, collector, inner_ty);
                            format!("list[{}]", inner_py.code)
                        } else {
                            result.add_any_import();
                            "list[Any]".to_string()
                        }
                    } else {
                        result.add_any_import();
                        "list[Any]".to_string()
                    }
                }
                // Custom or unknown type
                _ => {
                    // Custom pydantic ref
                    self.custom_type_to_pydantic(&ident_str, config, collector, &mut result);
                    result.code
                }
            };
        } else {
            // Qualified (e.g. <T as Trait>::Type). We'll just fallback:
            result.add_any_import();
            "Any".to_string();
        }

        result
    }

    fn custom_type_to_pydantic(
        &self,
        ident: &str,
        config: &GeneratorConfig,
        collector: &MetadataCollector,
        field_result: &mut GenerationResult,
    ) {
        field_result.code = if collector.contains_ident(ident) {
            field_result.additional_imports.insert(format!(
                "from {0}.{1} import {2}",
                &config.models_package_name,
                &ident.to_case(Case::Snake),
                &ident
            ));
            format!("{}", &ident)
        }
        // Unknown type
        else {
            field_result.add_any_import();
            format!("Any # Unknown type {}", &ident)
        }
    }
}
