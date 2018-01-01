//!
//!
use super::SpanExt;
use proc_macro2::{Literal, Spacing, Span, TokenNode, TokenTree};
use syn::*;
use syn::punctuated::Element;

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
        is_sugared_doc: true,
        path: Path {
            leading_colon: None,
            segments: vec![
                Element::End(PathSegment {
                    ident: Ident::new("doc", span),
                    arguments: Default::default(),
                }),
            ].into_iter()
                .collect(),
        },
        tts: vec![
            TokenTree {
                span,
                kind: TokenNode::Op('=', Spacing::Alone),
            },
            TokenTree {
                span,
                kind: TokenNode::Literal(Literal::string(s.as_ref())),
            },
        ].into_iter()
            .collect(),
    }
}
