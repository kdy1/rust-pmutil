use testing::*;

fn_like!();

#[test]
fn test_fn_like() {
    assert_eq!(output_fn_like(), "foo");
}

omit_vars!();

#[test]
fn test_omit_vars() {
    assert_eq!(output_omit_vars(), "foo");
}

expanded_omit_vars!();

#[test]
fn test_expanded_omit_vars() {
    assert_eq!(output_expanded_omit_vars(), "foo");
}

expanded_fn_like!();

#[test]
fn test_expanded_fn_like() {
    assert_eq!(output_expanded_fn_like(), "foo");
}
