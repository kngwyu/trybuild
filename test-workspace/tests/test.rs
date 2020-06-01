#[test]
fn test() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/compile-fail-0.rs");
}
