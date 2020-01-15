//! Span-aware quasi quotting built on top of `quote` crate.
//!

mod buffer;
pub use self::buffer::{Location, Quote};

#[macro_export]
#[doc(hidden)]
macro_rules! quoter_location {
    () => {{
        $crate::spanned_quote::Location {
            line: line!(),
            col: column!(),
            file_name: file!(),
        }
    }};
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
        $crate::declare_vars_for_quote!(
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
        $crate::handle_vars_for_quote!(
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
        $crate::handle_vars_for_quote!(
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
        $crate::handle_vars_for_quote!(
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
        $crate::handle_vars_for_quote!(
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
        $tokens.push_group(::proc_macro2::Delimiter::Parenthesis, $crate::__sq_quote_closure! {
            $($inner)*
        });
        $crate::__sq_quote_tokens_to!($tokens, $($rest)*);
    }};


    ($tokens:expr, { $($inner:tt)* }  $($rest:tt)*) => {{
        $tokens.push_group(::proc_macro2::Delimiter::Brace, $crate::__sq_quote_closure! {
            $($inner)*
        });
        $crate::__sq_quote_tokens_to!($tokens, $($rest)*);
    }};

    ($tokens:expr, [ $($inner:tt)* ]  $($rest:tt)*) => {{
        $tokens.push_group(::proc_macro2::Delimiter::Bracket, $crate::__sq_quote_closure! {
            $($inner)*
        });
        $crate::__sq_quote_tokens_to!($tokens, $($rest)*);
    }};

    // If we have to quote one token, check if user declared variable.
    ($tokens:expr, $first:tt $($rest:tt)*) => {
        __sq_push_token_custom!($tokens, $first);

        $crate::__sq_quote_tokens_to!($tokens, $($rest)*);
    };
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
///  tokens are wrapped in block or paren like `{ tokens.. }`/ `( tokens.. )`.
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
    (
        Vars{ $($vars:tt)* },
        {
            $(
                $tokens:tt
            )*
        }
    ) => {{
        |_tokens: &mut $crate::Quote| {
            $crate::handle_vars_for_quote!(@NORMALIZED{}, $($vars)*);

            _tokens.report_loc($crate::quoter_location!());
            $crate::__sq_quote_tokens_to!(_tokens, $($tokens)*);
        }
    }};

    (
        Vars{ $($vars:tt)* },
        (
            $(
                $tokens:tt
            )*
        )
    ) => {{
        |_tokens: &mut $crate::Quote| {
            $crate::handle_vars_for_quote!(@NORMALIZED{}, $($vars)*);

            _tokens.report_loc($crate::quoter_location!());
            $crate::__sq_quote_tokens_to!(_tokens, $($tokens)*);
        }
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __sq_quote_closure {
    ( $($tokens:tt)* ) => {{
        |_tokens: &mut $crate::Quote| {
            _tokens.report_loc($crate::quoter_location!());
            $crate::__sq_quote_tokens_to!(_tokens, $($tokens)*);
        }
    }};
}

/// Shortcut for `Quote::new_call_site().quote_with(smart_quote!( $tokens ))`
#[macro_export]
macro_rules! q {
    ( $($tokens:tt)* ) => {{
        $crate::Quote::new_call_site().quote_with($crate::smart_quote!( $($tokens)* ))
    }};
}
