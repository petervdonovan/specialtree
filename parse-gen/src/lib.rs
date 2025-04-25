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
    let parses = lg.ty_gen_datas(None).filter_map(|tgd| {
        tgd.id.as_ref().map(|sid| {
            generate_parse(
                bps,
                &tgd,
                cst_tgds
                    .iter()
                    .find(|cst_tgd| cst_tgd.snake_ident == tgd.snake_ident) // FIXME: comparing idents
                    .unwrap(),
            )
        })
    });
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
    let tpmspi = term_pattern_match_strategy_provider_impl_gen::generate(
        &term_pattern_match_strategy_provider_impl_gen::BasePaths {
            term_trait: syn::parse_quote!(#base_path::term_trait),
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
        #tpmspi
        #tt_words_impl
        #tt_impl
    }
}

pub fn bridge_words_impls<L: LangSpec>(
    extension_ds_base_path: &syn::Path,
    og_words_base_path: &syn::Path,
    elsg: &LsGen<L>,
) -> syn::ItemMod {
    let impls = elsg.ty_gen_datas(Some(og_words_base_path.clone())).map(|tgd| -> syn::ItemImpl {
        let camel_ident = &tgd.camel_ident;
        let cstfication = transparency2cstfication(&tgd.transparency);
        let ty: syn::Type = syn::parse_quote! {
            parse_adt::cstfy::#cstfication<#extension_ds_base_path::Heap, #extension_ds_base_path::#camel_ident>
        };
        syn::parse_quote! {
            impl words::Implements<#og_words_base_path::L> for #ty {
                type LWord = #og_words_base_path::sorts::#camel_ident;
            }
        }
    });
    let byline = byline!();
    syn::parse_quote! {
        #byline
        pub mod words_impls {
            #(#impls)*
        }
    }
}

pub fn gen_bridge<LOg: LangSpec, LExt: LangSpec>(
    extension_base_path: &syn::Path,
    og_base_path: &syn::Path,
    ext_lg: &LsGen<LExt>,
    oglsg: &LsGen<LOg>,
) -> proc_macro2::TokenStream {
    let words_impls = bridge_words_impls(
        &syn::parse_quote! { #extension_base_path::data_structure },
        &syn::parse_quote! { #og_base_path::term_trait::words },
        ext_lg,
    );
    let bridge = term_bridge_gen::generate(
        ext_lg,
        oglsg.bak().name(),
        &term_bridge_gen::BasePaths {
            ext_data_structure: syn::parse_quote!(#extension_base_path::data_structure),
            og_term_trait: syn::parse_quote!(#og_base_path::term_trait),
        },
    );
    quote::quote! {
        #words_impls
        #bridge
    }
}

pub fn formatted<L: LangSpec>(l: &L) -> String {
    let lg = LsGen::from(l);
    let bps = BasePaths {
        data_structure: parse_quote! { crate::data_structure },
        term_trait: parse_quote! { crate::term_trait },
        words: parse_quote! { crate::term_trait::words },
        cst_data_structure: parse_quote! { crate::cst::data_structure },
        cst_term_trait: parse_quote! { crate::cst::term_trait },
        cst_words: parse_quote! { crate::cst::words },
    };
    let m = generate(&bps, &lg);
    let (cst_impls, bridge) = {
        let arena = bumpalo::Bump::new();
        let cst = cst(&arena, l);
        let ext_lg = LsGen::from(&cst);
        (
            gen_impls(
                &syn::parse_quote! { crate::cst },
                &ext_lg,
                &[cst.name().clone(), l.name().clone()],
            ),
            gen_bridge(
                &syn::parse_quote! { crate::cst },
                &syn::parse_quote! { crate },
                &ext_lg,
                &lg,
            ),
        )
    };
    let root_impls = gen_impls(&syn::parse_quote! { crate }, &lg, &[l.name().clone()]);
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
                let mut parser = parse_adt::Parser::new("sum { 3 }");
                let mut heap = crate::cst::data_structure::Heap::default();
                <parse_adt::cstfy::Cstfy<
                    crate::cst::data_structure::Heap,
                    crate::cst::data_structure::Sum,
                > as term::co_visit::CoVisitable<
                    parse_adt::Parser<'_, ()>,
                    crate::pattern_match_strategy::PatternMatchStrategyProvider<crate::cst::data_structure::Heap>,
                    crate::cst::data_structure::Heap,
                    typenum::U3,
                >>::co_visit(&mut parser, &mut heap);
            }
            {
                println!("test Nat");
                let mut parser = parse_adt::Parser::new("3");
                let mut heap = crate::cst::data_structure::Heap::default();
                <parse_adt::cstfy::CstfyTransparent<
                    crate::cst::data_structure::Heap,
                    crate::cst::data_structure::Nat,
                > as term::co_visit::CoVisitable<
                    parse_adt::Parser<'_, ()>,
                    crate::pattern_match_strategy::PatternMatchStrategyProvider<crate::cst::data_structure::Heap>,
                    crate::cst::data_structure::Heap,
                    typenum::U4,
                >>::co_visit(&mut parser, &mut heap);
            }
            {
                println!("test Nat");
                let mut parser = parse_adt::Parser::new("f 3");
                let mut heap = crate::cst::data_structure::Heap::default();
                <CstfyTransparent<
                    crate::cst::data_structure::Heap,
                    crate::cst::data_structure::Nat,
                > as term::co_visit::CoVisitable<
                    Parser<'_, ()>,
                    crate::pattern_match_strategy::PatternMatchStrategyProvider<crate::cst::data_structure::Heap>,
                    crate::cst::data_structure::Heap,
                    typenum::U2,
                >>::co_visit(&mut parser, &mut heap);
            }
            {
                println!("test Sum");
                let mut parser = parse_adt::Parser::new("sum { 3 }");
                let mut heap = crate::cst::data_structure::Heap::default();
                <Cstfy<
                    crate::cst::data_structure::Heap,
                    crate::cst::data_structure::Sum,
                > as term::co_visit::CoVisitable<
                    Parser<'_, ()>,
                    crate::pattern_match_strategy::PatternMatchStrategyProvider<crate::cst::data_structure::Heap>,
                    crate::cst::data_structure::Heap,
                    typenum::U3,
                >>::co_visit(&mut parser, &mut heap);
            }
            {
                println!("test F");
                let mut parser = parse_adt::Parser::new("f 3");
                let mut heap = crate::cst::data_structure::Heap::default();
                <parse_adt::cstfy::Cstfy<
                    crate::cst::data_structure::Heap,
                    crate::cst::data_structure::F,
                > as term::co_visit::CoVisitable<
                    Parser<'_, ()>,
                    crate::pattern_match_strategy::PatternMatchStrategyProvider<crate::cst::data_structure::Heap>,
                    crate::cst::data_structure::Heap,
                    typenum::U7,
                >>::co_visit(&mut parser, &mut heap);
            }
            {
                println!("test Plus");
                let mut parser = parse_adt::Parser::new("plus left_operand 3 right_operand 4");
                let mut heap = crate::cst::data_structure::Heap::default();
                <parse_adt::cstfy::Cstfy<
                    crate::cst::data_structure::Heap,
                    crate::cst::data_structure::Plus,
                > as term::co_visit::CoVisitable<
                    Parser<'_, ()>,
                    crate::pattern_match_strategy::PatternMatchStrategyProvider<crate::cst::data_structure::Heap>,
                    crate::cst::data_structure::Heap,
                    typenum::U3,
                >>::co_visit(&mut parser, &mut heap);
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
                let mut parser = parse_adt::Parser::new("sum { f 3, f plus left_operand f 1 right_operand 4 }");
                let mut heap = crate::cst::data_structure::Heap::default();
                <CstfyTransparent<
                    crate::cst::data_structure::Heap,
                    crate::cst::data_structure::Nat,
                > as term::co_visit::CoVisitable<
                    Parser<'_, ()>,
                    crate::pattern_match_strategy::PatternMatchStrategyProvider<crate::cst::data_structure::Heap>,
                    crate::cst::data_structure::Heap,
                    typenum::U2,
                >>::co_visit(&mut parser, &mut heap);
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
        #root_impls
    };
    prettyplease::unparse(&syn_insert_use::insert_use(f))
}
