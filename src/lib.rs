//! Utils for implementing proc-macro. Works on stable.
//!
//!
//!

#![recursion_limit = "128"]

pub extern crate proc_macro2;
pub extern crate proc_macro;
pub extern crate quote;
pub extern crate syn;

pub use self::span_ext::SpanExt;
use quote::{ToTokens, Tokens};
pub use spanned_quote::Quote;
use syn::Ident;

pub mod comment;
pub mod prelude;
pub mod spanned_quote;
pub mod respan;
mod span_ext;
pub mod synom_ext;

/// Extension trait for [syn::Ident][].
///
///
///[syn::Ident]:../syn/struct.Ident.html
pub trait IdentExt {
    /// Creates a new ident with same span by applying `map` to `self`.
    fn new_ident_with<F, S>(&self, map: F) -> Ident
    where
        F: for<'a> FnOnce(&'a str) -> S,
        S: AsRef<str>;
}

impl IdentExt for Ident {
    /// Creates a new ident with same span by applying `map` to `self`.
    fn new_ident_with<F, S>(&self, map: F) -> Ident
    where
        F: for<'a> FnOnce(&'a str) -> S,
        S: AsRef<str>,
    {
        Ident::new(map(self.as_ref()).as_ref(), self.span)
    }
}

pub trait ToTokensExt: ToTokens {
    fn dump(&self) -> Tokens {
        let mut tmp = Tokens::new();
        self.to_tokens(&mut tmp);
        tmp
    }

    /// Usage: `Quote::new(body.first_last())`
    fn first_last(&self) -> respan::FirstLast {
        respan::FirstLast::from_tokens(&self)
    }
}

impl<T: ToTokens> ToTokensExt for T {}
