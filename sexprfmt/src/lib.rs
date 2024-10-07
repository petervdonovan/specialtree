pub fn fmt_value(v: &lexpr::Value) -> String {
    let mut buf = String::new();
    fn rec(v: &lexpr::Value, indentation: usize, buf: &mut String) {
        match v {
            lexpr::Value::Nil => buf.push_str("nil"),
            lexpr::Value::Bool(b) => buf.push_str(&b.to_string()),
            lexpr::Value::Number(n) => buf.push_str(&n.to_string()),
            lexpr::Value::String(s) => buf.push_str(&format!("\"{}\"", s)),
            lexpr::Value::Symbol(s) => buf.push_str(s),
            lexpr::Value::Keyword(k) => buf.push_str(&format!("#:{}", k)),
            lexpr::Value::Cons(l) => {
                let indentation = indentation + 1;
                let one_line = v.to_string();
                if one_line.len() < 40 {
                    buf.push_str(&one_line);
                    return;
                }
                buf.push('(');
                let mut l = Some(l);
                while let Some(cons) = l {
                    rec(cons.car(), indentation, buf);
                    l = match *cons.cdr() {
                        lexpr::Value::Cons(ref c) => Some(c),
                        lexpr::Value::Nil => None,
                        lexpr::Value::Null => None,
                        ref cdr => {
                            buf.push_str(" . ");
                            rec(cdr, indentation, buf);
                            None
                        }
                    };
                    if l.is_some() {
                        buf.push('\n');
                        buf.push_str(&" ".repeat(indentation));
                    }
                }
                buf.push(')');
            }
            lexpr::Value::Vector(v) => {
                buf.push('[');
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        buf.push(' ');
                    }
                    rec(e, indentation, buf);
                }
                buf.push(']');
            }
            lexpr::Value::Null => buf.push_str("null"),
            lexpr::Value::Char(c) => buf.push_str(&format!("'{}'", c)),
            lexpr::Value::Bytes(b) => buf.push_str(&format!(
                "[{}]",
                b.iter()
                    .map(|it| it.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            )),
        };
    }
    rec(v, 0, &mut buf);
    buf
}

#[cfg(test)]
mod tests {
    use lexpr::sexp;

    use super::*;

    #[test]
    fn test_fmt_value() {
        let v = sexp!(
            (
             (name (name . "fib") (alias))
             (products
              ((name . ((name . "fib") (alias "f")))
               (sorts
                (Algebraic Sum . 0)
                (Algebraic Sum . 0))))
             (sums
              ((name (name . "Nat") (alias "ℕ"))
               (sorts
                NatLiteral
                (Algebraic Product . 0)))))
        );
        println!("{}", lexpr::from_str(&fmt_value(&v)).unwrap());
        assert_eq!(lexpr::from_str(&fmt_value(&v)).unwrap(), v);
        let expect = expect_test::expect![[r#"
            ((name (name . "fib") (alias))
             (products
              ((name (name . "fib") (alias "f"))
               (sorts
                (Algebraic Sum . 0)
                (Algebraic Sum . 0))))
             (sums
              ((name (name . "Nat") (alias "ℕ"))
               (sorts
                NatLiteral
                (Algebraic Product . 0)))))"#]];
        expect.assert_eq(&fmt_value(&v));
    }

    #[test]
    fn test_fmt_value_basic() {
        assert_eq!(fmt_value(&lexpr::Value::Nil), "nil");
        assert_eq!(fmt_value(&lexpr::Value::Bool(true)), "true");
        assert_eq!(fmt_value(&lexpr::Value::Bool(false)), "false");
        assert_eq!(fmt_value(&lexpr::Value::Number(42.into())), "42");
        assert_eq!(fmt_value(&lexpr::Value::String("foo".into())), "\"foo\"");
        assert_eq!(fmt_value(&lexpr::Value::Symbol("foo".into())), "foo");
        assert_eq!(fmt_value(&lexpr::Value::Keyword("foo".into())), "#:foo");
        assert_eq!(
            fmt_value(&lexpr::Value::Cons(lexpr::Cons::new(
                lexpr::Value::Symbol("foo".into()),
                lexpr::Value::Null
            ))),
            "(foo)"
        );
        assert_eq!(
            fmt_value(&lexpr::Value::Cons(lexpr::Cons::new(
                lexpr::Value::Symbol("foo".into()),
                lexpr::Value::Cons(lexpr::Cons::new(
                    lexpr::Value::Symbol("bar".into()),
                    lexpr::Value::Null
                ))
            ))),
            "(foo bar)"
        );
        assert_eq!(
            fmt_value(&lexpr::Value::Cons(lexpr::Cons::new(
                lexpr::Value::Symbol("foo".into()),
                lexpr::Value::Cons(lexpr::Cons::new(
                    lexpr::Value::Symbol("bar".into()),
                    lexpr::Value::Symbol("baz".into())
                ))
            ))),
            "(foo bar . baz)"
        );
        assert_eq!(
            fmt_value(&lexpr::Value::Vector(
                vec![
                    lexpr::Value::Symbol("foo".into()),
                    lexpr::Value::Symbol("bar".into())
                ]
                .into_boxed_slice()
            )),
            "[foo bar]"
        );
        assert_eq!(fmt_value(&lexpr::Value::Null), "null");
        assert_eq!(fmt_value(&lexpr::Value::Char('a')), "'a'");
        assert_eq!(
            fmt_value(&lexpr::Value::Bytes(vec![1, 2, 3].into_boxed_slice())),
            "[1 2 3]"
        );
    }
}
