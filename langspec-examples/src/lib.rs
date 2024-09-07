#[cfg(test)]
mod tests {
    use langspec::*;
    use lexpr::sexp;

    #[test]
    fn fib2() {
        let fib = sexp!(
            (
             (functions
              ((name . "fib")
               (domain Algebraic Product . 0)))
             (products
              ((name . "Nat2")
               (sorts
                (Algebraic Sum . 0)
                (Algebraic Sum . 0))))
             (sums
              ((name . "Nat")
               (sorts
                NatLiteral
                (Algebraic Atom . 0)))))
        );
        let fib: Language = serde_lexpr::from_value(&fib).unwrap();
        let expected = expect_test::expect![[r#"
            Language {
                functions: {
                    FunctionId(
                        0,
                    ): Function {
                        name: "fib",
                        domain: Algebraic(
                            Product(
                                ProductId(
                                    0,
                                ),
                            ),
                        ),
                    },
                },
                products: {
                    ProductId(
                        0,
                    ): Product {
                        name: "Nat2",
                        sorts: [
                            Algebraic(
                                Sum(
                                    SumId(
                                        0,
                                    ),
                                ),
                            ),
                            Algebraic(
                                Sum(
                                    SumId(
                                        0,
                                    ),
                                ),
                            ),
                        ],
                    },
                },
                sums: {
                    SumId(
                        0,
                    ): Sum {
                        name: "Nat",
                        sorts: [
                            NatLiteral,
                            Algebraic(
                                Atom(
                                    FunctionId(
                                        0,
                                    ),
                                ),
                            ),
                        ],
                    },
                },
            }"#]];
        expected.assert_eq(&format!("{:#?}", fib));
    }
}
