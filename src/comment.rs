//!
//!
use super::SpanExt;
use proc_macro2::Span;
use syn::punctuated::Pair;
use syn::*;

/// Creates a comment from `s`.
pub fn comment<S>(s: S) -> Attribute
where
    S: AsRef<str>,
{
    let span = Span::call_site();

    Attribute {
        style: AttrStyle::Outer,
        bracket_token: span.as_token(),
        pound_token: span.as_token(),
        meta: Meta::NameValue(MetaNameValue {
            path: Path {
                leading_colon: None,
                segments: vec![Pair::End(PathSegment {
                    ident: Ident::new("doc", span),
                    arguments: Default::default(),
                })]
                .into_iter()
                .collect(),
            },
            eq_token: span.as_token(),
            value: Expr::Lit(ExprLit {
                attrs: Default::default(),
                lit: Lit::Str(LitStr::new(s.as_ref(), span)),
            }),
        }),
    }
}
