//! Span support for quasi-quotting.

use proc_macro2::{Span, TokenStream, TokenTree};
use quote::ToTokens;
use std::cell::Cell;

pub trait Respan {
    /// Used while quasi quotting.
    fn next_span(&self) -> Span;

    fn respan(&self, mut tt: TokenTree) -> TokenTree {
        let span = self.next_span();

        match tt {
            TokenTree::Group(ref mut tt) => tt.set_span(span),
            TokenTree::Ident(ref mut tt) => tt.set_span(span),
            TokenTree::Punct(ref mut tt) => tt.set_span(span),
            TokenTree::Literal(ref mut tt) => tt.set_span(span),
        }

        tt
    }
}

impl Respan for Span {
    fn next_span(&self) -> Span {
        *self
    }
}

impl<'a, S> Respan for &'a S
where
    S: ?Sized + Respan,
{
    fn next_span(&self) -> Span {
        <S as Respan>::next_span(self)
    }
}

impl<S> Respan for Box<S>
where
    S: ?Sized + Respan,
{
    fn next_span(&self) -> Span {
        <S as Respan>::next_span(self)
    }
}

#[derive(Debug, Clone)]
pub struct FirstLast {
    first: Cell<Option<Span>>,
    last: Span,
}
impl Respan for FirstLast {
    fn next_span(&self) -> Span {
        // Default value of Option<_> is None, so Cell<Option<_>>.take() works
        self.first.take().unwrap_or(self.last)
    }
}

impl FirstLast {
    pub fn from_tokens(tokens: &ToTokens) -> Self {
        let mut spans = TokenStream::new();
        tokens.to_tokens(&mut spans);
        let good_tokens = TokenStream::from(spans).into_iter().collect::<Vec<_>>();
        let first_span = good_tokens
            .first()
            .map(|t| t.span())
            .unwrap_or(Span::call_site());
        let last = good_tokens.last().map(|t| t.span()).unwrap_or(first_span);
        FirstLast {
            first: Cell::new(Some(first_span)),
            last,
        }
    }
}
