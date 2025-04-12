use extension_everywhere_alternative::EverywhereAlternative;
use extension_everywhere_maybemore::EverywhereMaybeMore;
use langspec::{
    langspec::{LangSpec, MappedType, Name, SortId},
    tymetafunc::Transparency,
};
use langspec_gen_util::{byline, AlgebraicsBasePath, HeapType, LsGen, TyGenData};
use syn::parse_quote;

pub struct BasePaths {
    pub data_structure: syn::Path,
    pub term_trait: syn::Path,
    pub cst_data_structure: syn::Path,
    pub cst_term_trait: syn::Path,
}

pub fn generate<L: LangSpec>(bps: &BasePaths, lg: &LsGen<L>) -> syn::ItemMod {
    let byline = byline!();
    let arena = bumpalo::Bump::new();
    let cst = cst(&arena, lg.bak());
    let lg_cst = LsGen::from(&cst);
    let cst_tgds = lg_cst.ty_gen_datas().collect::<Vec<_>>();
    // let parses = lg.ty_gen_datas().filter_map(|tgd| {
    //     tgd.id.as_ref().map(|sid| {
    //         generate_parse(
    //             bps,
    //             &tgd,
    //             cst_tgds
    //                 .iter()
    //                 .find(|cst_tgd| cst_tgd.snake_ident == tgd.snake_ident) // FIXME: comparing idents
    //                 .unwrap(),
    //         )
    //     })
    // });
    let parses: Vec<syn::ItemImpl> = vec![];
    parse_quote! {
        #byline
        pub mod parse {
            #(#parses)*
        }
    }
}

pub fn generate_parse<L: LangSpec, LCst: LangSpec>(
    bps: &BasePaths,
    tgd: &TyGenData<L>,
    cst_tgd: &TyGenData<LCst>,
) -> syn::Item {
    let camel_ident = &cst_tgd.camel_ident;
    let snake_ident = &cst_tgd.snake_ident;
    let cstfication = transparency2cstfication(&tgd.transparency);
    let cst_data_structure_bp = &bps.cst_data_structure;
    let my_cst_ty: syn::Type = syn::parse_quote! { #cst_data_structure_bp::#camel_ident };
    syn::parse_quote! {
        impl parse_adt::ParseLL for parse_adt::cstfy::#cstfication<#cst_data_structure_bp::Heap, #my_cst_ty> {
            const START: parse_adt::KeywordSequence = parse_adt::KeywordSequence(
                &[parse_adt::Keyword::new("#snake_ident")],
            );
            const PROCEED: &'static [parse_adt::KeywordSequence] = &[];
            const END: parse_adt::KeywordSequence = parse_adt::KeywordSequence {
                keywords: &[parse_adt::Keyword::#snake_ident],
            };
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
    let fallible_ast = arena.alloc(EverywhereAlternative {
        name: Name {
            human: "FallibleAst".into(),
            camel: "FallibleAst".into(),
            snake: "fallible_ast".into(),
        },
        l0: l,
        l1: errlang,
        l1_root: SortId::TyMetaFunc(MappedType {
            f: std_parse_error::ParseErrorTmfId(),
            a: vec![],
        }),
    });
    let parse_metadata = arena.alloc(std_parse_metadata::parse_metadata());
    EverywhereMaybeMore {
        name: Name {
            human: "Cst".into(),
            camel: "Cst".into(),
            snake: "cst".into(),
        },
        l0: fallible_ast,
        l1: parse_metadata,
        l1_root: SortId::TyMetaFunc(MappedType {
            f: std_parse_metadata::ParseMetadataTmfId(),
            a: vec![],
        }),
    }
}

pub fn formatted<L: LangSpec>(l: &L) -> String {
    let lg = LsGen::from(l);
    let bps = BasePaths {
        data_structure: parse_quote! { crate::data_structure },
        term_trait: parse_quote! { crate::extension_of },
        cst_data_structure: parse_quote! { crate::cst::data_structure },
        cst_term_trait: parse_quote! { crate::cst::extension_of },
    };
    let m = generate(&bps, &lg);
    let (cst_mod, cst_tt, cst_tpmspi, cst_tt_impl) = {
        let arena = bumpalo::Bump::new();
        let cst = cst(&arena, l);
        let lg = LsGen::from(&cst);
        let cst_mod = term_specialized_gen::generate(
            &parse_quote! { crate::cst::data_structure },
            &lg,
            false,
        );
        let cst_tt = term_trait_gen::generate(&parse_quote! { crate::cst::extension_of }, &cst);
        let cst_tt_impl = term_specialized_impl_gen::generate(
            &term_specialized_impl_gen::BasePaths {
                data_structure: parse_quote! { crate::cst::data_structure },
                term_trait: parse_quote! { crate::cst::extension_of },
            },
            &lg,
            &lg,
        );
        let cst_tpmspi = term_pattern_match_strategy_provider_impl_gen::generate(
            &term_pattern_match_strategy_provider_impl_gen::BasePaths {
                term_trait: syn::parse_quote!(crate::cst::extension_of),
                data_structure: syn::parse_quote!(crate::cst::data_structure),
                strategy_provider: syn::parse_quote!(crate::cst::pattern_match_strategy),
            },
            &lg,
        );
        (cst_mod, cst_tt, cst_tpmspi, cst_tt_impl)
    };
    let ds = term_specialized_gen::generate(&bps.data_structure, &lg, false);
    let tt = term_trait_gen::generate(&bps.term_trait, lg.bak());
    let tpmspi = term_pattern_match_strategy_provider_impl_gen::generate(
        &term_pattern_match_strategy_provider_impl_gen::BasePaths {
            term_trait: syn::parse_quote!(crate::extension_of),
            data_structure: syn::parse_quote!(crate::data_structure),
            strategy_provider: syn::parse_quote!(crate::pattern_match_strategy),
        },
        &lg,
    );
    prettyplease::unparse(&syn::parse_quote! {
        fn test() {
            impl<'a>
                term::co_visit::CoVisitable<
                    parse_adt::Parser<'_>,
                    cst::pattern_match_strategy::PatternMatchStrategyProvider,
                    cst::data_structure::Heap,
                > for parse_adt::cstfy::CstfyTransparent<cst::data_structure::Heap, cst::data_structure::Nat>
            {
                fn co_visit(
                    visitor: &mut parse_adt::Parser<'_>,
                    heap: &mut cst::data_structure::Heap,
                ) -> Self {
                    todo!()
                }
            }

            let mut parser = parse_adt::Parser::new("test");
            let mut heap = cst::data_structure::Heap::default();
            <parse_adt::cstfy::CstfyTransparent<cst::data_structure::Heap, cst::data_structure::Nat> as term::co_visit::CoVisitable<
                parse_adt::Parser<'_>,
                cst::pattern_match_strategy::PatternMatchStrategyProvider,
                cst::data_structure::Heap,
            >>::co_visit(&mut parser, &mut heap);

            <parse_adt::cstfy::Cstfy<cst::data_structure::Heap, cst::data_structure::F> as term::co_visit::CoVisitable<
                parse_adt::Parser<'_>,
                cst::pattern_match_strategy::PatternMatchStrategyProvider,
                cst::data_structure::Heap,
            >>::co_visit(&mut parser, &mut heap);

            <parse_adt::cstfy::Cstfy<cst::data_structure::Heap, cst::data_structure::Plus> as term::co_visit::CoVisitable<
                parse_adt::Parser<'_>,
                cst::pattern_match_strategy::PatternMatchStrategyProvider,
                cst::data_structure::Heap,
            >>::co_visit(&mut parser, &mut heap);
        }
        #m
        pub mod cst {
            #cst_mod
            #cst_tt
            // #cst_tpmspi
            #cst_tt_impl
        }
        #ds
        #tt
        #tpmspi
        // #tt_impl
    })
}
