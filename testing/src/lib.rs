extern crate proc_macro;

use pmutil::prelude::*;

#[proc_macro]
pub fn fn_like(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    q!(Vars {}, {
        fn output_fn_like() -> &'static str {
            "foo"
        }
    })
    .into()
}

#[proc_macro]
pub fn omit_vars(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    q!({
        fn output_omit_vars() -> &'static str {
            "foo"
        }
    })
    .into()
}

#[proc_macro]
pub fn expanded_omit_vars(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    Quote::new_call_site()
        .quote_with(smart_quote!({
            fn output_expanded_omit_vars() -> &'static str {
                "foo"
            }
        }))
        .into()
}

#[proc_macro]
pub fn expanded_fn_like(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    Quote::new_call_site()
        .quote_with(smart_quote!({
            fn output_expanded_fn_like() -> &'static str {
                "foo"
            }
        }))
        .into()
}

#[proc_macro]
pub fn vars_1(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    Quote::new_call_site()
        .quote_with(smart_quote!(Vars { a: "foo" }, {
            fn output_vars_1() -> &'static str {
                a
            }
        }))
        .into()
}
