use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{braced, parse::Parse, parse::ParseStream, parse_macro_input, Expr, Ident, Token};

enum RegexPattern {
    Any(Vec<RegexPattern>),
    All(Vec<RegexPattern>),
    Sequence(Vec<(String, String)>), // (pattern, flags) for each item in sequence
    One(String, String),             // (pattern, flags)
}

impl Parse for RegexPattern {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            let ident: Ident = input.parse()?;
            let content;
            braced!(content in input);
            match ident.to_string().as_str() {
                "any" | "all" => {
                    let mut patterns = Vec::new();
                    while !content.is_empty() {
                        patterns.push(content.parse()?);
                        if content.is_empty() {
                            break;
                        }
                        content.parse::<Token![,]>()?;
                    }
                    if ident == "any" {
                        Ok(RegexPattern::Any(patterns))
                    } else {
                        Ok(RegexPattern::All(patterns))
                    }
                }
                "sequence" | "seq" => {
                    let mut patterns = Vec::new();
                    while !content.is_empty() {
                        let expr: Expr = content.parse()?;
                        if let Expr::Lit(ref lit) = expr {
                            if let syn::Lit::Str(ref s) = lit.lit {
                                let pattern = s.value();
                                let flags = s.suffix().to_owned();
                                patterns.push((pattern, flags));
                            } else {
                                return Err(syn::Error::new_spanned(
                                    expr,
                                    "Expected string literal",
                                ));
                            }
                        } else {
                            return Err(syn::Error::new_spanned(expr, "Expected string literal"));
                        }
                        if content.is_empty() {
                            break;
                        }
                        content.parse::<Token![,]>()?;
                    }
                    Ok(RegexPattern::Sequence(patterns))
                }
                _ => Err(syn::Error::new(ident.span(), "Unknown identifier")),
            }
        } else {
            let expr: Expr = input.parse()?;
            if let Expr::Lit(ref lit) = expr {
                if let syn::Lit::Str(ref s) = lit.lit {
                    let pattern = s.value();
                    let flags = s.suffix().to_owned();
                    return Ok(RegexPattern::One(pattern, flags));
                }
            }
            Err(syn::Error::new_spanned(expr, "Expected string literal"))
        }
    }
}

impl ToTokens for RegexPattern {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            RegexPattern::Any(patterns) => {
                let patterns = patterns.iter().map(|p| quote! { #p });
                quote! {
                    Rule::Or(vec![#(#patterns),*])
                }
            }
            RegexPattern::All(patterns) => {
                let patterns = patterns.iter().map(|p| quote! { #p });
                quote! {
                    Rule::And(vec![#(#patterns),*])
                }
            }
            RegexPattern::Sequence(patterns) => {
                let patterns = patterns.iter().map(|(p, f)| {
                    let regex_str = if f.is_empty() {
                        quote! { #p }
                    } else {
                        let regex_with_flags = format!("(?{}){}", f, p);
                        quote! { #regex_with_flags }
                    };
                    quote! { lazy_regex::regex!(#regex_str) }
                });
                quote! {
                    Rule::Sequence(vec![#(#patterns),*])
                }
            }
            RegexPattern::One(pattern, flags) => {
                let regex_str = if flags.is_empty() {
                    quote! { #pattern }
                } else {
                    let regex_with_flags = format!("(?{}){}", flags, pattern);
                    quote! { #regex_with_flags }
                };
                quote! {
                    Rule::One(lazy_regex::regex!(#regex_str))
                }
            }
        }
        .to_tokens(tokens);
    }
}

#[proc_macro]
pub fn rule(input: TokenStream) -> TokenStream {
    let pattern = parse_macro_input!(input as RegexPattern);
    quote! { #pattern }.into()
}
