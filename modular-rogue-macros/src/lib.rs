use heck::ToSnakeCase;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use proc_macro_crate::{FoundCrate, crate_name};
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, LitStr, Path, Token, parse_macro_input, punctuated::Punctuated};

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

enum HandledEvent {
    Const(Path),
    Mut(Path),
}

impl syn::parse::Parse for HandledEvent {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Token![mut]) {
            input.parse::<Token![mut]>()?;
            Ok(HandledEvent::Mut(input.parse()?))
        } else {
            Ok(HandledEvent::Const(input.parse()?))
        }
    }
}


#[proc_macro_derive(DynamicHandlers, attributes(handles))]
pub fn get_handlers_macro(_item: TokenStream) -> TokenStream {
    let modular_rogue_path = crate_path();

    let input = parse_macro_input!(_item as DeriveInput);

    let struct_identifier = input.ident;

    match &input.data {
        Data::Struct(DataStruct { ..}) => { 
            let events: Vec<HandledEvent> = input.attrs.iter()
                .find(|a| a.path().is_ident("handles"))
                .map(|a| a.parse_args_with(Punctuated::<HandledEvent, Token![,]>::parse_terminated).unwrap())
                .into_iter()
                .flatten()
                .collect();

            let entries = events.iter()
                .filter_map(|ev| match ev { HandledEvent::Const(p) => Some(p), _ => None })
                .map(|ev| quote! {
                    (::std::any::TypeId::of::<#ev>(), #modular_rogue_path::components::component::dispatch::<#struct_identifier, #ev>)
                });

            let entries_mut = events.iter()
                .filter_map(|ev| match ev { HandledEvent::Mut(p) => Some(p), _ => None })
                .map(|ev| quote! {
                    (::std::any::TypeId::of::<#ev>(), #modular_rogue_path::components::component::dispatch_mut::<#struct_identifier, #ev>)
                });

            quote! {
                #[automatically_derived]
                impl #modular_rogue_path::components::dynamic_handlers::DynamicHandlers for #struct_identifier {
                    fn get_handlers(&self) -> Vec<(::std::any::TypeId, fn(&dyn #modular_rogue_path::components::component::Component, &mut dyn ::std::any::Any))> {
                        vec![#(#entries),*]
                    }
                    
                    fn get_handlers_mut(&self) -> Vec<(::std::any::TypeId, fn(&mut dyn #modular_rogue_path::components::component::Component, &mut dyn ::std::any::Any))> {
                        vec![#(#entries_mut),*]
                    }
                }
            }
         },
        _ => unimplemented!()
    }.into()
}

#[proc_macro_derive(ModElement, attributes(component))]
pub fn mod_component_macro(item: TokenStream) -> TokenStream {
    let path = crate_path();
    let input = parse_macro_input!(item as DeriveInput);
    let ident = &input.ident;

    let mut name = ident.to_string().to_snake_case();

    for attr in input.attrs.iter().filter(|a| a.path().is_ident("component")) {
        let result = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("name") {
                name = meta.value()?.parse::<LitStr>()?.value();
                Ok(())
            } else {
                Err(meta.error("expected `name = \"...\"`"))
            }
        });

        if let Err(e) = result {
            return e.to_compile_error().into();
        }
    }

    let mr_name = format!("__MR_{}", ident.to_string().to_uppercase());
    let mr_name_ident = syn::Ident::new(&mr_name, ident.span());

    quote! {
        #[linkme::distributed_slice(crate::MR_COMPONENTS)]
        static #mr_name_ident: #path::glue::load_mods::RegisterFn = |ctx| ctx.register_component::<#ident>();

        #[automatically_derived]
        impl #path::glue::mod_element::ModElement for #ident {
            const NAME: &'static str = #name;
        }
    }.into()
}