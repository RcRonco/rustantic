#[allow(unused_imports)]
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn pydantic(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    // Combine the original struct with the metadata registration.
    let expanded = quote! {
        #[pyclass]
        #input
    };
    expanded.into()
}
