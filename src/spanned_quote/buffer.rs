use crate::respan::{self, Respan};
use proc_macro2::{Delimiter, Group, Ident, Span, TokenStream, TokenTree};
use quote::{ToTokens, TokenStreamExt};
use std::collections::HashSet;
use std::env;
use std::fmt::{self, Display, Formatter, Write};
use syn::parse::Parse;

/// Buffer for quasi quotting.
pub struct Quote {
    tts: TokenStream,
    span: Option<Box<(dyn Respan + 'static)>>,
    /// Location of smart_quote! invokations.
    /// Used for error reporting.
    sources: HashSet<Location>,
}

const INVALID_SPAN_STATE: &str = "Span is in invalid state.
Closure provided to push_group should not panic.";

/// Location of `smart_quote!` macro invocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Location {
    pub file_name: &'static str,
    pub line: u32,
    pub col: u32,
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file_name, self.line, self.col)?;
        Ok(())
    }
}

impl Quote {
    pub fn new<S: Respan + 'static>(span: S) -> Self {
        Quote {
            span: Some(Box::new(span)),
            tts: TokenStream::new(),
            sources: Default::default(),
        }
    }

    /// Shorthand for
    ///
    /// ```rust,ignore
    /// Quote::new(Span::call_site())
    /// ```
    pub fn new_call_site() -> Self {
        Self::new(Span::call_site())
    }

    /// Shorthand for
    ///
    /// ```rust,ignore
    /// Quote::new(tokens.first_last())
    /// ```
    pub fn from_tokens(tokens: &dyn ToTokens) -> Self {
        Self::new(respan::FirstLast::from_tokens(tokens))
    }

    /// Shorthand for
    ///
    ///```rust,ignore
    /// tokens
    ///   .as_ref()
    ///   .map(|tokens| Quote::from_tokens(tokens))
    ///   .unwrap_or_else(|| Quote::new(default_span))
    ///```
    ///
    pub fn from_tokens_or<T: ToTokens>(tokens: &Option<T>, default_span: Span) -> Self {
        match *tokens {
            Some(ref tokens) => Self::from_tokens(tokens),
            None => Self::new(default_span),
        }
    }
}

impl Quote {
    /// Parse tokens as `Node`.
    /// Panics if parsing failed.
    pub fn parse<Node>(self) -> Node
    where
        Node: Parse,
    {
        // TODO: Use span to report error.
        let Quote { tts, sources, .. } = self;

        let debug_tts = if env::var("DBG_DUMP").is_ok() {
            Some(tts.clone())
        } else {
            None
        };

        syn::parse2(tts).unwrap_or_else(|err| {
            let debug_tts: &dyn Display = match debug_tts {
                Some(ref tts) => tts,
                None => {
                    &"To get code failed to parse,
 please set environment variable `DBG_DUMP` and run in again"
                }
            };

            let notes = {
                let mut b = String::from("Note: quasi quotting was invoked from:\n");
                for src in &sources {
                    writeln!(b, "       {src}").unwrap();
                }
                b
            };

            panic!(
                "Quote::parse() failed.
                {notes}
Error from syn: {err}
    >>>>>
        {debug_tts}
    <<<<<",
                notes = notes,
                err = err,
                debug_tts = debug_tts
            )
        })
    }
}

/// Methods for quasi-quotting.
impl Quote {
    #[doc(hidden)]
    /// Reports location of `smart_quote!` invocation.
    pub fn report_loc(&mut self, loc: Location) {
        self.sources.insert(loc);
    }

    pub fn quote_with<F>(mut self, quote: F) -> Self
    where
        F: FnOnce(&mut Self),
    {
        (quote)(&mut self);
        self
    }

    /// Parse `token` and append it to `self`.
    pub fn push_parsed(&mut self, token: &str) {
        let Quote {
            ref mut span,
            ref mut tts,
            ..
        } = *self;

        token
            .parse::<TokenStream>()
            .expect("Failed to parse token to quote")
            .into_iter()
            .map(|tt| span.as_ref().expect(INVALID_SPAN_STATE).respan(tt))
            .for_each(|tt| tts.append(tt));
    }

    /// Append `tt` to `self`.
    pub fn push_tt(&mut self, tt: TokenTree) {
        self.tts.append(tt)
    }

    /// Respan symbol and append it to `self`.
    pub fn push_sym(&mut self, term: &str) {
        let span = self.next_span();
        self.push_tt(TokenTree::Ident(Ident::new(term, span)))
    }

    fn next_span(&self) -> Span {
        self.span.as_ref().expect(INVALID_SPAN_STATE).next_span()
    }

    /// Respan and append `TokenStream::Group`
    pub fn push_group<F>(&mut self, delim: Delimiter, child: F)
    where
        F: FnOnce(&mut Quote),
    {
        //TODO: Exception safety
        let span = self.span.take().expect(INVALID_SPAN_STATE);
        let mut sub = Quote::new(span);
        child(&mut sub);
        self.sources.extend(sub.sources);

        debug_assert!(self.span.is_none());
        self.span = Some(sub.span.expect(INVALID_SPAN_STATE));

        self.push_tt(TokenTree::Group(Group::new(delim, sub.tts)))
    }

    /// Appends node into `self` **without respanning**.
    pub fn push_tokens<T: ?Sized + ToTokens>(&mut self, node: &T) {
        node.to_tokens(&mut self.tts);
    }
}

impl IntoIterator for Quote {
    type IntoIter = <TokenStream as IntoIterator>::IntoIter;
    type Item = <TokenStream as IntoIterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.tts.into_iter()
    }
}

impl From<Quote> for TokenStream {
    fn from(quote: Quote) -> Self {
        quote.tts
    }
}

impl From<Quote> for proc_macro::TokenStream {
    fn from(quote: Quote) -> Self {
        TokenStream::from(quote).into()
    }
}

impl ToTokens for Quote {
    fn to_tokens(&self, dst: &mut TokenStream) {
        self.tts.to_tokens(dst)
    }

    fn into_token_stream(self) -> TokenStream {
        self.tts
    }
}
