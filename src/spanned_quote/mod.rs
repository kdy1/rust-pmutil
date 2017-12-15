//! Span-aware quasi quotting built on top of `quote` crate.
//!

mod buffer;
pub use self::buffer::{Location, Quote};

#[macro_export]
#[doc(hidden)]
macro_rules! quoter_location {
    () => {{
        $crate::spanned_quote::Location{
            line: line!(),
            col: column!(),
            file_name: file!(),
        }
    }}
}

// ----- Start of variable handling macros.



/// Usage: __sq_handle_vars! { a, b: expression(), c, };
///
#[doc(hidden)]
#[macro_export]
macro_rules! handle_vars_for_quote {
    (
        @NORMALIZED {
            $(
                $name:ident: $value:expr,
            )*
        },
    ) => {
        declare_vars_for_quote!(
            $($name: $value,)*
        );
    };

    (
        @NORMALIZED {
            $($norm:tt)*
        },
        $name:ident,
        $($rest:tt)*
    ) => {
        handle_vars_for_quote!(
            @NORMALIZED {
                $($norm)*
                $name: $name,
            },
            $($rest)*
        )
    };

    (
        @NORMALIZED {
            $($norm:tt)*
        },
        $name:ident
    ) => {
        handle_vars_for_quote!(
            @NORMALIZED {
                $($norm)*
                $name: $name,
            },
        )
    };

    (
        @NORMALIZED {
            $($norm:tt)*
        },
        $name:ident: $value:expr,
        $($rest:tt)*
    ) => {
        handle_vars_for_quote!(
            @NORMALIZED {
                $($norm)*
                $name: $value,
            },
            $($rest)*
        )
    };

    (
        @NORMALIZED {
            $($norm:tt)*
        },
        $name:ident: $value:expr
    ) => {
        handle_vars_for_quote!(
            @NORMALIZED {
                $($norm)*
                $name: $value,
            },
        )
    };
}


/// This macro handles `Vars`, and creates a new hidden macro used inside quasi-quotting.
#[doc(hidden)]
#[macro_export]
macro_rules! declare_vars_for_quote {
    (
        $(
            $name:ident: $val:expr,
        )*
    ) => {
        $(
            #[allow(non_snake_case)]
            let $name = $val;
        )*

        // This macro quotes only one token at once.
        macro_rules! __sq_push_token_custom {
            $(
                ($tokens:expr, $name) => {
                    $tokens.push_tokens(&$name);
                };
            )*
            // default (stringify + parse)
            ($tokens:expr, $t:tt) => {
                $tokens.push_parsed(stringify!($t));
            };
        }

    };
}




// ----- End of variable handling macros.



/// This macro assumes that `Vars` is already handled.
#[doc(hidden)]
#[macro_export]
macro_rules! __sq_quote_tokens_to {
    // Done.
    ($tokens:expr,) => {{}};

    ($tokens:expr, ( $($inner:tt)* ) $($rest:tt)*) => {{
        $tokens.push_group(::proc_macro2::Delimiter::Parenthesis, __sq_quote_closure! {
            $($inner)*
        });
        __sq_quote_tokens_to!($tokens, $($rest)*);
    }};


    ($tokens:expr, { $($inner:tt)* }  $($rest:tt)*) => {{
        $tokens.push_group(::proc_macro2::Delimiter::Brace, __sq_quote_closure! {
            $($inner)*
        });
        __sq_quote_tokens_to!($tokens, $($rest)*);
    }};

    ($tokens:expr, [ $($inner:tt)* ]  $($rest:tt)*) => {{
        $tokens.push_group(::proc_macro2::Delimiter::Bracket, __sq_quote_closure! {
            $($inner)*
        });
        __sq_quote_tokens_to!($tokens, $($rest)*);
    }};

    // If we have to quote one token, check if user declared variable.
    ($tokens:expr, $first:tt $($rest:tt)*) => {
        __sq_push_token_custom!($tokens, $first);

        __sq_quote_tokens_to!($tokens, $($rest)*);
    };
}




#[doc(hidden)]
#[macro_export]
macro_rules! __sq_quote_closure {
    ( $($tokens:tt)* ) => {{
        |_tokens: &mut $crate::Quote| {
            _tokens.report_loc(quoter_location!());
            __sq_quote_tokens_to!(_tokens, $($tokens)*);
        }
    }};
}


/// ide-friendly quasi quotting.
///
///# Syntax
///## Vars
///Syntax is simillar to field initialization syntax.
///
///```rust,ignore
///Vars {
///  a,
///  b: expr_b,
///  c: fn_c(),
///  d,
///}
/// // is equivalent to
///Vars {
///  a: a,
///  b: expr_b,
///  c: fn_c(),
///  d: d,
///}
///
///```
///
///Note that `Vars{}` is required even if there's no variable.
///
///## Tokens
/// As parsers for syntax highligters implement error recovery,
///  tokens are wrapped in block like `{ tokens.. }`.
/// Note that `rustfmt` works for this tokens,
///  as invocation seems like a function call
///  with `Vars{}` as first argument, and block expression as a second argument.
///
///# Example
///
///```rust,ignore
/// smart_quote!(Vars{
///     OrigTrait: tr.ident,
///     SpecializerTrait: tr.ident.new_ident_with(|tr| format!("{}Specializer", tr)),
/// }, {
///     impl<T> OrigTrait for T where T: SpecializerTrait {
///     }
/// })
///```
///
///# Example (no variable)
///
///```rust,ignore
/// smart_quote!(Vars{}, {
///     yield ();
/// })
///```
#[macro_export]
macro_rules! smart_quote {
    // Make last comma for vars optional
    (
        Vars{ $($vars:tt)* },
        {
            $(
                $tokens:tt
            )*
        }
    ) => {{
        move |_tokens: &mut $crate::Quote| {
            handle_vars_for_quote!(@NORMALIZED{}, $($vars)*);

            _tokens.report_loc(quoter_location!());
            __sq_quote_tokens_to!(_tokens, $($tokens)*);
        }
    }};
}
