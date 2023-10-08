pub struct Args {
    pub ownable2step: Option<(syn::Ident, syn::Expr)>,
    pub upgradable: bool,
}

impl syn::parse::Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut ownable2step = None;
        let mut upgradable = false;

        let name: syn::Ident = input.parse()?;

        if name == "Ownable2Step" {
            let content;
            syn::bracketed!(content in input);

            let err_name: syn::Ident = content.parse()?;
            let _: syn::Token![=] = content.parse()?;
            let err_val: syn::Expr = content.parse()?;

            ownable2step = Some((err_name, err_val));
        } else if name == "Upgradable" {
            upgradable = true;
        } else {
            return Err(syn::Error::new_spanned(name, "unidentified keyword found"));
        }

        Ok(Args {
            ownable2step,
            upgradable,
        })
    }
}
