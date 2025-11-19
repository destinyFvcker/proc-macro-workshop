use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let _intput = parse_macro_input!(input as DeriveInput);
    proc_macro::TokenStream::new()
}
