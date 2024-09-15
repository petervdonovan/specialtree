#[cfg(test)]
mod tests {
    use langspec::*;
    use lexpr::sexp;

    #[test]
    fn fib() {
        let fib = sexp!(
            (
             (name . ((name . "fib") (alias . ())))
             (products
              ((name . ((name . "fib") (alias . ("f"))))
               (sorts
                (Algebraic Sum . 0)
                (Algebraic Sum . 0))))
             (sums
              ((name . ((name . "Nat") (alias . ("ℕ"))))
               (sorts
                NatLiteral
                (Algebraic Product . 0)))))
        );
        let fib: Language = serde_lexpr::from_value(&fib).unwrap();
        let expected = expect_test::expect![[r#"
            Language {
                name: Name {
                    name: "fib",
                    alias: None,
                },
                products: {
                    ProductId(
                        0,
                    ): Product {
                        name: Name {
                            name: "fib",
                            alias: Some(
                                "f",
                            ),
                        },
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
                        name: Name {
                            name: "Nat",
                            alias: Some(
                                "ℕ",
                            ),
                        },
                        sorts: [
                            NatLiteral,
                            Algebraic(
                                Product(
                                    ProductId(
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
