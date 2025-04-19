use extension_everywhere_alternative::EverywhereAlternative;
use extension_everywhere_maybemore::EverywhereMaybeMore;
use langspec::{
    langspec::{LangSpec, MappedType, Name, SortId},
    tymetafunc::Transparency,
};
use langspec_gen_util::{byline, proc_macro2, AlgebraicsBasePath, HeapType, LsGen, TyGenData};
use syn::parse_quote;

pub struct BasePaths {
    pub data_structure: syn::Path,
    pub term_trait: syn::Path,
    pub words: syn::Path,
    pub cst_data_structure: syn::Path,
    pub cst_term_trait: syn::Path,
    pub cst_words: syn::Path,
}

// pub fn generate<L: LangSpec>(bps: &BasePaths, lg: &LsGen<L>) -> syn::ItemMod {
//     let byline = byline!();
//     let arena = bumpalo::Bump::new();
//     let cst = cst(&arena, lg.bak());
//     let lg_cst = LsGen::from(&cst);
//     let cst_tgds = lg_cst.ty_gen_datas().collect::<Vec<_>>();
//     // let parses = lg.ty_gen_datas().filter_map(|tgd| {
//     //     tgd.id.as_ref().map(|sid| {
//     //         generate_parse(
//     //             bps,
//     //             &tgd,
//     //             cst_tgds
//     //                 .iter()
//     //                 .find(|cst_tgd| cst_tgd.snake_ident == tgd.snake_ident) // FIXME: comparing idents
//     //                 .unwrap(),
//     //         )
//     //     })
//     // });
//     let parses: Vec<syn::ItemImpl> = vec![];
//     parse_quote! {
//         #byline
//         pub mod parse {
//             #(#parses)*
//         }
//     }
// }

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

pub fn gen_impls<L: LangSpec>(base_path: &syn::Path, lsg: &LsGen<L>) -> proc_macro2::TokenStream {
    let ds =
        term_specialized_gen::generate(&syn::parse_quote!(#base_path::data_structure), lsg, false);
    let tt = term_trait_gen::generate(&syn::parse_quote!(#base_path::extension_of), lsg.bak());
    let tpmspi = term_pattern_match_strategy_provider_impl_gen::generate(
        &term_pattern_match_strategy_provider_impl_gen::BasePaths {
            term_trait: syn::parse_quote!(#base_path::extension_of),
            data_structure: syn::parse_quote!(#base_path::data_structure),
            words: syn::parse_quote!(#base_path::extension_of::words),
            strategy_provider: syn::parse_quote!(#base_path::pattern_match_strategy),
        },
        lsg,
    );
    let tt_impl = term_specialized_impl_gen::generate(
        &term_specialized_impl_gen::BasePaths {
            data_structure: syn::parse_quote!(#base_path::data_structure),
            term_trait: syn::parse_quote!(#base_path::extension_of),
        },
        lsg,
    );
    // let tt_words = words::words_mod(lsg);
    let tt_words_impl = words::words_impls(
        &syn::parse_quote! { #base_path::extension_of::words },
        &syn::parse_quote! { #base_path::data_structure },
        lsg,
        lsg,
    );
    quote::quote! {
        #ds
        #tt
        #tpmspi
        // #tt_words
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

pub fn gen_bridge<L: LangSpec>(
    extension_base_path: &syn::Path,
    og_base_path: &syn::Path,
    elsg: &LsGen<L>,
    oglsg: &LsGen<L>,
) -> proc_macro2::TokenStream {
    let words_impls = bridge_words_impls(
        &syn::parse_quote! { #extension_base_path::data_structure },
        &syn::parse_quote! { #og_base_path::extension_of::words },
        elsg,
    );
    // let bridge_tt_impl = term_specialized_impl_gen::generate_bridge(
    //     &term_specialized_impl_gen::BasePaths {
    //         data_structure: syn::parse_quote!(#extension_base_path::data_structure),
    //         term_trait: syn::parse_quote!(#og_base_path::extension_of),
    //     },
    //     elsg,
    //     oglsg,
    // );
    let temp_paste = temp_bridge_paste();
    quote::quote! {
        #words_impls
        #temp_paste
    }
}

pub fn formatted<L: LangSpec>(l: &L) -> String {
    let lg = LsGen::from(l);
    // let bps = BasePaths {
    //     data_structure: parse_quote! { crate::data_structure },
    //     term_trait: parse_quote! { crate::extension_of },
    //     words: parse_quote! { crate::extension_of::words },
    //     cst_data_structure: parse_quote! { crate::cst::data_structure },
    //     cst_term_trait: parse_quote! { crate::cst::extension_of },
    //     cst_words: parse_quote! { crate::cst::words },
    // };
    // let m = generate(&bps, &lg);
    let cst_impls = {
        let arena = bumpalo::Bump::new();
        let cst = cst(&arena, l);
        let lg = LsGen::from(&cst);
        gen_impls(&syn::parse_quote! { crate::cst }, &lg)
    };
    let bridge = gen_bridge(
        &syn::parse_quote! { crate::cst },
        &syn::parse_quote! { crate },
        &lg,
        &lg,
    );
    let root_impls = gen_impls(&syn::parse_quote! { crate }, &lg);
    let byline = byline!();
    let f: syn::File = syn::parse_quote! {
        // #byline
        // fn test() {
        //     // impl<'a>
        //     //     term::co_visit::CoVisitable<
        //     //         parse_adt::Parser<'_>,
        //     //         crate::pattern_match_strategy::PatternMatchStrategyProvider<cst::data_structure::Heap>,
        //     //         cst::data_structure::Heap,
        //     //     > for parse_adt::cstfy::CstfyTransparent<cst::data_structure::Heap, cst::data_structure::Nat>
        //     // {
        //     //     fn co_visit(
        //     //         visitor: &mut parse_adt::Parser<'_>,
        //     //         heap: &mut cst::data_structure::Heap,
        //     //     ) -> Self {
        //     //         todo!()
        //     //     }
        //     // }

        //     let mut parser = parse_adt::Parser::new("test");
        //     let mut heap = cst::data_structure::Heap::default();
        //     <parse_adt::cstfy::CstfyTransparent<cst::data_structure::Heap, cst::data_structure::Nat> as term::co_visit::CoVisitable<
        //         parse_adt::Parser<'_>,
        //         crate::pattern_match_strategy::PatternMatchStrategyProvider<cst::data_structure::Heap>,
        //         cst::data_structure::Heap,
        //     >>::co_visit(&mut parser, &mut heap);

        //     <parse_adt::cstfy::Cstfy<cst::data_structure::Heap, cst::data_structure::F> as term::co_visit::CoVisitable<
        //         parse_adt::Parser<'_>,
        //         crate::pattern_match_strategy::PatternMatchStrategyProvider<cst::data_structure::Heap>,
        //         cst::data_structure::Heap,
        //     >>::co_visit(&mut parser, &mut heap);

        //     <parse_adt::cstfy::Cstfy<cst::data_structure::Heap, cst::data_structure::Plus> as term::co_visit::CoVisitable<
        //         parse_adt::Parser<'_>,
        //         crate::pattern_match_strategy::PatternMatchStrategyProvider<cst::data_structure::Heap>,
        //         cst::data_structure::Heap,
        //     >>::co_visit(&mut parser, &mut heap);
        // }
        // #m
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

pub(crate) fn temp_bridge_paste() -> syn::ItemMod {
    let byline = byline!();
    syn::parse_quote! {
    #byline
    pub mod term_impls {
        impl crate::extension_of::Heap for crate::cst::data_structure::Heap {
            type F = parse_adt::cstfy::Cstfy<
                crate::cst::data_structure::Heap,
                crate::cst::data_structure::F,
            >;
            type Plus = parse_adt::cstfy::Cstfy<
                crate::cst::data_structure::Heap,
                crate::cst::data_structure::Plus,
            >;
            type LeftOperand = parse_adt::cstfy::Cstfy<
                crate::cst::data_structure::Heap,
                crate::cst::data_structure::LeftOperand,
            >;
            type RightOperand = parse_adt::cstfy::Cstfy<
                crate::cst::data_structure::Heap,
                crate::cst::data_structure::RightOperand,
            >;
            type Sum = parse_adt::cstfy::Cstfy<
                crate::cst::data_structure::Heap,
                crate::cst::data_structure::Sum,
            >;
            type Nat = parse_adt::cstfy::CstfyTransparent<
                crate::cst::data_structure::Heap,
                crate::cst::data_structure::Nat,
            >;
        }
        pub mod owned_impls {
            impl crate::extension_of::owned::F
                for parse_adt::cstfy::Cstfy<
                    crate::cst::data_structure::Heap,
                    crate::cst::data_structure::F,
                >
            {
            }
            impl crate::extension_of::owned::Plus
                for parse_adt::cstfy::Cstfy<
                    crate::cst::data_structure::Heap,
                    crate::cst::data_structure::Plus,
                >
            {
            }
            impl crate::extension_of::owned::LeftOperand
                for parse_adt::cstfy::Cstfy<
                    crate::cst::data_structure::Heap,
                    crate::cst::data_structure::LeftOperand,
                >
            {
            }
            impl crate::extension_of::owned::RightOperand
                for parse_adt::cstfy::Cstfy<
                    crate::cst::data_structure::Heap,
                    crate::cst::data_structure::RightOperand,
                >
            {
            }
            impl crate::extension_of::owned::Sum
                for parse_adt::cstfy::Cstfy<
                    crate::cst::data_structure::Heap,
                    crate::cst::data_structure::Sum,
                >
            {
            }
            impl crate::extension_of::owned::Nat
                for parse_adt::cstfy::CstfyTransparent<
                    crate::cst::data_structure::Heap,
                    crate::cst::data_structure::Nat,
                >
            {
            }
        }

        impl term::MapsTmf<crate::extension_of::words::L, tymetafuncspec_core::IdxBox<cds::Heap, tymetafuncspec_core::Either<cds::Heap, tymetafuncspec_core::Pair<cds::Heap,cds::Sum,tymetafuncspec_core::Maybe<cds::Heap,std_parse_metadata::ParseMetadata<cds::Heap>>>, std_parse_error::ParseError<cds::Heap>>>> for cds::Heap {
            type Tmf = Cstfy<cds::Heap, tymetafuncspec_core::IdxBox<cds::Heap, tymetafuncspec_core::Either<cds::Heap, tymetafuncspec_core::Pair<cds::Heap,cds::Sum,tymetafuncspec_core::Maybe<cds::Heap,std_parse_metadata::ParseMetadata<cds::Heap>>>, std_parse_error::ParseError<cds::Heap>>>>;
        }

        impl term::MapsTmf<crate::extension_of::words::L, tymetafuncspec_core::IdxBox<crate::cst::data_structure::Heap, tymetafuncspec_core::Either<crate::cst::data_structure::Heap, tymetafuncspec_core::Pair<crate::cst::data_structure::Heap, crate::cst::data_structure::Plus, tymetafuncspec_core::Maybe<crate::cst::data_structure::Heap, std_parse_metadata::ParseMetadata<crate::cst::data_structure::Heap>>>, std_parse_error::ParseError<crate::cst::data_structure::Heap>>>> for cds::Heap {
            type Tmf = Cstfy<cds::Heap, tymetafuncspec_core::IdxBox<crate::cst::data_structure::Heap, tymetafuncspec_core::Either<crate::cst::data_structure::Heap, tymetafuncspec_core::Pair<crate::cst::data_structure::Heap, crate::cst::data_structure::Plus, tymetafuncspec_core::Maybe<crate::cst::data_structure::Heap, std_parse_metadata::ParseMetadata<crate::cst::data_structure::Heap>>>, std_parse_error::ParseError<crate::cst::data_structure::Heap>>>>;
        }

        impl term::MapsTmf<crate::extension_of::words::L, tymetafuncspec_core::BoundedNat<cds::Heap>> for cds::Heap {
            type Tmf = parse_adt::cstfy::Cstfy<cds::Heap, tymetafuncspec_core::BoundedNat<cds::Heap>>;
        }

        impl term::MapsTmf<crate::extension_of::words::L, tymetafuncspec_core::Set<crate::cst::data_structure::Heap, tymetafuncspec_core::Either<crate::cst::data_structure::Heap, crate::cst::data_structure::Nat, std_parse_error::ParseError<crate::cst::data_structure::Heap>>>> for cds::Heap {
            type Tmf = Cstfy<cds::Heap, tymetafuncspec_core::Set<cds::Heap, tymetafuncspec_core::Either<cds::Heap, crate::cst::data_structure::Nat, std_parse_error::ParseError<cds::Heap>>>>;
        }

        impl term::MapsTmf<crate::extension_of::words::L, tymetafuncspec_core::IdxBox<crate::cst::data_structure::Heap, tymetafuncspec_core::Either<crate::cst::data_structure::Heap, tymetafuncspec_core::Pair<crate::cst::data_structure::Heap, crate::cst::data_structure::F, tymetafuncspec_core::Maybe<crate::cst::data_structure::Heap, std_parse_metadata::ParseMetadata<crate::cst::data_structure::Heap>>>, std_parse_error::ParseError<crate::cst::data_structure::Heap>>>> for cds::Heap {
            type Tmf = Cstfy<cds::Heap, tymetafuncspec_core::IdxBox<cds::Heap, tymetafuncspec_core::Either<cds::Heap, tymetafuncspec_core::Pair<cds::Heap, cds::F, tymetafuncspec_core::Maybe<cds::Heap, std_parse_metadata::ParseMetadata<cds::Heap>>>, std_parse_error::ParseError<cds::Heap>>>>;
        }

        impl term::MapsTmf<crate::extension_of::words::L, tymetafuncspec_core::Either<cds::Heap, tymetafuncspec_core::Pair<cds::Heap, cds::F, tymetafuncspec_core::Maybe<cds::Heap, std_parse_metadata::ParseMetadata<cds::Heap>>>, std_parse_error::ParseError<cds::Heap>>> for cds::Heap {
            type Tmf = tymetafuncspec_core::Either<cds::Heap, tymetafuncspec_core::Pair<cds::Heap, cds::F, tymetafuncspec_core::Maybe<cds::Heap, std_parse_metadata::ParseMetadata<cds::Heap>>>, std_parse_error::ParseError<cds::Heap>>;
        }
        impl term::MapsTmf<crate::extension_of::words::L, tymetafuncspec_core::Either<cds::Heap, tymetafuncspec_core::Pair<cds::Heap, cds::Plus, tymetafuncspec_core::Maybe<cds::Heap, std_parse_metadata::ParseMetadata<cds::Heap>>>, std_parse_error::ParseError<cds::Heap>>> for cds::Heap {
            type Tmf = tymetafuncspec_core::Either<cds::Heap, tymetafuncspec_core::Pair<cds::Heap, cds::Plus, tymetafuncspec_core::Maybe<cds::Heap, std_parse_metadata::ParseMetadata<cds::Heap>>>, std_parse_error::ParseError<cds::Heap>>;

        }
        impl term::MapsTmf<crate::extension_of::words::L, tymetafuncspec_core::Either<cds::Heap, tymetafuncspec_core::Pair<cds::Heap, cds::LeftOperand, tymetafuncspec_core::Maybe<cds::Heap, std_parse_metadata::ParseMetadata<cds::Heap>>>, std_parse_error::ParseError<cds::Heap>>> for cds::Heap {
            type Tmf = tymetafuncspec_core::Either<cds::Heap, tymetafuncspec_core::Pair<cds::Heap, cds::LeftOperand, tymetafuncspec_core::Maybe<cds::Heap, std_parse_metadata::ParseMetadata<cds::Heap>>>, std_parse_error::ParseError<cds::Heap>>;
        }
        impl term::MapsTmf<crate::extension_of::words::L, tymetafuncspec_core::Either<cds::Heap, tymetafuncspec_core::Pair<cds::Heap, cds::RightOperand, tymetafuncspec_core::Maybe<cds::Heap, std_parse_metadata::ParseMetadata<cds::Heap>>>, std_parse_error::ParseError<cds::Heap>>> for cds::Heap {
            type Tmf = tymetafuncspec_core::Either<cds::Heap, tymetafuncspec_core::Pair<cds::Heap, cds::RightOperand, tymetafuncspec_core::Maybe<cds::Heap, std_parse_metadata::ParseMetadata<cds::Heap>>>, std_parse_error::ParseError<cds::Heap>>;
        }
        impl term::MapsTmf<crate::extension_of::words::L, tymetafuncspec_core::Either<cds::Heap, tymetafuncspec_core::Pair<cds::Heap, cds::Sum, tymetafuncspec_core::Maybe<cds::Heap, std_parse_metadata::ParseMetadata<cds::Heap>>>, std_parse_error::ParseError<cds::Heap>>> for cds::Heap {
            type Tmf = tymetafuncspec_core::Either<cds::Heap, tymetafuncspec_core::Pair<cds::Heap, cds::Sum, tymetafuncspec_core::Maybe<cds::Heap, std_parse_metadata::ParseMetadata<cds::Heap>>>, std_parse_error::ParseError<cds::Heap>>;
        }
        impl term::MapsTmf<crate::extension_of::words::L, tymetafuncspec_core::Either<cds::Heap, cds::Nat, std_parse_error::ParseError<cds::Heap>>> for cds::Heap {
            type Tmf = tymetafuncspec_core::Either<cds::Heap, cds::Nat, std_parse_error::ParseError<cds::Heap>>;
        }
    }
    }
}
