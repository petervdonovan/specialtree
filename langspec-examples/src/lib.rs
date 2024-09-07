use langspec::{Arity, Domain, Function, Language, SortId};
use lexpr::sexp;

#[test]
pub fn test() {
    let fib = Language {
        functions: vec![Function {
            name: "fib".to_string(),
            domain: Domain(vec![(SortId(0), Arity::N(2))].into_boxed_slice()),
            codomain: SortId(0),
        }]
        .into(),
        sort_names: vec!["number".to_string()].into(),
    };
    let s = serde_lexpr::to_value(&fib).unwrap();
    println!("{}", lexpr::print::to_string(&s).unwrap());
    let fib2 = sexp!(
            (
                (functions
                    ((name . "fib") (domain #(0 (N . 2))) (codomain . 0)))
                (sort_names)));
    let fib2: Language = serde_lexpr::from_value(&fib2).unwrap();
    println!("{}", serde_lexpr::to_string(&fib2).unwrap());
    assert_eq!(fib, fib2);
}
