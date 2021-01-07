mod emit;
mod parse;

use proc_macro::TokenStream;

#[proc_macro]
pub fn scanner(input: TokenStream) -> TokenStream {
    emit::emit(parse::parse(input).dynamic_dispatch())
}

#[proc_macro]
pub fn static_scanner(input: TokenStream) -> TokenStream {
    emit::emit(parse::parse(input))
}

struct Abstract {
    segments: Vec<Segment<(Option<usize>, usize)>>,
    rules: Vec<Rule>,
    dispatch: Dispatch,
}
impl Abstract {
    fn dynamic_dispatch(mut self) -> Self {
        self.dispatch = Dispatch::Dynamic;
        self
    }
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

enum Dispatch {
    Static,
    Dynamic,
}
