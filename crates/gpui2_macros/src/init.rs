// Input:
//
// fn foo(cx: &mut AppContext) {
//   ...
// }
//
// Output:
//
// #[distributed_slice(gpui::__GPUI_INIT)]
// fn foo(cx: &mut AppContext) {
//   ...
// }
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

pub fn init(_args: TokenStream, function: TokenStream) -> TokenStream {
    let initializer = parse_macro_input!(function as ItemFn);

    let expanded = quote! {
        #[gpui::linkme::distributed_slice(gpui::__GPUI_INIT)]
        #[linkme(crate = gpui::linkme)]
        #initializer
    };

    TokenStream::from(expanded)
}
