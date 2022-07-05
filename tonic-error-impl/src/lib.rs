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

        use std::convert::TryFrom;
        static CUSTOM_ERROR: &str = "x-custom-tonic-error";

        impl TryFrom<tonic::Status> for #name {
            type Error = tonic_error::ParseError;

            fn try_from(s: tonic::Status) -> Result<#name, Self::Error> {
                match s.code() {
                    tonic::Code::Internal => {
                        if let Some(err) = s.metadata().get(CUSTOM_ERROR) {
                            Ok(serde_json::from_str(err.to_str()?)?)
                        } else {
                            Err(tonic_error::ParseError::MissingMetadata)
                        }
                    }
                    c => Err(tonic_error::ParseError::InvalidStatusCode(s)),
                }
            }
        }

        impl From<#name> for tonic::Status {
            fn from(e: #name) -> Self {
                let mut status = tonic::Status::internal(format!("internal error: {}", e));

                status.metadata_mut().insert(CUSTOM_ERROR,
                                             serde_json::to_string(&e)
                                                .unwrap_or("could not serialize: {e}".to_string())
                                                .parse()
                                                .unwrap_or(tonic::metadata::MetadataValue::from_static("unable to create metadata value")));
                status
            }
        }
    };
    gen.into()
}
