#[cfg(test)]
mod tests {
    use langspec::*;
    use lexpr::sexp;

    #[test]
    fn fib() {
        let fib = sexp!(
            (
             (products
              ((name . "fib")
               (sorts
                (Algebraic Sum . 0)
                (Algebraic Sum . 0))))
             (sums
              ((name . "Nat")
               (sorts
                NatLiteral
                (Algebraic Product . 0)))))
        );
        let fib: Language = serde_lexpr::from_value(&fib).unwrap();
        let expected = expect_test::expect![[r#"
            Language {
                products: {
                    ProductId(
                        0,
                    ): Product {
                        name: "fib",
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
