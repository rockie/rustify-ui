//! Typed site registrations using the shared native declaration parser.

use proc_macro::TokenStream;
use rustify_docs_syntax::SiteDeclaration;

mod expand;

/// Declares a site's components, demos, pages and layout slots.
#[proc_macro]
pub fn site(input: TokenStream) -> TokenStream {
    let declaration = syn::parse_macro_input!(input as SiteDeclaration);
    match expand::expand(declaration) {
        Ok(expansion) => expansion.into(),
        Err(error) => error.to_compile_error().into(),
    }
}
