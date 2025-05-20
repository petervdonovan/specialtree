use fib_pat::words_mod_file_pattern_fib::L;
use fib_pat_parse::term_specialized_cst_autoboxed_file_pattern_fib::Heap;

#[test]
fn test() {
    {
        println!("test Nat");
        let (heap, v) = fib_pat_parse::parse_file_pattern_fib::nat("3");
        println!("unparse: {}", unparse_adt::unparse::<L, _, _>(&heap, &v));
        println!(
            "pattern: {:?}",
            pattern_dyn::to_pattern::<L, _, _>(&heap, &v).unwrap()
        );
    }
    {
        println!("test Nat");
        let (heap, v) = fib_pat_parse::parse_file_pattern_fib::nat("f 3");
        println!("unparse: {}", unparse_adt::unparse::<L, _, _>(&heap, &v));
        println!(
            "pattern: {:?}",
            pattern_dyn::to_pattern::<L, _, _>(&heap, &v).unwrap()
        );
    }
    {
        println!("test Sum");
        let (heap, v) = fib_pat_parse::parse_file_pattern_fib::sum("sum { $k }");
        println!("unparse: {}", unparse_adt::unparse::<L, _, _>(&heap, &v));
        println!(
            "pattern: {:?}",
            pattern_dyn::to_pattern::<L, _, _>(&heap, &v).unwrap()
        );
    }
    {
        println!("test F");
        let (heap, v) = fib_pat_parse::parse_file_pattern_fib::f("f _");
        println!("unparse: {}", unparse_adt::unparse::<L, _, _>(&heap, &v));
        println!(
            "pattern: {:?}",
            pattern_dyn::to_pattern::<L, _, _>(&heap, &v).unwrap()
        );
    }
    {
        println!("test Plus");
        let (heap, v) =
            fib_pat_parse::parse_file_pattern_fib::plus("plus left_operand 3 right_operand 4");
        println!("unparse: {}", unparse_adt::unparse::<L, _, _>(&heap, &v));
        println!(
            "pattern: {:?}",
            pattern_dyn::to_pattern::<L, _, _>(&heap, &v).unwrap()
        );
    }
    {
        println!("test Nat");
        let (heap, v) = fib_pat_parse::parse_file_pattern_fib::nat(
            "sum { f 3, f plus left_operand f $t right_operand 4, ...z }",
        );
        println!("unparse: {}", unparse_adt::unparse::<L, _, _>(&heap, &v));
        println!(
            "pattern: {:?}",
            pattern_dyn::to_pattern::<L, _, _>(&heap, &v).unwrap()
        );
    }
    {
        println!("test File");
        let (heap, v) = file_tmf::parse::file::<Heap, L, _>(
            r#"
            f plus left_operand $a right_operand $b
            sum { 4 , 5 , ...c }
            f 9
        "#,
        );
        println!("unparse: {}", file_tmf::unparse::file(&heap, &v));
        // use ccf::DirectlyCanonicallyConstructibleFrom;
        // let (f, ()): (file_tmf::File<_, _>, ()) = v.deconstruct(&heap);
        for item in file_tmf::items(&heap, &v) {
            println!(
                "pattern: {:?}",
                pattern_dyn::to_pattern::<L, _, _>(&heap, item).unwrap()
            );
        }
    }
}
