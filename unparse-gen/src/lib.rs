use extension_autobox::autobox;
use langspec::langspec::LangSpec;
use langspec_gen_util::LsGen;
use syn::parse_quote;

pub fn formatted<L: LangSpec>(l: &L) -> String {
    let lg = LsGen::from(l);
    let bps = BasePaths {
        unparse: parse_quote! { crate::unparse },
        data_structure: parse_quote! { crate::data_structure },
        term_trait: parse_quote! { crate::term_trait },
        words: parse_quote! { crate::term_trait::words },
        pattern_match_strategy: parse_quote! { crate::pattern_match_strategy },
    };
    let m = generate(&bps, &lg);
    let (cst_impls, bridge) = {
        let arena = bumpalo::Bump::new();
        let autoboxed = autobox(l);
        let cst = cst(&arena, &autoboxed);
        let ext_lg = LsGen::from(&cst);
        (
            gen_impls(
                &syn::parse_quote! { crate::cst },
                &ext_lg,
                &[cst.name().clone(), l.name().clone()],
            ),
            {
                term_bridge_gen::generate(
                    &ext_lg,
                    lg.bak().name(),
                    &term_bridge_gen::BasePaths {
                        ext_data_structure: syn::parse_quote!(crate::cst::data_structure),
                        og_term_trait: syn::parse_quote!(crate::term_trait),
                    },
                )
            },
        )
    };
    let pmsp = term_pattern_match_strategy_provider_gen::generate(
        &term_pattern_match_strategy_provider_gen::BasePaths {
            term_trait: syn::parse_quote!(crate::term_trait),
            data_structure: syn::parse_quote!(crate::data_structure),
            words: syn::parse_quote!(crate::term_trait::words),
            strategy_provider: syn::parse_quote!(crate::pattern_match_strategy),
        },
        &lg,
    );
    let tt = term_trait_gen::generate(&syn::parse_quote!(crate::term_trait), lg.bak());
    let byline = byline!();
    let f: syn::File = syn::parse_quote! {
        #byline
        #[test]
        fn test() {
            {
                println!("test Sum");
                crate::parse::sum("sum { 3 }");
            }
            {
                println!("test Nat");
                crate::parse::nat("3");
            }
            {
                println!("test Nat");
                crate::parse::nat("f 3");
            }
            {
                println!("test Sum");
                crate::parse::sum("sum { 3 }");
            }
            {
                println!("test F");
                crate::parse::f("f 3");
            }
            {
                println!("test Plus");
                crate::parse::plus("plus left_operand 3 right_operand 4");
            }
            {
                println!("test Nat");
                crate::parse::nat("sum { f 3, f plus left_operand f 1 right_operand 4 }");
            }
        }
        #m
        #byline
        pub mod cst {
            #cst_impls
        }
        pub mod bridge {
            #bridge
        }
        #pmsp
        #tt
    };
    prettyplease::unparse(&syn_insert_use::insert_use(f))
}
