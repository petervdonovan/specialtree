use extension_autobox::autobox;
use extension_everywhere_alternative::everywhere_alternative;
use extension_everywhere_maybemore::everywhere_maybemore;
use langspec::{
    langspec::{LangSpec, MappedType, Name, SortId},
    tymetafunc::Transparency,
};
use langspec_gen_util::{LsGen, TyGenData, byline, proc_macro2};
use syn::parse_quote;

pub struct BasePaths {
    pub data_structure: syn::Path,
    pub term_trait: syn::Path,
    pub words: syn::Path,
    pub pattern_match_strategy: syn::Path,
    pub cst_data_structure: syn::Path,
    pub cst_term_trait: syn::Path,
    pub cst_words: syn::Path,
}

pub fn generate<L: LangSpec>(bps: &BasePaths, lg: &LsGen<L>) -> syn::ItemMod {
    let byline = byline!();
    let arena = bumpalo::Bump::new();
    let cst = cst(&arena, lg.bak());
    let lg_cst = LsGen::from(&cst);
    let cst_tgds = lg_cst.ty_gen_datas(None).collect::<Vec<_>>();
    let parsells = lg.ty_gen_datas(None).filter_map(|tgd| {
        tgd.id.as_ref().map(|_| {
            generate_parsell(
                bps,
                cst_tgds
                    .iter()
                    .find(|cst_tgd| cst_tgd.snake_ident == tgd.snake_ident)
                    .unwrap(),
            )
        })
    });
    let parses = lg.ty_gen_datas(None).filter_map(|tgd| {
        tgd.id.as_ref().map(|_| {
            generate_parse(
                bps,
                cst_tgds
                    .iter()
                    .find(|cst_tgd| cst_tgd.snake_ident == tgd.snake_ident)
                    .unwrap(),
            )
        })
    });
    parse_quote! {
        #byline
        pub mod parse {
            #(#parses)*
            pub mod parsell {
                #(#parsells)*
            }
        }
    }
}

pub fn generate_parse<LCst: LangSpec>(bps: &BasePaths, cst_tgd: &TyGenData<LCst>) -> syn::Item {
    let camel_ident = &cst_tgd.camel_ident;
    let snake_ident = &cst_tgd.snake_ident;
    let cst_data_structure_bp = &bps.cst_data_structure;
    let my_cst_ty: syn::Type = syn::parse_quote! { #cst_data_structure_bp::#camel_ident };
    let cstfication = transparency2cstfication(&cst_tgd.transparency);
    let my_cstfied_ty: syn::Type = syn::parse_quote! {
        parse_adt::cstfy::#cstfication<
            #cst_data_structure_bp::Heap,
            #my_cst_ty,
        >
    };
    let pattern_match_strategy = &bps.pattern_match_strategy;
    syn::parse_quote! {
        pub fn #snake_ident(source: &str) -> (#cst_data_structure_bp::Heap, #my_cstfied_ty) {
            let mut parser = parse_adt::Parser::new(source);
            let mut heap = #cst_data_structure_bp::Heap::default();
            let ret = <#my_cstfied_ty as term::co_visit::CoVisitable<
                parse_adt::Parser<'_, ()>,
                #pattern_match_strategy::PatternMatchStrategyProvider<#cst_data_structure_bp::Heap>,
                #cst_data_structure_bp::Heap,
                typenum::U16,
            >>::co_visit(&mut parser, &mut heap);
            (heap, ret)
        }
    }
}

pub fn generate_parsell<LCst: LangSpec>(bps: &BasePaths, cst_tgd: &TyGenData<LCst>) -> syn::Item {
    let camel_ident = &cst_tgd.camel_ident;
    let snake_ident = &cst_tgd.snake_ident;
    let cst_data_structure_bp = &bps.cst_data_structure;
    let my_cst_ty: syn::Type = syn::parse_quote! { #cst_data_structure_bp::#camel_ident };
    syn::parse_quote! {
        impl parse_adt::ParseLL for #my_cst_ty {
            const START: parse::KeywordSequence = parse::KeywordSequence(
                &[parse::Keyword::new(stringify!(#snake_ident))],
            );
            const PROCEED: &'static [parse::KeywordSequence] = &[];
            const END: parse::KeywordSequence = parse::KeywordSequence(
                &[],
            );
        }
    }
}

pub fn gen_unparse(
    snake_ident: &syn::Ident,
    transparency: &Transparency,
    cst_bp: &syn::Path,
    my_ty: &syn::Type,
    my_cst_ty: &syn::Type,
    ccftys: &[syn::Type],
    ccf_idxses: &[Vec<usize>],
) -> syn::ItemFn {
    todo!()
}

fn transparency2cstfication(transparency: &Transparency) -> syn::Ident {
    match transparency {
        Transparency::Transparent => syn::parse_quote! { CstfyTransparent },
        Transparency::Visible => syn::parse_quote! { Cstfy },
    }
}

pub(crate) fn cst<'a, 'b: 'a, L: LangSpec>(
    arena: &'a bumpalo::Bump,
    l: &'b L,
) -> impl LangSpec + 'a {
    let errlang = arena.alloc(std_parse_error::parse_error());
    let fallible_ast = arena.alloc(everywhere_alternative(
        Name {
            human: "FallibleAst".into(),
            camel: "FallibleAst".into(),
            snake: "fallible_ast".into(),
        },
        l,
        errlang,
        SortId::TyMetaFunc(MappedType {
            f: std_parse_error::ParseErrorTmfId(),
            a: vec![],
        }),
    ));
    let parse_metadata = arena.alloc(std_parse_metadata::parse_metadata());
    everywhere_maybemore(
        Name {
            human: "Cst".into(),
            camel: "Cst".into(),
            snake: "cst".into(),
        },
        fallible_ast,
        parse_metadata,
        SortId::TyMetaFunc(MappedType {
            f: std_parse_metadata::ParseMetadataTmfId(),
            a: vec![],
        }),
    )
}

pub fn gen_impls<L: LangSpec>(
    base_path: &syn::Path,
    lsg: &LsGen<L>,
    important_sublangs: &[Name],
) -> proc_macro2::TokenStream {
    let ds =
        term_specialized_gen::generate(&syn::parse_quote!(#base_path::data_structure), lsg, false);
    let tt = term_trait_gen::generate(&syn::parse_quote!(#base_path::term_trait), lsg.bak());
    let tpmsp = term_pattern_match_strategy_provider_gen::generate(
        &term_pattern_match_strategy_provider_gen::BasePaths {
            term_trait: syn::parse_quote!(#base_path::term_trait),
            data_structure: syn::parse_quote!(#base_path::data_structure),
            words: syn::parse_quote!(#base_path::term_trait::words),
            strategy_provider: syn::parse_quote!(#base_path::pattern_match_strategy),
        },
        lsg,
    );
    let tpmspi = term_pattern_match_strategy_provider_impl_gen::generate(
        &term_pattern_match_strategy_provider_impl_gen::BasePaths {
            data_structure: syn::parse_quote!(#base_path::data_structure),
            words: syn::parse_quote!(#base_path::term_trait::words),
            strategy_provider: syn::parse_quote!(#base_path::pattern_match_strategy),
        },
        lsg,
    );
    let tt_impl = term_specialized_impl_gen::generate(
        &term_specialized_impl_gen::BasePaths {
            data_structure: syn::parse_quote!(#base_path::data_structure),
            term_trait: syn::parse_quote!(#base_path::term_trait),
        },
        lsg,
        important_sublangs,
    );
    let tt_words_impl = words::words_impls(
        &syn::parse_quote! { #base_path::term_trait::words },
        &syn::parse_quote! { #base_path::data_structure },
        lsg,
        lsg,
    );
    quote::quote! {
        #ds
        #tt
        #tpmsp
        #tpmspi
        #tt_words_impl
        #tt_impl
    }
}

pub fn formatted<L: LangSpec>(l: &L) -> String {
    let lg = LsGen::from(l);
    let bps = BasePaths {
        data_structure: parse_quote! { crate::data_structure },
        term_trait: parse_quote! { crate::term_trait },
        words: parse_quote! { crate::term_trait::words },
        pattern_match_strategy: parse_quote! { crate::pattern_match_strategy },
        cst_data_structure: parse_quote! { crate::cst::data_structure },
        cst_term_trait: parse_quote! { crate::cst::term_trait },
        cst_words: parse_quote! { crate::cst::words },
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
                println!("test Set");
                let mut parser = parse_adt::Parser::new("{ 3 }");
                let mut heap = crate::cst::data_structure::Heap::default();
                <parse_adt::cstfy::Cstfy<
                    crate::cst::data_structure::Heap,
                    tymetafuncspec_core::Set<crate::cst::data_structure::Heap, tymetafuncspec_core::Either<crate::cst::data_structure::Heap, crate::cst::data_structure::Nat, std_parse_error::ParseError<crate::cst::data_structure::Heap>>>,
                > as term::co_visit::CoVisitable<
                    parse_adt::Parser<'_, ()>,
                    crate::pattern_match_strategy::PatternMatchStrategyProvider<crate::cst::data_structure::Heap>,
                    crate::cst::data_structure::Heap,
                    typenum::U2,
                >>::co_visit(&mut parser, &mut heap);
            }
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
                println!("test IdxBox");
                let mut parser = parse_adt::Parser::new("plus left_operand 3 right_operand 4");
                let mut heap = crate::cst::data_structure::Heap::default();
                <parse_adt::cstfy::Cstfy<
                    cds::Heap,
                    tymetafuncspec_core::IdxBox<cds::Heap, parse_adt::cstfy::Cstfy<cds::Heap, cds::Plus>>,
                > as term::co_visit::CoVisitable<
                    Parser<'_, ()>,
                    crate::pattern_match_strategy::PatternMatchStrategyProvider<cds::Heap>,
                    cds::Heap,
                    typenum::U5,
                >>::co_visit(&mut parser, &mut heap);
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
