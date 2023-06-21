use crate::synom_ext::FromSpan;
use proc_macro2::Span;

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
        syn::Ident::new(s.as_ref(), self.into_pm2_span())
    }

    /// Creates `Token` from `self`.
    fn as_token<Token>(self) -> Token
    where
        Token: FromSpan,
    {
        Token::from_span(self.into_pm2_span())
    }

    fn into_pm2_span(self) -> Span;
}

impl SpanExt for Span {
    fn into_pm2_span(self) -> Self {
        self
    }
}
