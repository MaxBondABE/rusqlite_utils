use quote::quote;
use syn::{Data, Ident};

pub fn impl_try_from_row(ident: Ident, data: Data) -> proc_macro2::TokenStream {
    let field_conversions;
    if let Data::Struct(s) = data {
        field_conversions = match s.fields {
            syn::Fields::Named(f) => f
                .named
                .into_iter()
                .map(|f| {
                    let field_ident = f.ident.expect("fields are named");
                    let column_name_str = field_ident.to_string();
                    quote! {
                        #field_ident: row.get(#column_name_str)?
                    }
                })
                .collect::<Vec<_>>(),

            syn::Fields::Unnamed(_) => {
                unimplemented!("This macro is only implemented for named structs.")
            }
            syn::Fields::Unit => {
                unimplemented!("This macro is only implemented for named structs.")
            }
        };
    } else {
        unimplemented!("This macro is only implemented for named structs.")
    }

    quote! {
        impl<'stmt> TryFrom<&rusqlite::Row<'stmt>> for #ident {
            type Error = rusqlite::Error;
            fn try_from(row: &rusqlite::Row<'stmt>) -> Result<#ident, rusqlite::Error> {
                Ok(Self {
                    #(#field_conversions),*
                })
            }
        }
    }
}
