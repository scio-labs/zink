pub fn ownable2step_impl(
    struct_name: &syn::Ident,
    err_name: &syn::Ident,
    err_val: &syn::Expr,
) -> proc_macro::TokenStream {
    quote::quote!(
        impl #struct_name {
            #[ink(message)]
            pub fn get_admin(&self) -> AccountId {
                self.admin
            }

            #[ink(message)]
            pub fn get_pending_admin(&self) -> Option<AccountId> {
                self.pending_admin
            }

            #[ink(message)]
            pub fn transfer_ownership(
                &mut self,
                account: Option<AccountId>,
            ) -> core::result::Result<(), #err_name> {
                self.ensure_admin()?;
                if account == Some([0u8; 32].into()) {
                    panic!("Zero Address not allowed");
                }
                self.pending_admin = account;
                Ok(())
            }

            #[ink(message)]
            pub fn accept_ownership(&mut self) -> core::result::Result<(), #err_name> {
                let caller = self.env().caller();
                if self.pending_admin != Some(caller) {
                    return Err(#err_val);
                }
                self.admin = self.pending_admin.expect("Infallible");
                self.pending_admin = None;
                Ok(())
            }

            fn ensure_admin(&self) -> core::result::Result<(), #err_name> {
                if self.env().caller() != self.admin {
                    return Err(#err_val);
                }
                Ok(())
            }
        }
    )
    .into()
}
