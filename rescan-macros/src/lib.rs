#[cfg(test)]
mod tests {
    #[test]
    fn parse_format() {
        use super::*;
        let parsed = parse("{test} this is a {test\\{string that should{2,4}} work }} just {{ fine");
        assert_eq!(parsed, Ok(FormatSpec { segments: vec![
            Segment::Capture(String::from("test")),
            Segment::Literal(String::from(" this is a ")),
            Segment::Capture(String::from("test\\{string that should{2,4}")),
            Segment::Literal(String::from(" work } just { fine"))
        ]}));
    }
}

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};

mod emit;
mod parse;

#[proc_macro]
pub fn scanner(input: TokenStream) -> TokenStream {
    let abs2 = parse::parse(input);
    println!("{:#?}", abs2);
    let output = emit::emit(abs2);
    println!("{}", output);

    output
}

#[proc_macro]
pub fn format(input: TokenStream) -> TokenStream {
    let format_str = syn::parse_macro_input!(input as syn::LitStr);
    let format_str = format_str.value();

    let format = parse(&format_str).unwrap();

    quote!(#format).into()
}

#[derive(Debug, PartialEq, Eq)]
struct FormatSpec {
    segments: Vec<Segment>,
}

impl ToTokens for FormatSpec {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let new_tokens = self.to_token_stream();
        *tokens = quote!(#tokens#new_tokens);
    }
    fn to_token_stream(&self) -> TokenStream2 {
        let mut segments = quote!();
        for segment in &self.segments {
            let segment = match segment {
                Segment::Literal(str) => quote!(Segment::Literal(#str),),
                Segment::Capture(str) => quote!(Segment::Capture(#str),),
            };
            segments = quote!(#segments #segment);
        }
        quote! {
            FormatSpec::new(::std::vec![
                #segments
            ])
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Segment {
    Literal(String),
    Capture(String),
}

struct Parser<'s> {
    source: &'s str,
    pos: usize,
    output: Vec<Segment>,
}
impl<'s> Parser<'s> {
    fn new(source: &'s str) -> Self {
        Self {
            source,
            pos: 0,
            output: vec![],
        }
    }
    fn parse(mut self) -> Result<FormatSpec, String> {
        loop {
            if self.pos >= self.source.len() { break; }
            self.parse_literal()?;
            if self.pos >= self.source.len() { break; }
            self.parse_capture()?;
        }
        Ok(FormatSpec { segments: self.output })
    }
    fn parse_literal(&mut self) -> Result<(), String> {
        let mut result = String::new();
        let source = &self.source.as_bytes();
        while let Some(&ch) = source.get(self.pos) {
            match ch as char {
                '{' | '}' if source.get(self.pos + 1) == Some(&ch) => self.pos += 1,
                '{' => break,
                '}' => return Err("Unmatched '}' in format string".into()),
                _ => (),
            }
            result.push(ch as char);
            self.pos += 1;
        }
        if !result.is_empty() {
            self.output.push(Segment::Literal(result));
        }
        Ok(())
    }
    fn parse_capture(&mut self) -> Result<(), String> {
        let mut result = String::new();
        let source = &self.source.as_bytes();
        assert_eq!(Some(b'{'), source.get(self.pos).copied());
        self.pos += 1;
        let mut escape = false;
        let mut brace_level = 0;
        while let Some(&ch) = source.get(self.pos) {
            if ch == b'\\' && !escape {
                escape = true;
            } else if escape {
                escape = false;
            } else {
                match ch as char {
                    '{' => brace_level += 1,
                    '}' => brace_level -= 1,
                    _ => (),
                }
                escape = false;
            }
            self.pos += 1;
            if brace_level >= 0 {
                result.push(ch as char);
            } else {
                break;
            }
        }
        if brace_level != -1 {
            return Err("Unmatched '{' in format string".into());
        }
        self.output.push(Segment::Capture(result));
        Ok(())
    }
}

fn parse(format_str: &str) -> Result<FormatSpec, String> {
    Parser::new(format_str).parse()
}





// This is the idea of how the actual scanning function might work:

fn call_scan<T: DoScan>() -> T {
    T::do_scan()
}

trait DoScan {
    fn do_scan() -> Self;
}

impl<> DoScan for () {
    fn do_scan() -> Self {}
}
impl<A> DoScan for (A,)
    where A: std::str::FromStr,
{
    fn do_scan() -> Self { todo!() }
}
impl<A, B> DoScan for (A, B)
    where A: std::str::FromStr,
          B: std::str::FromStr,
{
    fn do_scan() -> Self { todo!() }
}
impl<A, B, C> DoScan for (A, B, C)
    where A: std::str::FromStr,
          B: std::str::FromStr,
          C: std::str::FromStr,
{
    fn do_scan() -> Self { todo!() }
}
