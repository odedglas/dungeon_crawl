use proc_macro::{TokenStream};
use syn::{parse_macro_input, DeriveInput, Data};
use quote::quote;

extern crate proc_macro;

#[proc_macro_derive(BaseMapArchitect)]
pub fn base_map_builder_derive(input: TokenStream) -> TokenStream {
    let parsed_input: DeriveInput = parse_macro_input!(input);

    impl_base_map_architect_derive(parsed_input)
}

fn impl_base_map_architect_derive(parsed_input: DeriveInput) -> TokenStream {
    let struct_name = parsed_input.ident;

    match parsed_input.data {
        Data::Struct(_s) => {
            let tokens = quote! {
                use super::BaseMapArchitect;

                impl BaseMapArchitect for #struct_name {
                    fn get_map_builder(&self) -> &MapBuilder {
                        &self.map_builder
                    }

                    fn get_mut_map_builder(&mut self) -> &mut MapBuilder {
                        &mut self.map_builder
                    }
                }
            };

            proc_macro::TokenStream::from(tokens)
        }
        other => panic!("Cannot implement BehaviorState for: {:?}", other),
    }
}
