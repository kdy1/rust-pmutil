use testing::*;

vars_1!();

#[test]
fn test_vars_1() {
    assert_eq!(output_vars_1(), "foo");
}
