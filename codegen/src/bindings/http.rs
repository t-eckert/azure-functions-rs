use proc_macro::Diagnostic;
use quote::ToTokens;
use std::convert::TryFrom;
use syn::spanned::Spanned;
use syn::{Ident, Lit};
use util::AttributeArguments;

#[derive(Debug)]
pub struct Http {
    pub name: String,
}

impl<'a> TryFrom<&'a AttributeArguments> for Http {
    type Error = Diagnostic;

    fn try_from(args: &'a AttributeArguments) -> Result<Self, Self::Error> {
        let mut name = None;

        for (key, value) in args.0.iter() {
            let key_str = key.to_string();

            match key_str.as_str() {
                "name" => match value {
                    Lit::Str(s) => {
                        let v = s.value();
                        name = match v.as_ref() {
                            "$return" => Some(v),
                            _ => s
                                .parse::<Ident>()
                                .map(|x| Some(x.to_string()))
                                .map_err(|_| {
                                    value.span().unstable().error(
                                            "a legal parameter identifier is required for the 'name' argument",
                                        )
                                })?,
                        };
                    }
                    _ => {
                        return Err(value
                            .span()
                            .unstable()
                            .error("expected a literal string value for the 'name' argument"));
                    }
                },
                _ => {
                    return Err(key
                        .span()
                        .unstable()
                        .error(format!("unsupported attribute argument '{}'", key_str)));
                }
            };
        }

        Ok(Http {
            name: name.expect("expected a name for the Http binding"),
        })
    }
}

impl ToTokens for Http {
    fn to_tokens(&self, tokens: &mut ::proc_macro2::TokenStream) {
        let name = &self.name;
        quote!(::azure_functions::codegen::bindings::Http { name: #name }).to_tokens(tokens)
    }
}