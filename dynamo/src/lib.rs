#![deny(clippy::pedantic)]
#![deny(missing_docs)]
#![deny(clippy::missing_panics_doc)]

//! Codegen crate for Hyperion

use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// Creates a route handler with a function
#[proc_macro_attribute]
pub fn route(
    attibutes: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let function = parse_macro_input!(item as ItemFn);
    let function_identifier = function.sig.ident;
    let block = function.block;
    let return_type = match function.sig.output {
        syn::ReturnType::Default => quote!(hyperion::http::response::Response<()>),
        syn::ReturnType::Type(_, typ) => quote!(#typ),
    };

    quote!(
        #[allow(non_camel_case_types)]
        struct #function_identifier;

        impl hyperion::http::router::RequestHandler for #function_identifier {
            fn handle(&self, req: hyperion::http::request::Request) -> Result<hyperion::http::response::Response<Box<dyn hyperion::http::response::Body>>, hyperion::http::error::Error> {
                let response: #return_type = #block;

                Ok(Response::new(
                    Box::new(response.body) as Box<dyn hyperion::http::response::Body>,
                    response.headers,
                    response.status,
                ))
            }
        }
    )
    .into()
}
