use syn::Attribute;

pub fn has_rustantic_attr(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident("pydantic"))
}
