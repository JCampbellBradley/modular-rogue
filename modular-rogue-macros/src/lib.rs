use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use proc_macro_crate::{FoundCrate, crate_name};
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Path, Token, parse_macro_input, punctuated::Punctuated};

fn crate_path() -> proc_macro2::TokenStream {
    match crate_name("modular-rogue") {
        Ok(FoundCrate::Name(name)) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!(#ident)
        }
        Ok(FoundCrate::Itself) => quote!(crate),
        Err(e) => panic!("modular-rogue must be a dependency of the crate using #[derive(Component)]: {e}"),
    }
}

#[proc_macro_derive(DynamicHandlers, attributes(handles))]
pub fn get_handlers_macro(_item: TokenStream) -> TokenStream {
    let modular_rogue_path = crate_path();

    let input = parse_macro_input!(_item as DeriveInput);

    let struct_identifier = input.ident;

    match &input.data {
        Data::Struct(DataStruct { ..}) => { 
            let events: Vec<Path> = input.attrs.iter()
                .find(|a| a.path().is_ident("handles"))
                .map(|a| a.parse_args_with(Punctuated::<Path, Token![,]>::parse_terminated).unwrap())
                .into_iter()
                .flatten()
                .collect();

            let entries = events.iter().map(|ev| quote! {
                (::std::any::TypeId::of::<#ev>(), #modular_rogue_path::component::component::dispatch::<#struct_identifier, #ev>)
            });

            quote! {
                #[automatically_derived]
                impl #modular_rogue_path::component::dynamic_handlers::DynamicHandlers for #struct_identifier {
                    fn get_handlers(&self) -> Vec<(::std::any::TypeId, fn(&mut dyn #modular_rogue_path::component::component::Component, &mut dyn ::std::any::Any))> {
                        vec![#(#entries),*]
                    }
                }
            }
         },
        _ => unimplemented!()
    }.into()
}