#[allow(unused_imports)]
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};

#[proc_macro_attribute]
pub fn pydantic(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);
    let output = match input {
        Item::Struct(item_struct) => {
            quote! {
                #[pyclass]
                #item_struct
            }
        }
        Item::Enum(item_enum) => {
            if item_enum.variants.iter().all(|f| f.fields.is_empty()) {
                quote! {
                    #[pyclass]
                    #[pyo3(eq, eq_int)]
                    #[derive(PartialEq)]
                    #item_enum
                }
            } else {
                quote! {
                    #[pyclass]
                    #[derive(PartialEq)]
                    #item_enum
                }
            }
        }
        // For any other item types, produce a compile error.
        _ => {
            return syn::Error::new_spanned(
                input,
                "The #[my_pydantic] attribute can only be used on structs or enums.",
            )
            .to_compile_error()
            .into();
        }
    };

    output.into()
}
