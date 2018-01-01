//! Span support for quasi-quotting.

use proc_macro2::{Span, TokenNode, TokenStream, TokenTree};
use quote::{ToTokens, Tokens};
use std::cell::Cell;

pub trait Respan {
    /// Used while quasi quotting.
    fn span_for(&self, kind: &TokenNode) -> Span;

    fn respan(&self, tt: TokenTree) -> TokenTree {
        TokenTree {
            span: self.span_for(&tt.kind),
            kind: tt.kind,
        }
    }
}

impl Respan for Span {
    fn span_for(&self, _: &TokenNode) -> Span {
        *self
    }
}

impl<'a, S> Respan for &'a S
where
    S: ?Sized + Respan,
{
    fn span_for(&self, node: &TokenNode) -> Span {
        <S as Respan>::span_for(self, node)
    }
}

impl<S> Respan for Box<S>
where
    S: ?Sized + Respan,
{
    fn span_for(&self, node: &TokenNode) -> Span {
        <S as Respan>::span_for(self, node)
    }
}

#[derive(Debug, Clone)]
pub struct FirstLast {
    first: Cell<Option<Span>>,
    last: Span,
}
impl Respan for FirstLast {
    fn span_for(&self, _: &TokenNode) -> Span {
        // Default value of Option<_> is None, so Cell<Option<_>>.take() works
        self.first.take().unwrap_or(self.last)
    }
}

impl FirstLast {
    pub fn from_tokens(tokens: &ToTokens) -> Self {
        let mut spans = Tokens::new();
        tokens.to_tokens(&mut spans);
        let good_tokens = TokenStream::from(spans).into_iter().collect::<Vec<_>>();
        let first_span = good_tokens
            .first()
            .map(|t| t.span)
            .unwrap_or(Default::default());
        let last = good_tokens.last().map(|t| t.span).unwrap_or(first_span);
        FirstLast {
            first: Cell::new(Some(first_span)),
            last,
        }
    }
}
