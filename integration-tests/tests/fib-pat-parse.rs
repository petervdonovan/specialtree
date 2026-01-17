use fib::l_words_mod_r_fib::{L as LSub, sorts};
use fib_pat::l_words_mod_r_l_file_l_pattern_fib_r_r::{L, sorts as supsorts};
use fib_pat_parse::l_term_specialized_r_l_autoboxed_l_cst_l_file_l_pattern_fib_r_r_r_r::Heap;
use file_tmf::File;
use langspec::{flat::LangSpecFlat, langspec::TerminalLangSpec as _};

#[test]
fn test() {
    let ls = LangSpecFlat::canonical_from(&langspec_examples::fib());
    {
        println!("test Nat");
        let (heap, v) = fib_pat_parse::parse_l_file_l_pattern_fib_r_r::nat("3");
        println!(
            "  unparse: {}",
            unparse_adt::unparse::<supsorts::Nat, L, _, _>(&heap, &v)
        );
        println!(
            "  pattern: {:?}",
            pattern_dyn::to_pattern::<supsorts::Nat, L, _, _, LSub, _>(&heap, &v, &ls).unwrap()
        );
    }
    {
        println!("test Nat");
        let (heap, v) = fib_pat_parse::parse_l_file_l_pattern_fib_r_r::nat("f 3");
        println!(
            "  unparse: {}",
            unparse_adt::unparse::<supsorts::Nat, L, _, _>(&heap, &v)
        );
        println!(
            "  pattern: {:?}",
            pattern_dyn::to_pattern::<supsorts::Nat, L, _, _, LSub, _>(&heap, &v, &ls).unwrap()
        );
    }
    {
        println!("test Sum");
        let (heap, v) = fib_pat_parse::parse_l_file_l_pattern_fib_r_r::sum("sum { $k }");
        println!(
            "  unparse: {}",
            unparse_adt::unparse::<supsorts::Sum, L, _, _>(&heap, &v)
        );
        println!(
            "  pattern: {:?}",
            pattern_dyn::to_pattern::<supsorts::Sum, L, _, _, LSub, _>(&heap, &v, &ls).unwrap()
        );
    }
    {
        println!("test F");
        let (heap, v) = fib_pat_parse::parse_l_file_l_pattern_fib_r_r::f("f _");
        println!(
            "  unparse: {}",
            unparse_adt::unparse::<supsorts::F, L, _, _>(&heap, &v)
        );
        println!(
            "  pattern: {:?}",
            pattern_dyn::to_pattern::<supsorts::F, L, _, _, LSub, _>(&heap, &v, &ls).unwrap()
        );
    }
    {
        println!("test Plus");
        let (heap, v) = fib_pat_parse::parse_l_file_l_pattern_fib_r_r::plus(
            "plus left_operand 3 right_operand 4",
        );
        println!(
            "  unparse: {}",
            unparse_adt::unparse::<supsorts::Plus, L, _, _>(&heap, &v)
        );
        println!(
            "  pattern: {:?}",
            pattern_dyn::to_pattern::<supsorts::Plus, L, _, _, LSub, _>(&heap, &v, &ls).unwrap()
        );
    }
    {
        println!("test Nat");
        let (heap, v) = fib_pat_parse::parse_l_file_l_pattern_fib_r_r::nat(
            "sum { f 3, f plus left_operand f $t right_operand 4, ...z }",
        );
        println!(
            "  unparse: {}",
            unparse_adt::unparse::<supsorts::Nat, L, _, _>(&heap, &v)
        );
        println!(
            "  pattern: {:?}",
            pattern_dyn::to_pattern::<supsorts::Nat, L, _, _, LSub, _>(&heap, &v, &ls).unwrap()
        );
    }
    // {
    //     println!("test File");
    //     let (heap, v) = file_tmf::parse::file::<Heap, L, _, _>(
    //         r#"
    //         @a = f plus left_operand $a right_operand $b
    //         @b = sum { 4 , 5 , ...c }
    //         @c = f 9
    //     "#,
    //     );
    //     println!(
    //         "  unparse: {}",
    //         file_tmf::unparse::file::<L, _, _, _, _>(&heap, &v)
    //     );
    //     for item in file_tmf::items(&heap, &v) {
    //         println!(
    //             "pattern from file: {:#?}",
    //             pattern_dyn::to_pattern_skip::<supsorts::FileItem, L, LSub, _, _, _>(
    //                 &heap, item, &ls
    //             )
    //             .unwrap()
    //         );
    //     }
    // }
}
