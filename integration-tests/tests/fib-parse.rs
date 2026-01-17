use fib::l_words_mod_r_fib::{L, sorts};

#[test]
fn test() {
    {
        println!("test Nat");
        let (heap, v) = fib_parse::parse_fib::nat("3");
        println!(
            "  unparse: {}",
            unparse_adt::unparse::<sorts::Nat, L, _, _>(&heap, &v)
        );
    }
    {
        println!("test Nat");
        let (heap, v) = fib_parse::parse_fib::nat("f 3");
        println!(
            "  unparse: {}",
            unparse_adt::unparse::<sorts::Nat, L, _, _>(&heap, &v)
        )
    }
    {
        println!("test Sum");
        let (heap, v) = fib_parse::parse_fib::sum("sum { 3 }");
        println!(
            "  unparse: {}",
            unparse_adt::unparse::<sorts::Sum, L, _, _>(&heap, &v)
        )
    }
    {
        println!("test F");
        let (heap, v) = fib_parse::parse_fib::f("f 3");
        println!(
            "  unparse: {}",
            unparse_adt::unparse::<sorts::F, L, _, _>(&heap, &v)
        )
    }
    {
        println!("test Plus");
        let (heap, v) = fib_parse::parse_fib::plus("plus left_operand 3 right_operand 4");
        println!(
            "  unparse: {}",
            unparse_adt::unparse::<sorts::Plus, L, _, _>(&heap, &v)
        )
    }
    {
        println!("test Nat");
        let (heap, v) =
            fib_parse::parse_fib::nat("sum { f 3, f plus left_operand f 1 right_operand 4 }");
        println!(
            "  unparse: {}",
            unparse_adt::unparse::<sorts::Nat, L, _, _>(&heap, &v)
        )
    }
}
