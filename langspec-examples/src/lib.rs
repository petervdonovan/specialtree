use langspec::humanreadable::LangSpecHuman;

pub fn fib() -> LangSpecHuman {
    serde_yml::from_str(
        r#"
    name:
        human: fib
        camel: Fib
        snake: fib
    products:
    - name:
        human: f
        camel: F
        snake: f
      sorts:
      - !Algebraic ℕ
    - name:
        human: +
        camel: Plus
        snake: plus
      sorts:
      - !Algebraic ℕ
      - !Algebraic ℕ
    sums:
    - name:
        human: ℕ
        camel: Nat
        snake: nat
      sorts:
      - !NatLiteral
      - !Algebraic
        f
      - !Algebraic
        +
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
              human: fib
              camel: Fib
              snake: fib
            products:
            - name:
                human: f
                camel: F
                snake: f
              sorts:
              - !Algebraic ℕ
            - name:
                human: '+'
                camel: Plus
                snake: plus
              sorts:
              - !Algebraic ℕ
              - !Algebraic ℕ
            sums:
            - name:
                human: ℕ
                camel: Nat
                snake: nat
              sorts:
              - NatLiteral
              - !Algebraic f
              - !Algebraic '+'
        "#]];
        expected.assert_eq(&serde_yml::to_string(&lsh).unwrap());
    }
}
