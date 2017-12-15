use proc_macro2::{Span, Term};
use syn;
use synom_ext::FromSpan;

/// Extension trait for [Span][] and [syn::Span][].
///
///
///[Span]:../proc_macro2/struct.Span.html
///[syn::Span]:../syn/struct.Span.html
pub trait SpanExt: Copy {
    fn new_ident<S>(self, s: S) -> syn::Ident
    where
        S: AsRef<str>,
    {
        syn::Ident::new(Term::intern(s.as_ref()), self.as_syn_span())
    }

    /// Creates `Token` from `self`.
    fn as_token<Token>(self) -> Token
    where
        Token: FromSpan,
    {
        Token::from_span(self.into_pm2_span())
    }

    fn into_pm2_span(self) -> Span;


    fn as_syn_span(self) -> syn::Span {
        syn::Span(self.into_pm2_span())
    }
}

impl SpanExt for Span {
    fn into_pm2_span(self) -> Self {
        self
    }
}
impl SpanExt for syn::Span {
    fn into_pm2_span(self) -> Span {
        self.0
    }
}
