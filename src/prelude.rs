//! Prelude for convenience.

pub use super::{IdentExt, SpanExt, ToTokensExt};
pub use super::comment::comment;
pub use super::spanned_quote::Quote;
pub use proc_macro2::{Literal, Span, Term, TokenStream, TokenTree};
pub use syn::{self, Ident};
pub use syn::Span as SynSpan;
