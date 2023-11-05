use proc_macro::TokenStream;

use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data};

pub fn derive_task(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let enum_name = input.ident;

    match input.data {
        Data::Enum(data) => {
            let tokens = data
                .variants
                .iter()
                .map(|v| {
                    let v_ident = v.ident.clone();
                    let v_field = Ident::new(
                        &v_ident.to_token_stream().to_string().to_lowercase(),
                        Span::call_site(),
                    );
                    let v_return = v_field.clone();
                    quote! {
                        #enum_name::#v_ident(#v_field) => #v_return
                    }
                })
                .collect::<Vec<_>>();

            let expended = quote! {
                impl AsRef<dyn Executable> for #enum_name {
                    fn as_ref(&self) -> &(dyn Executable + 'static) {
                        match self {
                            #(#tokens),*
                        }
                    }
                }

                impl AsMut<dyn Executable> for #enum_name {
                    fn as_mut(&mut self) -> &mut (dyn Executable + 'static) {
                        match self {
                            #(#tokens),*
                        }
                    }
                }
            };

            expended.into()
        }
        _ => panic!("Struct/Union is not supported"),
    }
}
