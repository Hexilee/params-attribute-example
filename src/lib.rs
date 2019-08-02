extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn rename_params(_args: TokenStream, input: TokenStream) -> TokenStream {
    println!("{}", &input);
    input
}
