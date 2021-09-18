use proc_macro::TokenStream;
use quote::quote;

/// This is a JSON responder derive macro.
/// It makes it easy to implement a rocket responder by just deriving it on the response struct.
#[proc_macro_derive(JsonResponder)]
pub fn json_responder(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;
    let gen = quote! {
        impl<'r> rocket::response::Responder<'r, 'static> for #name {
            fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
                let res = serde_json::json!({
                    "code": self.code(),
                    "error": self.to_string()
                })
                .to_string();

                rocket::Response::build()
                    .status(self.into())
                    .header(rocket::http::ContentType::JSON)
                    .sized_body(res.len(), std::io::Cursor::new(res))
                    .ok()
            }
        }
    };

    gen.into()
}
