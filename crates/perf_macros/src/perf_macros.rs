use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, Signature};

#[proc_macro_attribute]
pub fn measure(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Most of this code is copied from reading:
    // https://docs.rs/tracing-attributes/0.1.5/src/tracing_attributes/lib.rs.html#115-212
    let input: ItemFn = syn::parse_macro_input!(item as ItemFn);

    let ItemFn {
        attrs,
        vis,
        block,
        sig,
        ..
    } = input;

    let Signature {
        output: return_type,
        inputs: params,
        unsafety,
        asyncness,
        constness,
        abi,
        ident,
        generics:
            syn::Generics {
                params: gen_params,
                where_clause,
                ..
            },
        ..
    } = sig;

    // function name
    let ident_str = ident.to_string();

    // TODO check async support?
    quote!(
        #(#attrs) *
        #vis #constness #unsafety #asyncness #abi fn #ident<#gen_params>(#params) #return_type
            #where_clause
            {
                let __span_attr_guard = perf::measure_lifetime(stringify!(#ident_str));
                #block
            }
    )
    .into()
}
