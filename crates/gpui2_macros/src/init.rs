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
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn};

pub fn init(_args: TokenStream, function: TokenStream) -> TokenStream {
    let initializer = parse_macro_input!(function as ItemFn);
    let fn_name = &initializer.sig.ident;

    let static_slice_name = format_ident!("__GPUI_INIT_{}", fn_name.to_string().to_uppercase());

    let expanded = quote! {
        #initializer

        #[gpui::linkme::distributed_slice(gpui::__GPUI_INIT)]
        #[linkme(crate = gpui::linkme)]
        static #static_slice_name: gpui::MacroInitFn = #fn_name;
    };

    TokenStream::from(expanded)
}
