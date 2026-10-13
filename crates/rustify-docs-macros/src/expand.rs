use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use rustify_docs_syntax::{SiteDeclaration, Slot};
use serde_json::Value;
use syn::{Error, LitStr, Result};

pub(crate) fn expand(declaration: SiteDeclaration) -> Result<TokenStream> {
    let manifest = serde_json::to_value(declaration.manifest())
        .map_err(|error| Error::new(Span::call_site(), error.to_string()))?;
    let manifest = value_tokens(&manifest)?;
    let mut declarations = Vec::new();
    let mut registrations = Vec::new();
    for (component_index, component) in declaration.components.into_iter().enumerate() {
        let props_name = format_ident!("__RustifyDocsProps{component_index}");
        let component_name = LitStr::new(
            component.name.to_string().trim_start_matches("r#"),
            component.name.span(),
        );
        let mut fields = Vec::new();
        for (field_index, field) in component.fields.into_iter().enumerate() {
            let name = field.name;
            let ty = field.ty;
            let default = if let Some(expression) = field.default {
                let function =
                    format_ident!("__rustify_docs_default_{component_index}_{field_index}");
                let path = LitStr::new(&function.to_string(), name.span());
                declarations.push(quote! { fn #function() -> #ty { #expression } });
                quote! { #[serde(default = #path)] }
            } else {
                TokenStream::new()
            };
            fields.push(quote! { #default pub #name: #ty });
        }
        declarations.push(quote! {
            #[derive(::rustify_docs::__private::serde::Serialize, ::rustify_docs::__private::serde::Deserialize)]
            #[serde(crate = "::rustify_docs::__private::serde", deny_unknown_fields)]
            struct #props_name { #(#fields,)* }
        });
        let adapter = component.adapter;
        registrations.push(quote! {
            let __rustify_docs_registry = __rustify_docs_registry.component::<#props_name, _, _>(#component_name, #adapter);
        });
    }
    for demo in declaration.demos {
        let name = LitStr::new(
            demo.name.to_string().trim_start_matches("r#"),
            demo.name.span(),
        );
        let function = demo.function;
        registrations.push(quote! {
            let __rustify_docs_registry = __rustify_docs_registry.demo(#name, #function);
        });
    }
    for page in declaration.pages {
        let name = LitStr::new(
            page.name.to_string().trim_start_matches("r#"),
            page.name.span(),
        );
        let function = page.function;
        registrations.push(quote! {
            let __rustify_docs_registry = __rustify_docs_registry.page(#name, #function);
        });
    }
    for slot in declaration.slots {
        let variant = match slot.slot {
            Slot::Header => quote!(Header),
            Slot::Footer => quote!(Footer),
            Slot::SidebarTop => quote!(SidebarTop),
            Slot::SidebarBottom => quote!(SidebarBottom),
            Slot::BeforeContent => quote!(BeforeContent),
            Slot::AfterContent => quote!(AfterContent),
            Slot::NotFound => quote!(NotFound),
        };
        let function = slot.function;
        registrations.push(quote! {
            let __rustify_docs_registry = __rustify_docs_registry.slot(::rustify_docs::Slot::#variant, #function);
        });
    }
    Ok(quote! {{
        #(#declarations)*
        let __rustify_docs_registry = ::rustify_docs::SiteRegistry::new(#manifest);
        #(#registrations)*
        __rustify_docs_registry
    }})
}

fn value_tokens(value: &Value) -> Result<TokenStream> {
    let path = quote!(::rustify_docs::__private::serde_json::Value);
    Ok(match value {
        Value::Null => quote!(#path::Null),
        Value::Bool(value) => quote!(#path::Bool(#value)),
        Value::String(value) => {
            let literal = LitStr::new(value, Span::call_site());
            quote!(#path::String(#literal.to_string()))
        }
        Value::Array(values) => {
            let values = values
                .iter()
                .map(value_tokens)
                .collect::<Result<Vec<_>>>()?;
            quote!(#path::Array(vec![#(#values),*]))
        }
        Value::Object(values) => {
            let values = values
                .iter()
                .map(|(key, value)| {
                    let key = LitStr::new(key, Span::call_site());
                    let value = value_tokens(value)?;
                    Ok(quote!((#key.to_string(), #value)))
                })
                .collect::<Result<Vec<_>>>()?;
            quote!(#path::Object(::rustify_docs::__private::serde_json::Map::from_iter([#(#values),*])))
        }
        Value::Number(value) => {
            let expression: syn::Expr = syn::parse_str(&value.to_string())?;
            quote!(::rustify_docs::__private::serde_json::json!(#expression))
        }
    })
}
