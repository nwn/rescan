use syn;
use proc_macro2::{Span, TokenStream as TokenStream2};

#[derive(Debug)]
pub struct Abstract {
    pub segments: Vec<Segment>,
    positional_rules: Vec<Rule>,
    named_rules: Vec<(String, Rule)>,
}

#[derive(Debug)]
pub enum Segment {
    Literal(String),
    Capture(Capture),
}

#[derive(Debug)]
struct Capture {
    pos: CapturePos,
    rule: CaptureRule,
}

#[derive(Debug)]
enum CapturePos {
    Null,
    Implicit,
    Explicit(u32),
}

#[derive(Debug)]
enum CaptureRule {
    Implicit,
    Positional(u32),
    Named(String),
}

#[derive(Debug)]
struct Arg {
    name: Option<syn::Ident>,
    rule: Rule,
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

impl syn::parse::Parse for Abstract {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let format_string: syn::LitStr = input.parse()?;
        let segments = parse_format_string(format_string)
            .map_err(|err| syn::Error::new(Span::call_site(), err))?;
        let mut positional_rules = vec![];
        let mut named_rules = vec![];
        if !input.is_empty() {
            let _comma: syn::Token![,] = input.parse()?;
            let args = syn::punctuated::Punctuated::<Arg, syn::Token![,]>::parse_terminated(input)?;
            for Arg { name, rule } in args {
                if let Some(name) = name {
                    named_rules.push((name.to_string(), rule));
                } else {
                    if !named_rules.is_empty() {
                        panic!("Positional arguments must be before named arguments")
                    }
                    positional_rules.push(rule);
                }
            }
        }
        Ok(Self {
            segments,
            positional_rules,
            named_rules,
        })
    }
}

impl syn::parse::Parse for Arg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::{ExprCast, Ident, Token, Type};

        // First test if there is a leading `ident =`, which uniquely identifies
        // a named argument.
        let name = if input.peek(Ident) && input.peek2(Token![=]) {
            let name: Ident = input.parse()?;
            let _eq: Token![=] = input.parse()?;
            Some(name)
        } else {
            None
        };

        // Try to parse from here as a type. If successful, with a follow set
        // of only ",", return a type-only argument. Otherwise, backtrack and
        // try something else.
        let try_type = input.fork();
        if try_type.parse::<Type>().is_ok() && (try_type.is_empty() || try_type.peek(Token![,])) {
            let typ = Box::new(input.parse()?);
            return Ok(Self {
                name,
                rule: Rule::Default { typ },
            });
        }

        // At this point, we assume there must be a regex expression.
        let ExprCast {
            attrs,
            expr: regex,
            as_token: _,
            ty: typ,
        } = input.parse()?;
        if !attrs.is_empty() {
            // TODO: Throw error
        }

        // Check for the null type (written `_`) that can only be used with null
        // captures (those which don't actually extract a value).
        let rule = if let Type::Infer(_) = *typ {
            Rule::Null { regex }
        } else {
            Rule::Custom {
                regex,
                typ,
            }
        };

        Ok(Self {
            name,
            rule,
        })
    }
}

fn parse_format_string(input: syn::LitStr) -> Result<Vec<Segment>, String> {
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
        fn remainder(&self) -> &str {
            &self.source[self.pos..]
        }
        fn parse(mut self) -> Result<Vec<Segment>, String> {
            loop {
                if self.pos >= self.source.len() { break; }
                self.parse_literal()?;
                if self.pos >= self.source.len() { break; }
                self.parse_capture()?;
            }
            Ok(self.output)
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
            // First check for the common case.
            if self.remainder().starts_with("{}") {
                self.pos += 2;
                self.output.push(Segment::Capture(Capture {
                    pos: CapturePos::Implicit,
                    rule: CaptureRule::Implicit,
                }));
                return Ok(());
            }

            assert!(self.remainder().starts_with("{"));
            self.pos += 1;

            let source = self.remainder();
            let end_of_pos = source
                .find(|ch: char| ch != '_' && !ch.is_ascii_digit())
                .ok_or_else(|| String::from("Unmatched '{' in format string"))?;
            let pos = &source[..end_of_pos];
            let pos = match pos {
                "_" => CapturePos::Null,
                "" => CapturePos::Implicit,
                _ => if let Ok(num) = pos.parse() {
                    CapturePos::Explicit(num)
                } else {
                    return Err(format!("Invalid position: '{}'", pos));
                }
            };
            self.pos += end_of_pos;

            let source = self.remainder();
            if source.starts_with('}') {
                self.pos += 1;
                self.output.push(Segment::Capture(Capture {
                    pos,
                    rule: CaptureRule::Implicit,
                }));
                return Ok(());
            } else if source.starts_with(':') {
                self.pos += 1;
            } else {
                return Err(format!("Unexpected character '{}' in format string", source.chars().next().unwrap()));
            }

            let source = self.remainder();
            let end_of_rule = source
                .find(|ch: char| ch != '_' && !ch.is_ascii_alphanumeric())
                .ok_or_else(|| String::from("Unmatched '{' in format string"))?;
            let rule = &source[..end_of_rule];
            let rule = if rule.starts_with(|ch: char| ch == '_' || ch.is_ascii_alphabetic()) {
                CaptureRule::Named(rule.into())
            } else if rule.is_empty() {
                CaptureRule::Implicit
            } else if let Ok(num) = rule.parse() {
                CaptureRule::Positional(num)
            } else {
                return Err(format!("Invalid rule: '{}'", rule));
            };
            self.pos += end_of_rule;

            let source = self.remainder();
            if source.starts_with('}') {
                self.pos += 1;
                self.output.push(Segment::Capture(Capture {
                    pos,
                    rule,
                }));
                return Ok(());
            } else {
                return Err(format!("Unexpected character '{}' in format string", source.chars().next().unwrap()));
            }
        }
    }

    Parser::new(&input.value()).parse()
}
