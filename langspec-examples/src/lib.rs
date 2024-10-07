#[cfg(test)]
mod tests {
    use humanreadable::LangSpecHuman;
    use langspec::*;
    use lexpr::sexp;
    use typed_index_collections::TiVec;

    #[test]
    fn test0() {
        let ls = LangSpec {
            name: Name {
                name: "fib".to_string(),
                alias: None,
            },
            products: TiVec::new(),
            sums: TiVec::new(),
        };
        let lsh: LangSpecHuman = (&ls).into();
        let expected = expect_test::expect![[r#"
            name:
              name: fib
              alias: null
            products: []
            sums: []
        "#]];
        expected.assert_eq(&serde_yml::to_string(&lsh).unwrap());
    }

    #[test]
    fn yml() {
        let lsh: LangSpecHuman = serde_yml::from_str(r#"
name:
    name: fib
products:
- name:
    name: fib
    alias: f
  sorts:
  - !Algebraic
    name: Nat
  - !Algebraic
    name: Nat
sums:
- name:
    name: Nat
    alias: ℕ
  sorts:
  - !NatLiteral
  - !Algebraic
    name: fib
        "#).unwrap();
    }

    #[test]
    fn fibh() {
        let fibh = match knuffel::parse::<LangSpecHuman>("example.kdl", r#"
name "fib"
products {
    name "fib" "f"
    algebraic "Nat"
    algebraic "Nat"
}
sums {
    name "Nat" "ℕ"
    nat-literal
    algebraic "fib"
}
"#) {
            Ok(fibh) => fibh,
            Err(e) => {
                panic!("{:?}", miette::Report::new(e))
            },
        };
        let expected = expect_test::expect![[r#"
            name:
              name: fib
              alias: null
            products:
            - name:
                name: fib
                alias: f
              sorts:
              - !Algebraic
                name: Nat
              - !Algebraic
                name: Nat
            sums:
            - name:
                name: Nat
                alias: ℕ
              sorts:
              - NatLiteral
              - !Algebraic
                name: fib
        "#]];
        expected.assert_eq(&serde_yml::to_string(&fibh).unwrap());
    }

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
        let fib: LangSpec = serde_lexpr::from_value(&fib).unwrap();
        let expected = expect_test::expect![[r#"
            LangSpec {
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
