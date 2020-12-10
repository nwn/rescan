use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};

use crate::{Abstract, Rule, Segment};

pub(crate) fn emit(abs: Abstract) -> TokenStream {
    abs.to_token_stream().into()
}

impl ToTokens for Abstract {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let mut static_regexes = vec![];
        let mut local_regexes = vec![];
        for (idx, rule) in self.rules.iter().enumerate() {
            match rule {
                Rule::Null { regex } => todo!(),
                Rule::Default { typ } => todo!(),
                Rule::Custom { regex, typ: _ } => {
                    let static_ident = format_ident!("REGEX_{}", idx);
                    static_regexes.push(quote! {
                        static #static_ident: LazyRegex = LazyRegex::new(|| {
                            Regex::new(#regex)
                        });
                    });
                    let local_ident = format_ident!("regex_{}", idx);
                    local_regexes.push(quote! {
                        let #local_ident = #static_ident.as_ref()?;
                    });
                }
            }
        }

        let mut literals = vec![];
        let mut matches = vec![];
        let mut captures = vec![];

        for (idx, seg) in self.segments.iter().enumerate() {
            match seg {
                Segment::Literal(lit) => {
                    let ident = format_ident!("lit_{}", idx);
                    literals.push(quote! {
                        let #ident = #lit;
                    });
                    matches.push(quote! {
                        match_literal(reader, #ident)?;
                    });
                }
                Segment::Capture((None, rule)) => {
                    let regex_ident = format_ident!("regex_{}", rule);
                    matches.push(quote! {
                        {
                            let str_len = match_regex(reader, #regex_ident)?;
                            advance_from_regex(reader, str_len);
                        }
                    });
                }
                Segment::Capture((Some(pos), rule)) => {
                    let cap_ident = format_ident!("cap_{}", pos);
                    let (typ, regex) = match self.rules[*rule] {
                        Rule::Custom { ref typ, .. } => (
                            typ.as_ref(),
                            format_ident!("regex_{}", rule).to_token_stream(),
                        ),
                        Rule::Default { ref typ } => (
                            typ.as_ref(),
                            quote!(todo!()),
                        ),
                        Rule::Null { .. } => unreachable!(),
                    };
                    matches.push(quote! {
                        let #cap_ident = {
                            let str = match_regex(reader, #regex)?;
                            let val = <#typ as Scan>::scan(str).unwrap();
                            let str_len = str.len();
                            advance_from_regex(reader, str_len);
                            val
                        };
                    });
                    captures.push((pos, quote!(#cap_ident,), quote!(#typ,)));
                }
            }
        }

        captures.sort_unstable_by(|lhs, rhs| lhs.0.cmp(rhs.0));
        let (captures, types): (Vec<_>, Vec<_>) = captures.into_iter()
            .map(|(_num, cap, typ)| (cap, typ))
            .unzip();

        let static_regexes = join(&static_regexes);
        let literals = join(&literals);
        let local_regexes = join(&local_regexes);
        let matches = join(&matches);
        let captures = join(&captures);
        let types = join(&types);

        let output = quote! {
            {
                use rescan::{Scan, DefaultScan};
                use rescan::internal::*;

                #static_regexes

                fn scanner(reader: &mut impl std::io::BufRead) -> Result<(#types)> {
                    #literals

                    #local_regexes

                    #matches

                    Ok((#captures))
                }

                scanner
            }
        };
        *tokens = quote!(#tokens#output);
    }
}

fn join(token_streams: &[TokenStream2]) -> TokenStream2 {
    token_streams
        .iter()
        .fold(quote!(), |prev, cur| quote!(#prev #cur))
}
