#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    // Compile-fail expectation cases
    t.compile_fail("tests/fail-no-mutable-references.rs");
    t.compile_fail("tests/fail-no-mutable-self.rs");
    t.compile_fail("tests/fail-not-fn.rs");
    t.compile_fail("tests/fail-not-lifetime.rs");
    t.compile_fail("tests/fail-cloneable-must-implement-clone.rs");
    t.compile_fail("tests/fail-empty-attribute.rs");
    t.compile_fail("tests/fail-multiple-attribute-args.rs");
    t.compile_fail("tests/fail-unit-return-type.rs");
    t.compile_fail("tests/fail-cloneable-insufficient-lifetime.rs");

    // Pass expectation cases
    t.pass("tests/pass-minimal-working-example.rs");
    t.pass("tests/pass-working-mixed-lifetimes.rs");
    t.pass("tests/pass-const-generics.rs");
    t.pass("tests/pass-collision-edge-cases.rs");
    t.pass("tests/pass-non-static-generic.rs");
}
