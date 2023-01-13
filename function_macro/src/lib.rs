use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn a_proc_macro(_input: TokenStream) -> TokenStream {
    TokenStream::from(quote!(
        fn anwser() -> i32 {
            5
        }
    ))
}
