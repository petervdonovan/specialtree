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
    pub cst: syn::Path,
}

pub fn generate<L: LangSpec>(bps: &BasePaths, lg: &LsGen<L>) -> syn::ItemMod {
    let byline = byline!();
    let arena = bumpalo::Bump::new();
    let cst = cst(&arena, lg.bak());
    let lg_cst = LsGen::from(&cst);
    let cst_tgds = lg_cst.ty_gen_datas().collect::<Vec<_>>();
    let parses = lg.ty_gen_datas().filter_map(|tgd| {
        tgd.id.as_ref().map(|sid| {
            generate_parse(
                bps,
                &tgd,
                cst_tgds
                    .iter()
                    .find(|cst_tgd| cst_tgd.snake_ident.to_string() == tgd.snake_ident.to_string()) // FIXME: comparing idents
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
    let snake_ident = &cst_tgd.snake_ident;
    let camel_ident = &cst_tgd.camel_ident;
    let byline = byline!();
    let ast_bp = &bps.data_structure;
    let cst_bp = &bps.cst;
    let cst_bp: syn::Path = parse_quote! { #cst_bp::cst::data_structure };
    let my_ty: syn::Type = syn::parse_quote! { #ast_bp::#camel_ident };
    let my_cst_ty: syn::Type = syn::parse_quote! { #cst_bp::#camel_ident };
    let my_cst_path: syn::Path = parse_quote! { #cst_bp::#camel_ident };
    let cst_ccf_sort_snakes = (cst_tgd.ccf.ccf_sort_snake_idents)();
    let cst_ccf_sort_tys = (cst_tgd.ccf.ccf_sort_tys)(
        HeapType(syn::parse_quote! { #cst_bp::Heap }),
        AlgebraicsBasePath::new(quote::quote!(#cst_bp::)),
    );
    let maybe_parse_this = if cst_ccf_sort_tys.len() == 1 {
        let cst_ccf = cst_ccf_sort_tys.first().unwrap().clone();
        let cst_ccf_tys = (cst_tgd.ccf.ccf_sort_tyses)(
            HeapType(syn::parse_quote! { #cst_bp::Heap }),
            AlgebraicsBasePath::new(quote::quote!(#cst_bp::)),
        )
        .into_iter()
        .next()
        .unwrap();
        let ast_ccf_tys = (tgd.ccf.ccf_sort_tyses)(
            HeapType(syn::parse_quote! { #cst_bp::Heap }),
            AlgebraicsBasePath::new(quote::quote!(#cst_bp::)),
        )
        .into_iter()
        .next()
        .unwrap();
        let parse_directly = gen_parse_current(
            snake_ident,
            &tgd.transparency,
            ((tgd.ccf.ccf_sort_transparencies)()[0]).as_slice(),
            &cst_ccf_tys,
            &cst_bp,
            &cst_ccf,
            &my_cst_path,
        );
        quote::quote! {
            #parse_directly
        }
    } else {
        quote::quote! {let parse_directly = |_,_,_,_, ret| ret}
    };
    let ccf_tys_size1 = cst_ccf_sort_tys
        .into_iter()
        .zip(cst_ccf_sort_snakes.iter())
        .filter_map(|(it, snake)| if snake.len() == 1 + 1 { Some(it) } else { None });
    let untupled_ccf_tys_size1 = (cst_tgd.ccf.ccf_sort_tyses)(
        HeapType(syn::parse_quote! { #cst_bp::Heap }),
        AlgebraicsBasePath::new(quote::quote!(#cst_bp::)),
    )
    .into_iter()
    .filter_map(|it| {
        if it.len() == 1 {
            Some(it.into_iter().next().unwrap())
        } else {
            None
        }
    });
    let cstfication = transparency2cstfication(&tgd.transparency);
    syn::parse_quote! {
        #byline
        impl parse::Parse<#cst_bp::Heap> for tymetafuncspec_core::#cstfication<#cst_bp::Heap, #my_cst_ty> {
            fn parse(source: &str, offset: parse::miette::SourceOffset, heap: &mut #cst_bp::Heap, errors: &mut Vec<parse::ParseError>) -> (tymetafuncspec_core::#cstfication<#cst_bp::Heap, #my_cst_ty>, parse::miette::SourceOffset) {
                let mut ute = None;
                #maybe_parse_this ;
                #(
                    match <#untupled_ccf_tys_size1 as parse::Parse>::parse(source, offset, heap, errors) {
                        Ok((cst, offset)) => return Ok((
                            <#my_cst_ty as term::CanonicallyConstructibleFrom<#ccf_tys_size1>>::construct(heap, (tymetafuncspec_core::Either::Left(cst, std::marker::PhantomData), tymetafuncspec_core::Maybe::Nothing)),
                            offset
                        )),
                        Err(e) => {
                            ute = parse::ParseError::merge_over(e, ute);
                        }
                    }
                )*
                parse_directly(source, offset, heap, errors, ute.unwrap())
            }
        }
    }
}

pub fn gen_parse_current(
    snake_ident: &syn::Ident,
    transparency: &Transparency,
    ccf_tys_transparencies: &[Transparency],
    ast_ccf_tys: &[syn::Type],
    cst_bp: &syn::Path,
    cst_ccf: &syn::Type,
    cst: &syn::Path,
) -> syn::ItemFn {
    let byline = byline!();
    let cstfy_or_cstfytransparent = ccf_tys_transparencies
        .iter()
        .map(transparency2cstfication)
        .collect::<Vec<_>>();
    let top_level_cstfication = transparency2cstfication(transparency);
    let ok_expr: syn::Expr = match transparency {
        Transparency::Visible => {
            syn::parse_quote! { |x| tymetafuncspec_core::cstfy_ok(x, initial_offset, offset) }
        }
        Transparency::Transparent => {
            syn::parse_quote! { |x| tymetafuncspec_core::cstfy_transparent_ok(x) }
        }
    };
    syn::parse_quote! {
        #byline
        fn parse_directly(
            source: &str,
            initial_offset: parse::miette::SourceOffset,
            heap: &mut #cst_bp::Heap,
            errors: &mut Vec<parse::ParseError>,
            default_ret: parse::ParseError,
        ) -> (tymetafuncspec_core::#top_level_cstfication<#cst_bp::Heap, #cst>, parse::miette::SourceOffset) {
            match parse::unicode_segmentation::UnicodeSegmentation::unicode_words(&source[initial_offset.offset()..]).next() {
                Some(stringify!(#snake_ident)) => {
                    let mut offset = initial_offset;
                    let args = (
                        #({
                            let (res, new_offset) = <#ast_ccf_tys as parse::Parse<_>>::parse(source, offset, heap, errors);
                            offset = new_offset;
                            res
                        },)*
                    );
                    ((#ok_expr)(<#cst as term::CanonicallyConstructibleFrom<#cst_ccf>>::construct(heap, args)), offset)
                }
                None => (
                    tymetafuncspec_core::Either::Right(
                        std_parse_error::ParseError::new(
                            parse::ParseError::UnexpectedEndOfInput(initial_offset.into()),
                        ),
                        std::marker::PhantomData,
                    ),
                    initial_offset,
                ),
                _ => (
                    tymetafuncspec_core::Either::Right(
                        std_parse_error::ParseError::new(
                            default_ret,
                        ),
                        std::marker::PhantomData,
                    ),
                    initial_offset,
                ),
            }
        }
    }
}

fn transparency2cstfication(transparency: &Transparency) -> syn::Ident {
    match transparency {
        Transparency::Transparent => syn::parse_quote! { CstfyTransparent },
        Transparency::Visible => syn::parse_quote! { Cstfy },
    }
}

// pub fn generate_cst_mod<L: LangSpec>(bps: &BasePaths, lg: &LsGen<L>) -> syn::ItemMod {
//     let byline = byline!();
//     let base_path = &bps.cst;
//     let base_path = parse_quote! { #base_path::cst };
//     let bak = lg.bak();
//     let arena = bumpalo::Bump::new();
//     let cst = cst(&arena, bak);
//     let lg = LsGen::from(&cst);
//     let cst = term_specialized_gen::generate(&base_path, &lg, false);
//     parse_quote! {
//         #byline
//         pub mod cst {
//             #cst
//         }
//     }
// }

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
    // EverywhereAlternative {
    //     name: Name {
    //         human: "Cst".into(),
    //         camel: "Cst".into(),
    //         snake: "cst".into(),
    //     },
    //     l0: arena.alloc(EverywhereMaybeMore {
    //         name: Name {
    //             human: "MetadataAst".into(),
    //             camel: "MetadataAst".into(),
    //             snake: "metadata_ast".into(),
    //         },
    //         l0: l,
    //         l1: parse_metadata,
    //         l1_root: SortId::TyMetaFunc(MappedType {
    //             f: std_parse_metadata::ParseMetadataTmfId(),
    //             a: vec![],
    //         }),
    //     }),
    //     l1: errlang,
    //     l1_root: SortId::TyMetaFunc(MappedType {
    //         f: std_parse_error::ParseErrorTmfId(),
    //         a: vec![],
    //     }),
    // }
}

pub fn formatted<L: LangSpec>(l: &L) -> String {
    let lg = LsGen::from(l);
    let bps = BasePaths {
        data_structure: parse_quote! { crate },
        term_trait: parse_quote! { crate },
        cst: parse_quote! { crate },
    };
    let m = generate(&bps, &lg);
    let (cst_mod, cst_tt, cst_tt_impl) = {
        let arena = bumpalo::Bump::new();
        let cst = cst(&arena, l);
        let lg = LsGen::from(&cst);
        let bp = parse_quote! { crate::cst };
        let cst_mod = term_specialized_gen::generate(&bp, &lg, false);
        let cst_tt = term_trait_gen::generate(&bp, &cst);
        let cst_tt_impl = term_specialized_impl_gen::generate(
            &term_specialized_impl_gen::BasePaths {
                data_structure: bp.clone(),
                term_trait: bp.clone(),
            },
            &lg,
            &lg,
        );
        (cst_mod, cst_tt, cst_tt_impl)
    };
    // let ds = term_specialized_gen::generate(&bps.data_structure, &lg, false);
    // let tt = term_trait_gen::generate(&bps.term_trait, lg.bak());
    prettyplease::unparse(&syn::parse_quote! {
        #m
        pub mod cst {
            #cst_mod
            #cst_tt
            #cst_tt_impl
        }
        // #ds
        // #tt
        // #tt_impl
    })
}
