use langspec::humanreadable::LangSpecHuman;

pub fn fib() -> LangSpecHuman {
    serde_yml::from_str(
        r#"
    name:
        name: fiblang
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
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        let lsh = fib();
        let expected = expect_test::expect![[r#"
            name:
              name: fiblang
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
