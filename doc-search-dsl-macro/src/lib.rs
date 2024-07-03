use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{braced, parse::Parse, parse::ParseStream, parse_macro_input, Expr, Ident, Token};

enum RegexPattern {
    Any(Vec<String>),
    All(Vec<RegexPattern>),
    Sequence(Vec<String>),
    One(String),
}

impl Parse for RegexPattern {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            let ident: Ident = input.parse()?;
            let content;
            braced!(content in input);
            match ident.to_string().as_str() {
                "any" => {
                    let patterns = content
                        .parse_terminated(Expr::parse, Token![,])?
                        .into_iter()
                        .map(|expr| {
                            if let Expr::Lit(ref lit) = expr {
                                if let syn::Lit::Str(ref s) = lit.lit {
                                    return Ok(s.value());
                                }
                            }
                            Err(syn::Error::new_spanned(expr, "Expected string literal"))
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok(RegexPattern::Any(patterns))
                }
                "all" => {
                    let patterns = content
                        .parse_terminated(RegexPattern::parse, Token![,])?
                        .into_iter()
                        .collect();
                    Ok(RegexPattern::All(patterns))
                }
                "sequence" => {
                    let patterns = content
                        .parse_terminated(Expr::parse, Token![,])?
                        .into_iter()
                        .map(|expr| {
                            if let Expr::Lit(ref lit) = expr {
                                if let syn::Lit::Str(ref s) = lit.lit {
                                    return Ok(s.value());
                                }
                            }
                            Err(syn::Error::new_spanned(expr, "Expected string literal"))
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok(RegexPattern::Sequence(patterns))
                }
                _ => Err(syn::Error::new(ident.span(), "Unknown identifier")),
            }
        } else {
            let expr: Expr = input.parse()?;
            if let Expr::Lit(ref lit) = expr {
                if let syn::Lit::Str(ref s) = lit.lit {
                    return Ok(RegexPattern::One(s.value()));
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
                let patterns = patterns.iter().map(|p| quote! { R::One(regex!(#p)) });
                quote! {
                    R::Or(vec![#(#patterns),*])
                }
            }
            RegexPattern::All(patterns) => {
                let patterns = patterns.iter().map(|p| quote! { #p });
                quote! {
                    R::And(vec![#(#patterns),*])
                }
            }
            RegexPattern::Sequence(patterns) => {
                quote! {
                    R::Sequence(vec![#(regex!(#patterns)),*])
                }
            }
            RegexPattern::One(pattern) => {
                quote! {
                    R::One(regex!(#pattern))
                }
            }
        }
        .to_tokens(tokens);
    }
}

#[proc_macro]
pub fn r(input: TokenStream) -> TokenStream {
    let pattern = parse_macro_input!(input as RegexPattern);
    quote! { #pattern }.into()
}
