use langspec::humanreadable::LangSpecHuman;

pub fn pattern_ext() -> LangSpecHuman {
    serde_yml::from_str(
        r#"
name:
    human: pattern
    camel: Pattern
    snake: pattern
products:
- name:
    human: e
    camel: Expr
    snake: expr
  sorts:
  - !NatLiteral
- name:
    human: ...e
    camel: Exprs
    snake: exprs
  sorts:
  - !NatLiteral
sums:
    "#,
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern() {
        let lsh = pattern_ext();
        let expected = expect_test::expect![[r#"
            name:
              human: pattern
              camel: Pattern
              snake: pattern
            products:
            - name:
                human: e
                camel: Expr
                snake: expr
              sorts:
              - NatLiteral
            - name:
                human: '...e'
                camel: Exprs
                snake: exprs
              sorts:
              - NatLiteral
            sums: []
        "#]];
        expected.assert_eq(&serde_yml::to_string(&lsh).unwrap());
    }
}
