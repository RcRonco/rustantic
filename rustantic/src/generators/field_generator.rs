use std::collections::{HashMap, HashSet};

use convert_case::{Case, Casing};
use syn::{GenericArgument, PathArguments, Type, TypePath};

use crate::models::ItemMetadata;

use super::generator_base::{GenerationResult, GeneratorConfig};

#[derive(Default)]
struct FieldGenerationResult {
    additional_imports: HashSet<String>,
    ty: String,
    default_value: Option<String>,
    field_properties: HashSet<String>,
    comment: String,
}

impl FieldGenerationResult {
    fn new(ty: String, additional_imports: impl Iterator<Item = String>) -> Self {
        Self {
            additional_imports: additional_imports.collect(),
            ty: ty,
            default_value: None,
            field_properties: HashSet::new(),
            comment: "".to_owned(),
        }
    }

    fn create_any(comment: Option<&str>) -> Self {
        let mut any_field = Self::default();
        any_field.add_any_import();
        any_field.ty = "Any".to_owned();
        if let Some(comment_msg) = comment {
            any_field.comment.push_str(comment_msg);
        }

        any_field
    }

    fn add_any_import(&mut self) {
        self.additional_imports
            .insert("from typing import Any".to_string());
    }

    fn add_optional_import(&mut self) {
        self.additional_imports
            .insert("from typing import Optional".to_string());
    }

    fn add_pydantic_field_import(&mut self) {
        self.additional_imports
            .insert("from pydantic import Field".to_string());
    }

    fn require_field_definition(&self) -> bool {
        self.default_value.is_some() || !self.field_properties.is_empty()
    }
}

pub(crate) struct FieldGenerator<'a> {
    config: GeneratorConfig<'a>,
    entities: &'a HashMap<String, ItemMetadata>,
}

impl<'a> FieldGenerator<'a> {
    pub fn new(config: GeneratorConfig<'a>, entities: &'a HashMap<String, ItemMetadata>) -> Self {
        Self { config, entities }
    }

    pub fn get_type_ident(&self, ty: &Type) -> Option<String> {
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

    pub fn generate(&self, field_ident: &str, ty: &Type) -> GenerationResult {
        let generated_field = self.rust_type_to_pydantic(ty);
        let mut code = format!("{}: {}", field_ident, generated_field.ty);
        if generated_field.require_field_definition() {
            code.push_str(" = Field(");
            if let Some(default_val) = generated_field.default_value.as_ref() {
                code.push_str(&format!("default={}", default_val));
            } else {
                code.push_str("...");
            }

            if !generated_field.field_properties.is_empty() {
                code.push_str(", ");
                let fields: Vec<String> = generated_field.field_properties.into_iter().collect();
                code.push_str(&fields.join(", "));
            }
            code.push(')');
        }

        if generated_field.comment.len() > 0 {
            code.push_str(&format!(" # {}", generated_field.comment));
        }

        GenerationResult {
            code: code,
            additional_imports: generated_field.additional_imports,
        }
    }

    fn rust_type_to_pydantic(&self, ty: &Type) -> FieldGenerationResult {
        match ty {
            // Handle common types
            Type::Path(type_path) => self.path_to_pydantic(type_path),
            // Handle references like `&str` if you want
            Type::Reference(ref_type) => {
                let mut result = FieldGenerationResult::default();
                // If it's `&str`, treat like `String`
                if let Type::Path(inner_path) = &*ref_type.elem {
                    let last_seg = inner_path.path.segments.last().unwrap().ident.to_string();
                    if last_seg == "str" {
                        result.ty = "str".to_string();
                    } else {
                        result.add_any_import();
                        result.comment.push_str(&format!("Reference to {last_seg}"));
                        result.ty = "Any".to_owned();
                    }
                } else {
                    result.add_any_import();
                    result.comment.push_str("Unknown ref type");
                    result.ty = "Any".to_owned();
                };
                result
            }
            // Catch-all for function pointers, never types, tuples, etc.
            _ => {
                let mut result = FieldGenerationResult::default();
                result.add_any_import();
                result.comment.push_str("Unknown ref type");
                result.ty = "Any".to_owned();
                result
            }
        }
    }

    fn path_to_pydantic(&self, type_path: &TypePath) -> FieldGenerationResult {
        if type_path.qself.is_some() {
            FieldGenerationResult::create_any(Some(
                "Qualified (e.g. <T as Trait>::Type). We'll just fallback",
            ))
        } else {
            let mut result = FieldGenerationResult::default();

            // Get the last segment of the path, e.g. "Vec" for "std::vec::Vec"
            let segment = type_path.path.segments.last().unwrap();
            let ident_str = segment.ident.to_string();

            result.ty = match ident_str.as_str() {
                "i8" | "i16" | "i32" | "i64" => "int".to_string(),
                "u8" | "u16" | "u32" | "u64" | "isize" | "usize" => {
                    result.add_pydantic_field_import();
                    result.field_properties.insert("ge=0".to_owned());
                    "int".to_string()
                }
                "f32" | "f64" => "float".to_string(),
                "bool" => "bool".to_string(),
                "String" => "str".to_string(),
                "Option" => {
                    let mut option = self.resolve_inner_type("Optional", &segment.arguments);
                    option.add_optional_import();
                    return option;
                }
                "Vec" => {
                    return self.resolve_inner_type("list", &segment.arguments);
                }
                // Custom or unknown type
                _ => {
                    // Custom pydantic ref
                    return self.custom_type_to_pydantic(&ident_str);
                }
            };

            result
        }
    }

    fn resolve_inner_type(
        &self,
        parent_type: &str,
        inner_type: &PathArguments,
    ) -> FieldGenerationResult {
        if let PathArguments::AngleBracketed(angle_args) = inner_type {
            if let Some(GenericArgument::Type(inner_ty)) = angle_args.args.first() {
                let mut result = self.rust_type_to_pydantic(inner_ty);
                result.ty = format!("{}[{}]", parent_type, result.ty);
                result
            } else {
                let mut any = FieldGenerationResult::create_any(None);
                any.ty = format!("{}[{}]", parent_type, "Any");
                any
            }
        } else {
            let mut any = FieldGenerationResult::create_any(None);
            any.ty = format!("{}[{}]", parent_type, "Any");
            any
        }
    }

    fn custom_type_to_pydantic(&self, ident: &str) -> FieldGenerationResult {
        if self.entities.contains_key(ident) {
            FieldGenerationResult::new(
                format!("{}", &ident),
                [format!(
                    "from {0}.{1} import {2}",
                    self.config.models_package_name,
                    &ident.to_case(Case::Snake),
                    &ident
                )]
                .into_iter(),
            )
        }
        // Unknown type
        else {
            FieldGenerationResult::create_any(Some("Unknown ref type"))
        }
    }
}
