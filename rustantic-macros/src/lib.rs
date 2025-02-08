#[allow(unused_imports)]
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};

#[proc_macro_attribute]
pub fn pydantic(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);
    let output = match input {
        // If the input is a struct, add #[pyclass] and do any modifications you need.
        Item::Struct(item_struct) => {
            // Optionally check if #[pyclass] is already present and add it if not.
            // For simplicity, we just inject #[pyclass] before the struct.
            quote! {
                #[pyclass]
                #item_struct
            }
        }
        // If the input is an enum, process it accordingly.
        Item::Enum(item_enum) => {
            // For enums, you might not need #[pyclass] (depending on your design),
            // but you could also register it for code-generation purposes.
            // Here we simply return the enum unmodified.
            quote! {
                #[pyclass]
                #[derive(PartialEq)]
                #item_enum
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
