pub fn upgradability_impl(struct_name: &syn::Ident) -> proc_macro::TokenStream {
    quote::quote!(
        impl #struct_name {
            #[ink(message)]
            pub fn upgrade_contract(&mut self, code_hash: [u8; 32]) {
                self.ensure_admin().expect("Not Authorised");

                ink::env::set_code_hash(&code_hash).unwrap_or_else(|err| {
                    panic!(
                        "Failed to `set_code_hash` to {:?} due to {:?}",
                        code_hash, err
                    )
                });
                ink::env::debug_println!("Switched code hash to {:?}.", code_hash);
            }
        }
    )
    .into()
}
