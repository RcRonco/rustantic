use crate::models::{ConstructorMetadata, StructMetadata, UnitEnumMetadata};
use std::{collections::HashMap, fs, path::PathBuf};
use syn::{
    parse_file, visit::Visit, Attribute, FnArg, ImplItem, ImplItemFn, Item, ItemEnum, ItemImpl,
    ItemStruct, Type,
};
use walkdir::WalkDir;

pub(crate) struct MetadataCollector {
    lib_location: PathBuf,
    structs: HashMap<String, StructMetadata>,
    unit_enums: HashMap<String, UnitEnumMetadata>,
}

impl MetadataCollector {
    pub fn new(lib_location: &str) -> Self {
        Self {
            lib_location: PathBuf::from(lib_location),
            structs: HashMap::new(),
            unit_enums: HashMap::new(),
        }
    }

    pub fn collect(&mut self) {
        self.scan_lib();
    }

    pub fn structs(&self) -> &HashMap<String, StructMetadata> {
        &self.structs
    }

    pub fn unit_enum(&self) -> &HashMap<String, UnitEnumMetadata> {
        &self.unit_enums
    }

    pub fn contains_ident(&self, ident: &str) -> bool {
        self.structs.contains_key(ident) || self.unit_enums.contains_key(ident)
    }

    fn scan_lib(&mut self) {
        let src_dir = self.lib_location.join("src");
        for entry in WalkDir::new(&src_dir) {
            let entry = entry.expect("Failed to read entry");
            if entry.file_type().is_file()
                && entry
                    .path()
                    .extension()
                    .map(|ext| ext == "rs")
                    .unwrap_or(false)
            {
                let path = entry.path();
                match fs::read_to_string(path) {
                    Ok(code) => match parse_file(&code) {
                        Ok(syntax) => {
                            self.visit_file(&syntax);
                        }
                        Err(err) => {
                            println!("cargo:warning=Failed to parse {}: {}", path.display(), err);
                        }
                    },
                    Err(err) => {
                        println!("cargo:warning=Failed to read {}: {}", path.display(), err);
                    }
                }
            }
        }
    }

    fn has_rustantic_attr(&self, attrs: &[Attribute]) -> bool {
        attrs.iter().any(|attr| attr.path().is_ident("pydantic"))
    }

    fn is_pyo_constructor(&self, fn_: &ImplItemFn) -> bool {
        fn_.attrs.iter().any(|attr| attr.path().is_ident("new"))
    }

    fn collect_pydantic_struct(&mut self, item_struct: &ItemStruct) {
        self.structs.insert(
            item_struct.ident.to_string(),
            StructMetadata {
                ident: item_struct.ident.to_string(),
                constructor: None,
            },
        );
    }

    fn collect_pydantic_fn(&mut self, item_ident: &str, item_fn: &ImplItemFn) {
        let args: Vec<(String, Type)> = item_fn
            .sig
            .inputs
            .iter()
            .filter_map(|arg| match arg {
                FnArg::Typed(type_arg) => {
                    if let syn::Pat::Ident(ref arg_ident) = type_arg.pat.as_ref() {
                        let arg_name = arg_ident.ident.to_string();
                        let arg_type = type_arg.ty.as_ref().clone();
                        Some((arg_name, arg_type))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect();

        if let Some(struct_meta) = self.structs.get_mut(item_ident) {
            struct_meta.set_ctor(ConstructorMetadata { args });
        }
    }

    fn collect_pydantic_enum(&mut self, item_enum: &ItemEnum) {
        let mut variants = Vec::new();
        for variant in item_enum.variants.iter() {
            if let Some((_, ref expr)) = variant.discriminant {
                variants.push((
                    variant.ident.to_string(),
                    Some(quote::quote!(#expr).to_string()),
                ));
            } else {
                variants.push((variant.ident.to_string(), None));
            }
        }
        self.unit_enums.insert(
            item_enum.ident.to_string(),
            UnitEnumMetadata {
                ident: item_enum.ident.to_string(),
                variants,
            },
        );
    }
}

impl<'ast> Visit<'ast> for MetadataCollector {
    fn visit_item(&mut self, node: &'ast Item) {
        match node {
            Item::Struct(ref item_struct) => {
                if self.has_rustantic_attr(&item_struct.attrs) {
                    self.collect_pydantic_struct(item_struct);
                }
            }
            Item::Enum(ref item_enum) => {
                if self.has_rustantic_attr(&item_enum.attrs) {
                    self.collect_pydantic_enum(item_enum);
                }
            }
            _ => {}
        }
        // Continue visiting nested items.
        syn::visit::visit_item(self, node);
    }

    fn visit_item_impl(&mut self, node: &'ast ItemImpl) {
        let type_name = if let syn::Type::Path(type_path) = &*node.self_ty {
            type_path
                .path
                .segments
                .last()
                .map(|seg| seg.ident.to_string())
                .unwrap_or_else(|| "Unknown".to_string())
        } else {
            println!("cargo:error=Rustantic collector didn't find matching struct");
            "Unknown".to_string()
        };

        if self.structs.contains_key(&type_name) {
            for item in &node.items {
                if let ImplItem::Fn(ref item_fn) = item {
                    if self.is_pyo_constructor(item_fn) {
                        self.collect_pydantic_fn(&type_name, item_fn);
                    }
                }
            }
        }

        // Continue the visit so nested impls (if any) get processed.
        syn::visit::visit_item_impl(self, node);
    }
}
