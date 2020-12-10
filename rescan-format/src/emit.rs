use super::parse::Abstract2;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};

pub fn emit(abs: Abstract2) -> TokenStream {
    abs.to_token_stream().into()
}

impl ToTokens for Abstract2 {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let mut literals = vec![];
        let mut captures = vec![];
        let mut regexes = vec![];
        let mut matches = vec![];

        let mut match_num = 0;
        let mut new_match_ident = |prefix: &str| {
            let ident = quote::format_ident!("{}{}", prefix, match_num);
            match_num += 1;
            ident
        };

        let mut capture_num = 0;
        let mut new_capture_ident = |prefix: &str| {
            let ident = quote::format_ident!("{}{}", prefix, match_num);
            match_num += 1;
            ident
        };

        for seg in self.segments {
            match seg {
                Segment::Literal(lit) => {
                    let ident = new_match_ident("LIT_");
                    literals.push(quote! {
                        static #ident: &'static str = #lit;
                    });
                    matches.push(quote! {
                        match_lit(buf, #ident)?;
                    });
                }
                Segment::Capture((pos, rule)) => {
                    let ident = new_capture_ident("cap_");
                    quote! {
                    }
                }
            }
        }
    }
}
