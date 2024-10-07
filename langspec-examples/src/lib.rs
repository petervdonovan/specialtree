#[cfg(test)]
mod tests {
    use humanreadable::LangSpecHuman;
    use langspec::*;

    #[test]
    fn yml() {
        let lsh: LangSpecHuman = serde_yml::from_str(
            r#"
            name:
                name: fib
            products:
            - name:
                name: fib
                alias: f
              sorts:
              - !Algebraic Nat
              - !Algebraic Nat
            sums:
            - name:
                name: Nat
                alias: ℕ
              sorts:
              - !NatLiteral
              - !Algebraic
                fib
        "#,
        )
        .unwrap();
        let expected = expect_test::expect![[r#"
            name:
              name: fib
              alias: null
            products:
            - name:
                name: fib
                alias: f
              sorts:
              - !Algebraic Nat
              - !Algebraic Nat
            sums:
            - name:
                name: Nat
                alias: ℕ
              sorts:
              - NatLiteral
              - !Algebraic fib
        "#]];
        expected.assert_eq(&serde_yml::to_string(&lsh).unwrap());
    }
}
