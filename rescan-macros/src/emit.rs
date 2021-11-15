use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};

use crate::{Abstract, Rule, Segment};

pub(crate) fn emit(abs: Abstract) -> TokenStream {
    abs.to_token_stream().into()
}

impl ToTokens for Abstract {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let mut regex_array = vec![];
        for rule in self.rules.iter() {
            let regex_expr = match rule {
                Rule::Default { typ } => quote!(<#typ as DefaultScan>::DEFAULT_REGEX),
                Rule::Custom { regex, typ: _ } |
                Rule::Null { regex } => quote!(#regex),
            };
            regex_array.push(quote!(Regex::new(#regex_expr),));
        }

        let mut literals = vec![];
        let mut matches = vec![];
        let mut captures = vec![];

        for seg in self.segments.iter() {
            match seg {
                Segment::Literal(lit) => {
                    let ident = format_ident!("lit_{}", literals.len());
                    literals.push(quote! {
                        let #ident = #lit;
                    });
                    matches.push(quote! {
                        match_literal(reader, #ident)?;
                    });
                }
                Segment::Capture((None, rule)) => {
                    matches.push(quote! {
                        {
                            let str_len = match_regex(reader, &regexes[#rule])?.len();
                            advance_from_regex(reader, str_len);
                        }
                    });
                }
                Segment::Capture((Some(pos), rule)) => {
                    let cap_ident = format_ident!("cap_{}", pos);
                    let typ = match &self.rules[*rule] {
                        Rule::Custom { typ, .. } |
                        Rule::Default { typ } => typ.as_ref(),
                        // Parser ensures that only null captures can have null rules.
                        Rule::Null { .. } => unreachable!(),
                    };
                    matches.push(quote! {
                        let #cap_ident = {
                            let str = match_regex(reader, &regexes[#rule])?;
                            let val = <#typ as Scan>::scan(str).map_err(Error::from_parse_error)?;
                            let str_len = str.len();
                            advance_from_regex(reader, str_len);
                            val
                        };
                    });
                    captures.push((*pos, quote!(#cap_ident,), quote!(#typ,)));
                }
            }
        }

        captures.sort_unstable_by(|(lhs, ..), (rhs, ..)| lhs.cmp(rhs));
        let (captures, types): (Vec<_>, Vec<_>) = captures.into_iter()
            .map(|(_num, cap, typ)| (cap, typ))
            .unzip();

        let regex_array = join(&regex_array);
        let literals = join(&literals);
        let matches = join(&matches);
        let captures = join(&captures);
        let types = join(&types);

        let output = quote! {
            {
                use rescan::{Scan, DefaultScan, Error};
                use rescan::internal::*;

                fn build_regexes() -> Result<Vec<Regex>, RegexError> {
                    [#regex_array].into_iter().collect()
                }

                fn scan(reader: &mut dyn std::io::BufRead, regexes: &[Regex]) -> Result<(#types)> {
                    #literals
                    #matches
                    Ok((#captures))
                }

                Scanner::new(build_regexes, scan)
            }
        };
        *tokens = quote!(#tokens #output);
    }
}

fn join(token_streams: &[TokenStream2]) -> TokenStream2 {
    token_streams
        .iter()
        .fold(quote!(), |prev, cur| quote!(#prev #cur))
}
