use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(TonicError)]
pub fn tonic_error_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_tonic_error(&ast)
}

fn impl_tonic_error(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl<'t> TonicError<'t> for #name {}
    };
    gen.into()
}
