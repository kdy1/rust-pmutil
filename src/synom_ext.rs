//! Utils for tokens from synom::tokens.

use proc_macro2::Span;
use syn::token::*;

/// See [SpanExt#as_token][] for usage. Create tokens from [Span][].
///
///
///[SpanExt#as_token]:../trait.SpanExt.html#method.as_token
///[Span]:../../proc_macro2/struct.Span.html
pub trait FromSpan {
    fn from_span(span: Span) -> Self;
}

impl FromSpan for Span {
    #[inline(always)]
    fn from_span(span: Span) -> Self {
        span
    }
}

macro_rules! impl_array {
    ($n:expr) => {
        impl<T: FromSpan + Copy> FromSpan for [T; $n] {
            #[inline(always)]
            fn from_span(span: Span) -> Self{
                let e = FromSpan::from_span(span);
                [e; $n]
            }
        }
    };
    ($n:expr, $($rest:tt)*) => {
        impl_array!($n);
        impl_array!($($rest)*);
    };
}

impl_array!(1, 2, 3, 4);

macro_rules! bridge_spans {
    // Done
    ($t:path) => {
        impl FromSpan for $t {
            fn from_span(span: Span) -> Self {
                let spans = FromSpan::from_span(span);
                $t { spans }
            }
        }
    };

    ($t:path, $($rest:tt)+) => {
        bridge_spans!($t);
        bridge_spans!($($rest)*);
    };
}

macro_rules! bridge {
    // Done
    ($t:path) => {
        impl FromSpan for $t {
            fn from_span(span: Span) -> Self {
                let span = FromSpan::from_span(span);
                $t { span }
            }
        }
    };
    ($t:path,) => {
        bridge!($t);
    };

    ($t:path, $($rest:tt)+) => {
        bridge!($t);
        bridge!($($rest)*);
    };
}

bridge_spans!(
    AddEq, AndAnd, AndEq, CaretEq, Colon2, DivEq, Dot2, Dot3, DotDotEq, EqEq, FatArrow, LArrow, Le,
    Lt, MulEq, Ne, Or, OrEq, OrOr, Pound, Ge, RArrow, RemEq, Shl, ShlEq, Shr, ShrEq, SubEq, Gt,
    Rem, Tilde, Underscore, Star, Sub, Semi, Eq, Dot, Question, Add, And, At, Bang, Caret, Colon,
    Dollar, Comma, Div
);

bridge!(
    Mod, Abstract, As, Async, Auto, Become, Box, Brace, Bracket, Break, Const, Continue, Crate,
    Default, Do, Dyn, Else, Enum, Extern, Final, Fn, For, Group, If, Impl, In, Let, Loop, Macro,
    Match, Move, Mut, Override, Paren, Priv, Pub, Ref, Return, SelfType, SelfValue, Static, Struct,
    Super, Trait, Try, Type, Typeof, Union, Unsafe, Unsized, Use, Virtual, Where, While, Yield
);
