#[test]
fn test() {
    {
        println!("test Nat");
        let (mut heap, nat) = fib_parse::parse_fib::nat("3");
        println!(
            "unparse: {}",
            unparse_adt::unparse::<fib_parse::term_specialized_cst::Heap, fib::words_mod_fib::L, _>(
                &mut heap, &nat
            )
        )
    }
    {
        println!("test Nat");
        fib_parse::parse_fib::nat("f 3");
    }
    {
        println!("test Sum");
        fib_parse::parse_fib::sum("sum { 3 }");
    }
    {
        println!("test F");
        fib_parse::parse_fib::f("f 3");
    }
    {
        println!("test Plus");
        fib_parse::parse_fib::plus("plus left_operand 3 right_operand 4");
    }
    {
        println!("test Nat");
        fib_parse::parse_fib::nat("sum { f 3, f plus left_operand f 1 right_operand 4 }");
    }
}
