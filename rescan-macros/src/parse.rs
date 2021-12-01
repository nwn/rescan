use syn::{self, spanned::Spanned as _};
use proc_macro::TokenStream;
use proc_macro_error::{emit_call_site_error, set_dummy, abort_if_dirty, abort_call_site, emit_error};
use crate::{Abstract, Rule};

pub(crate) fn parse(input: TokenStream) -> Abstract {
    // Until we have parsed the desired return types of the macro call, in case
    // of an error simply output a dummy expression with inferred types to
    // suppress further errors.
    set_dummy(quote::quote!(rescan::internal::dummy()));

    let abs = match syn::parse::<Concrete>(input) {
        Ok(abs) => abs,
        Err(err) => abort_call_site!("{}", err),
    };
    Abstract::from(abs)
}

impl From<Concrete> for Abstract {
    fn from(Concrete { segments, positional_rules, named_rules }: Concrete) -> Self {
        let mut pos_idx = 0;
        let mut rule_idx = 0;
        let mut bad_positions = vec![];
        let mut bad_names = vec![];
        let segments: Vec<_> = segments.into_iter().filter_map(|seg| Some(match seg {
            Segment::Literal(lit) => Segment::Literal(lit),
            Segment::Capture(cap) => {
                let pos = match cap.pos {
                    CapturePos::Null => None,
                    CapturePos::Explicit(pos) => Some(pos),
                    CapturePos::Implicit => {
                        pos_idx += 1;
                        Some(pos_idx - 1)
                    }
                };
                let rule = match cap.rule {
                    CaptureRule::Implicit => {
                        // Ensure that implicit positional references are within range.
                        let idx = rule_idx;
                        rule_idx += 1;
                        let num_positions = positional_rules.len();
                        if idx >= num_positions {
                            bad_positions.push((idx,));
                        }
                        idx
                    }
                    CaptureRule::Positional(rule) => {
                        // Ensure that explicit positional references are within range.
                        let num_positions = positional_rules.len();
                        if rule >= num_positions {
                            bad_positions.push((rule,));
                        }
                        rule
                    }
                    CaptureRule::Named(name) => {
                        let idx = named_rules.iter().position(|(rule_name, _)| rule_name == &name);
                        if let Some(idx) = idx {
                            positional_rules.len() + idx
                        } else {
                            // Ensure that the referenced named argument exists.
                            bad_names.push((name,));
                            return None; // TODO: Somehow keep this capture around for further error checking,
                            // even though we don't have an obvious rule to pair it with.
                        }
                    }
                };
                Segment::Capture((pos, rule))
            }
        })).collect();

        // Report any invalid references to positional rules.
        if let Some((last, rest)) = bad_positions.split_last() {
            let bad_refs = match rest {
                [] => format!("argument {}", last.0),
                [first] => format!("arguments {} and {}", first.0, last.0),
                _ => {
                    format!("arguments {}and {}", rest.iter().fold("".into(), |str, bad_ref| format!("{}{}, ", str, bad_ref.0)), last.0)
                }
            };
            let args_provided = match positional_rules.len() {
                1 => format!("only 1 argument was provided"),
                n => format!("only {} arguments were provided", n),
            };

            let msg = format!("invalid reference to positional {} ({})", bad_refs, args_provided);
            emit_call_site_error!(msg);
        }

        // Report any invalid references to named rules.
        for (bad_name,) in bad_names {
            let msg = format!("there is no argument named `{}`", bad_name);
            emit_call_site_error!(msg);
        }

        // Ensure that outputs are unique.
        let mut outputs: Vec<_> = iter_captures(&segments)
            .filter_map(|(pos, _rule)| *pos)
            .collect();
        outputs.sort(); // Stable so we can highlight the first occurrence among duplicates.
        for (first, rest) in equal_ranges(&outputs).filter_map(<[_]>::split_first) {
            for dup in rest {
                emit_call_site_error!("duplicate reference to capture position {}", dup;
                    note = "first defined here: {}", first);
            }
        }

        // Ensure that outputs cover the range 0..n, where n is the number of outputs.
        let missing_outputs: Vec<_> = (0..outputs.len())
            .filter(|pos| outputs.binary_search(&pos).is_err())
            .collect();
        if let Some((last, rest)) = missing_outputs.split_last() {
            let missing_outputs = match rest {
                [] => format!("capture {}", last),
                [first] => format!("captures {} and {}", first, last),
                _ => format!("captures {}and {}", rest.iter().fold("".into(), |str, missing| format!("{}{}, ", str, missing)), last),
            };
            let captures_specified = match outputs.len() {
                1 => format!("there was 1 capture spec"),
                n => format!("there were {} capture specs", n),
            };
            emit_call_site_error!("missing {} ({})", missing_outputs, captures_specified);
        }

        // Ensure that all named rules are unique.
        let mut names: Vec<_> = named_rules
            .iter()
            .map(|(name, _)| name)
            .collect();
        names.sort(); // Stable so we can highlight the first occurrence among duplicates.
        for (first, rest) in equal_ranges(&names).filter_map(<[_]>::split_first) {
            for dup in rest {
                emit_call_site_error!("duplicate argument name `{}`", dup;
                    note = "first defined here: {}", first);
            }
        }

        let mut rules = positional_rules;
        rules.extend(named_rules.into_iter().map(|(_name, rule)| rule));

        // Ensure that all rules are referenced.
        let mut rule_refs: Vec<_> = iter_captures(&segments)
            .map(|&(_pos, rule)| rule)
            .collect();
        rule_refs.sort_unstable();
        for (idx, _rule) in rules.iter().enumerate() {
            if rule_refs.binary_search(&idx).is_err() {
                emit_call_site_error!("unused argument: {}", idx);
            }
        }

        // Ensure that null rules are only referenced by null captures.
        for &(pos, rule) in iter_captures(&segments) {
            if pos.is_some() {
                let rule = &rules[rule];
                if let Rule::Null { .. } = rule {
                    emit_call_site_error!("untyped arguments cannot be used in captures";
                        help = "try specifying an output type for the argument or using a non-capturing specifier");
                }
            }
        }

        // At this point, we should have caught all syntax errors.
        // Stop here if any such errors have occurred.
        abort_if_dirty();

        Self {
            segments,
            rules,
        }
    }
}

// TODO: Replace this with [`group_by`] when it stabilizes:
// https://doc.rust-lang.org/stable/std/primitive.slice.html#method.group_by
// https://github.com/rust-lang/rust/issues/80552
struct EqualRanges<'a, T> {
    arr: &'a [T],
}
impl<'a, T: PartialEq> Iterator for EqualRanges<'a, T> {
    type Item = &'a [T];
    fn next(&mut self) -> Option<Self::Item> {
        let (first, rest) = self.arr.split_first()?;
        let mut idx = 0;
        while idx < rest.len() && first.eq(&rest[idx]) {
            idx += 1;
        }
        let (range, rest) = self.arr.split_at(idx + 1);
        self.arr = rest;
        Some(range)
    }
}
fn equal_ranges<T: PartialEq>(arr: &[T]) -> EqualRanges<T> {
    EqualRanges { arr }
}

fn iter_captures<Cap>(segments: &[Segment<Cap>]) -> impl Iterator<Item = &Cap> {
    segments.iter().filter_map(|seg| match seg {
        Segment::Capture(cap) => Some(cap),
        Segment::Literal(_) => None,
    })
}

struct Concrete {
    pub segments: Vec<Segment>,
    positional_rules: Vec<Rule>,
    named_rules: Vec<(String, Rule)>,
}

type Segment<Cap = Capture> = crate::Segment<Cap>;

struct Capture {
    pos: CapturePos,
    rule: CaptureRule,
}

enum CapturePos {
    Null,
    Implicit,
    Explicit(usize),
}

enum CaptureRule {
    Implicit,
    Positional(usize),
    Named(String),
}

struct Arg {
    name: Option<syn::Ident>,
    rule: Rule,
}

impl syn::parse::Parse for Concrete {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let format_string: syn::LitStr = input.parse()?;
        let segments = parse_format_string(format_string)
            .unwrap_or_else(|err| abort_call_site!("{}", err));
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
                        // TODO: This doesn't need to abort; it could just emit instead.
                        // But as is, continuing after such an error could cause issues with captures
                        // using incorrect positional arguments and yielding incorrect errors.
                        abort_call_site!("positional arguments must be before named arguments");
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
        if let Some(first) = attrs.first() {
            emit_error!(first.span(), "arguments cannot have attributes");
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
    FormatStringParser::new(&input.value()).parse()
}

struct FormatStringParser<'s> {
    source: &'s str,
    pos: usize,
    output: Vec<Segment>,
}
impl<'s> FormatStringParser<'s> {
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

        // Extract the first field: the output position of this capture.
        // It will be one of:
        //   - Null ("_"), meaning the regex will be matched, but not captured
        //   - Implicit (""), meaning the position of the output will be sequential from the preceding implicit capture
        //   - Explicit ("2"), meaning the captured value will be output in the given position, e.g. 2
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

        // Exit early if there are no remaining fields. We'll assume the rule used corresponds to its position.
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
        } else if let Some(next) = source.chars().next() {
            return Err(format!("Unexpected character '{}' in format string", next));
        } else {
            return Err(format!("Unexpected end of format string"));
        }

        // Extract the second field: a reference to the argument describing this capture's pattern and type.
        // This will be one of:
        //   - Implicit (""), meaning the argument is chosen by its position
        //   - Positional ("2"), meaning the argument at the given position will be used, e.g. 2
        //   - Named ("word"), meaning the argument with the given label will be used, e.g. "word = ..."
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

        // Ensure we're at the end of the capture.
        let source = self.remainder();
        if source.starts_with('}') {
            self.pos += 1;
            self.output.push(Segment::Capture(Capture {
                pos,
                rule,
            }));
            return Ok(());
        } else if let Some(next) = source.chars().next() {
            return Err(format!("Unexpected character '{}' in format string", next));
        } else {
            return Err(format!("Unexpected end of format string"));
        }
    }
}
