extern crate proc_macro;

mod ownable2step;
mod parser;
mod upgradability;

use crate::parser::Args;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, Item, ItemMod, ItemStruct};

fn get_module_items(module: &mut ItemMod) -> &mut Vec<Item> {
    &mut module
        .content
        .as_mut()
        .expect("Expected contract module. Found mod declaration")
        .1
}

fn get_storage_struct(items: &[Item]) -> ItemStruct {
    let is_storage_struct = |item: &ItemStruct| -> bool {
        item.attrs
            .iter()
            .any(|attr| match attr.path().is_ident("ink") {
                false => false,
                true => attr
                    .parse_nested_meta(|meta| match meta.path.is_ident("storage") {
                        true => Ok(()),
                        false => Err(meta.error("not storage struct")),
                    })
                    .is_ok(),
            })
    };

    items
        .iter()
        .find_map(|item| match item {
            syn::Item::Struct(item_struct) if is_storage_struct(item_struct) => Some(item_struct),
            _ => None,
        })
        .expect("Could not find struct marked with #[ink(storage)]")
        .clone()
}

fn support_upgrade(items: &mut Vec<Item>, storage_name: &syn::Ident) {
    let code = upgradability::upgradability_impl(storage_name);
    let item = syn::parse(code).unwrap();
    items.push(item);
}

fn support_ownable2step(
    items: &mut Vec<Item>,
    storage_name: &syn::Ident,
    err_name: &syn::Ident,
    err_val: &syn::Expr,
) {
    let code = ownable2step::ownable2step_impl(storage_name, err_name, err_val);
    let item = syn::parse(code).unwrap();
    items.push(item);
}

#[proc_macro_attribute]
pub fn coating(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as Args);
    let mut module = parse_macro_input!(input as ItemMod);

    let module_items = get_module_items(&mut module);
    let storage_name = get_storage_struct(module_items).ident;

    if let Some(data) = args.ownable2step {
        let (err_name, err_val) = data;
        support_ownable2step(module_items, &storage_name, &err_name, &err_val);
    }
    if args.upgradable {
        support_upgrade(module_items, &storage_name);
    }

    module.into_token_stream().into()
}
