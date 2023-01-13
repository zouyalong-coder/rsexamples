use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// derive 宏。
/// ```rust
/// // Trait 是 derive(Trait) 的名字，内部生成的模版中，并不会解决依赖。
/// #[proc_macro_derive(Trait)]
/// // 外部用法
/// #[derive(Trait)]
/// struct A {}
/// ```
///
#[proc_macro_derive(Trait)]
pub fn derive_anyname(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote! {
        impl Print for #name {
            fn print(&self) -> usize {
                println!("{}","hello from #name");
                0
           }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
