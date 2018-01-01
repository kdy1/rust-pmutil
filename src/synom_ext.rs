//! Utils for tokens from synom::tokens.

use proc_macro2::Span;
use syn::token as tokens;

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

macro_rules! bridge {
    // Done
    ($t:path) => {
        impl FromSpan for $t {
            fn from_span(span: Span) -> Self {
                let span = FromSpan::from_span(span);
                $t(span)
            }
        }
    };

    ($t:path, $($rest:tt)+) => {
        bridge!($t);
        bridge!($($rest)*);
    };
}

bridge!(
    tokens::Add,
    tokens::AddEq,
    tokens::And,
    tokens::AndAnd,
    tokens::AndEq,
    tokens::As,
    tokens::At,
    tokens::Auto,
    tokens::Bang,
    tokens::Box,
    tokens::Brace,
    tokens::Bracket,
    tokens::Break,
    tokens::CapSelf,
    tokens::Caret,
    tokens::CaretEq,
    tokens::Catch,
    tokens::Colon,
    tokens::Colon2,
    tokens::Comma,
    tokens::Const,
    tokens::Continue,
    tokens::Crate,
    tokens::Default,
    tokens::Div,
    tokens::DivEq,
    tokens::Do,
    tokens::Dot,
    tokens::Dot2,
    tokens::Dot3,
    tokens::DotDotEq,
    tokens::Dyn,
    tokens::Else,
    tokens::Enum,
    tokens::Eq,
    tokens::EqEq,
    tokens::Extern,
    tokens::Fn,
    tokens::For,
    tokens::Ge,
    tokens::Group,
    tokens::Gt,
    tokens::If,
    tokens::Impl,
    tokens::In,
    tokens::LArrow,
    tokens::Le,
    tokens::Let,
    tokens::Loop,
    tokens::Lt,
    tokens::Macro,
    tokens::Match,
    tokens::Mod,
    tokens::Move,
    tokens::MulEq,
    tokens::Mut,
    tokens::Ne,
    tokens::Or,
    tokens::OrEq,
    tokens::OrOr,
    tokens::Paren,
    tokens::Pound,
    tokens::Pub,
    tokens::Question,
    tokens::RArrow,
    tokens::Ref,
    tokens::Rem,
    tokens::RemEq,
    tokens::Return,
    tokens::Rocket,
    tokens::Self_,
    tokens::Semi,
    tokens::Shl,
    tokens::ShlEq,
    tokens::Shr,
    tokens::ShrEq,
    tokens::Star,
    tokens::Static,
    tokens::Struct,
    tokens::Sub,
    tokens::SubEq,
    tokens::Super,
    tokens::Trait,
    tokens::Type,
    tokens::Underscore,
    tokens::Union,
    tokens::Unsafe,
    tokens::Use,
    tokens::Where,
    tokens::While,
    tokens::Yield
);
