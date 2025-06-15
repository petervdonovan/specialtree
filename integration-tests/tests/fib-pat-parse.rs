use fib::words_mod_fib::{L as LSub, sorts};
use fib_pat::words_mod_file_pattern_fib::L;
use fib_pat_parse::term_specialized_cst_autoboxed_file_pattern_fib::Heap;
use langspec::{flat::LangSpecFlat, langspec::TerminalLangSpec as _};

#[test]
fn test() {
    let ls = LangSpecFlat::canonical_from(&langspec_examples::fib());
    {
        println!("test Nat");
        let (heap, v) = fib_pat_parse::parse_file_pattern_fib::nat("3");
        println!(
            "unparse: {}",
            unparse_adt::unparse::<sorts::Nat, L, _, _>(&heap, &v)
        );
        println!(
            "pattern: {:?}",
            pattern_dyn::to_pattern::<sorts::Nat, L, _, _, LSub, _>(&heap, &v, &ls).unwrap()
        );
    }
    {
        println!("test Nat");
        let (heap, v) = fib_pat_parse::parse_file_pattern_fib::nat("f 3");
        println!(
            "unparse: {}",
            unparse_adt::unparse::<sorts::Nat, L, _, _>(&heap, &v)
        );
        println!(
            "pattern: {:?}",
            pattern_dyn::to_pattern::<sorts::Nat, L, _, _, LSub, _>(&heap, &v, &ls).unwrap()
        );
    }
    {
        println!("test Sum");
        let (heap, v) = fib_pat_parse::parse_file_pattern_fib::sum("sum { $k }");
        println!(
            "unparse: {}",
            unparse_adt::unparse::<sorts::Nat, L, _, _>(&heap, &v)
        );
        println!(
            "pattern: {:?}",
            pattern_dyn::to_pattern::<sorts::Sum, L, _, _, LSub, _>(&heap, &v, &ls).unwrap()
        );
    }
    {
        println!("test F");
        let (heap, v) = fib_pat_parse::parse_file_pattern_fib::f("f _");
        println!(
            "unparse: {}",
            unparse_adt::unparse::<sorts::F, L, _, _>(&heap, &v)
        );
        println!(
            "pattern: {:?}",
            pattern_dyn::to_pattern::<sorts::F, L, _, _, LSub, _>(&heap, &v, &ls).unwrap()
        );
    }
    {
        println!("test Plus");
        let (heap, v) =
            fib_pat_parse::parse_file_pattern_fib::plus("plus left_operand 3 right_operand 4");
        println!(
            "unparse: {}",
            unparse_adt::unparse::<sorts::Plus, L, _, _>(&heap, &v)
        );
        println!(
            "pattern: {:?}",
            pattern_dyn::to_pattern::<sorts::Plus, L, _, _, LSub, _>(&heap, &v, &ls).unwrap()
        );
    }
    {
        println!("test Nat");
        let (heap, v) = fib_pat_parse::parse_file_pattern_fib::nat(
            "sum { f 3, f plus left_operand f $t right_operand 4, ...z }",
        );
        println!(
            "unparse: {}",
            unparse_adt::unparse::<sorts::Nat, L, _, _>(&heap, &v)
        );
        println!(
            "pattern: {:?}",
            pattern_dyn::to_pattern::<sorts::Nat, L, _, _, LSub, _>(&heap, &v, &ls).unwrap()
        );
    }
    {
        println!("test File");
        let (heap, v) = file_tmf::parse::file::<Heap, L, _>(
            r#"
            @a = f plus left_operand $a right_operand $b
            @b = sum { 4 , 5 , ...c }
            @c = f 9
        "#,
        );
        println!(
            "unparse: {}",
            file_tmf::unparse::file::<L, _, _, _, _>(&heap, &v)
        );
        for item in file_tmf::items(&heap, &v) {
            println!(
                "pattern from file: {:#?}",
                pattern_dyn::to_pattern_skip::<L, LSub, _, _, _>(&heap, item, &ls).unwrap()
            );
        }
    }
}
