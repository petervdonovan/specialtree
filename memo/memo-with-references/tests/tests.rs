#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/0-parse-fn.rs");
    t.compile_fail("tests/1-not-fn.rs");
    t.compile_fail("tests/2-explicit-lifetime-required.rs");
    t.compile_fail("tests/3-expected-reference-type.rs");
    t.compile_fail("tests/4-fn-parameter-not-found.rs");
    t.compile_fail("tests/5-expected-token-indicating-cache-arg.rs");
    t.compile_fail("tests/6-expected-just-single-ident.rs");
    t.compile_fail("tests/7-expected-an-ident.rs");
    t.compile_fail("tests/8-cache-param-cannot-be-self.rs");
    t.compile_fail("tests/9-no-mutable-references.rs");
    t.compile_fail("tests/10-no-mutable-self.rs");
    t.pass("tests/11-multiple-same-lifetime.rs");
}
