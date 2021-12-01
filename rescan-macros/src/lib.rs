mod emit;
mod parse;

use proc_macro::TokenStream;
use proc_macro_error::{proc_macro_error};

#[proc_macro]
#[proc_macro_error]
pub fn scanner(input: TokenStream) -> TokenStream {
    emit::emit(parse::parse(input))
}

struct Abstract {
    segments: Vec<Segment<(Option<usize>, usize)>>,
    rules: Vec<Rule>,
}

enum Segment<Cap> {
    Literal(String),
    Capture(Cap),
}

#[derive(Debug)]
enum Rule {
    Null {
        regex: Box<syn::Expr>,
    },
    Default {
        typ: Box<syn::Type>,
    },
    Custom {
        regex: Box<syn::Expr>,
        typ: Box<syn::Type>,
    },
}
