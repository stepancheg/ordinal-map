mod ordinal;

fn wrapper(
    input: proc_macro::TokenStream,
    f: impl FnOnce(proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream>,
) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    let output = f(input).unwrap_or_else(|e| e.to_compile_error());
    proc_macro::TokenStream::from(output)
}

/// Derive `Ordinal` for structs or enums.
///
/// See `Ordinal` trait for more information.
#[proc_macro_derive(Ordinal)]
pub fn derive_ordinal(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    wrapper(input, ordinal::derive_ordinal)
}

#[proc_macro]
#[doc(hidden)]
pub fn impl_ordinal_for_tuple(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    wrapper(input, ordinal::impl_ordinal_for_tuple)
}
