use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data};

pub fn derive_state(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let struct_name = input.ident;

    match input.data {
        Data::Struct(ds) => {
            let state = ds.fields.iter().find(|field| {
                field
                    .ident
                    .as_ref()
                    .map(|i| i.into_token_stream().to_string())
                    == Some("state".to_string())
            });
            if state.is_none() {
                panic!("state field is not found");
            }
            let outputs = ds.fields.iter().find(|field| {
                field
                    .ident
                    .as_ref()
                    .map(|i| i.into_token_stream().to_string())
                    == Some("outputs".to_string())
            });
            if outputs.is_none() {
                panic!("outputs field is not found");
            }
            let errors = ds.fields.iter().find(|field| {
                field
                    .ident
                    .as_ref()
                    .map(|i| i.into_token_stream().to_string())
                    == Some("errors".to_string())
            });
            if errors.is_none() {
                panic!("errors field is not found");
            }
        }
        Data::Enum(_) => panic!("Enum is not supported"),
        Data::Union(_) => panic!("Union is not supported"),
    };

    let expanded = quote! {
        impl crate::state::StateAble for #struct_name {
           fn current_state(&mut self) -> &mut ExecutableState {
                &mut self.state
            }

            fn outputs(&mut self) -> &mut Vec<String> {
                &mut self.outputs
            }

            fn errors(&mut self) -> &mut Vec<String> {
                &mut self.errors
            }
        }
    };

    expanded.into()
}
