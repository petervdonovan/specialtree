use extension_everywhere_alternative::EverywhereAlternative;
use extension_everywhere_maybemore::EverywhereMaybeMore;
use langspec::langspec::{AlgebraicSortId, LangSpec, Name, SortId};
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
    let parses = lg
        .ty_gen_datas()
        .zip(lg_cst.ty_gen_datas())
        .map(|(ast_tgd, cst_tgd)| generate_parse(bps, &ast_tgd, &cst_tgd));
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
    let snake_ident = &tgd.snake_ident;
    let camel_ident = &tgd.camel_ident;
    let byline = byline!();
    let cst_bp = &bps.cst;
    let cst_bp: syn::Path = parse_quote! { #cst_bp::cst::data_structure };
    let my_ty: syn::Type = syn::parse_quote! { #cst_bp::#camel_ident };
    let cst: syn::Path = parse_quote! { #cst_bp::#camel_ident };
    let ccf_snakes = (tgd.ccf.ccf_sort_snake_idents)();
    let ccf_snakes_size1 =
        ccf_snakes
            .iter()
            .filter_map(|it| if it.len() == 1 { Some(&it[0]) } else { None });
    let cst_ccf_sort_tys = (cst_tgd.ccf.ccf_sort_tys)(
        HeapType(syn::parse_quote! { #cst_bp::Heap }),
        AlgebraicsBasePath::new(quote::quote!(#cst_bp::)),
    );
    let maybe_parse_this = if ccf_snakes.len() == 1 {
        let cst_ccf = cst_ccf_sort_tys.first().unwrap().clone();
        let ccf_tys = (cst_tgd.ccf.ccf_sort_tyses)(
            HeapType(syn::parse_quote! { #cst_bp::Heap }),
            AlgebraicsBasePath::new(quote::quote!(#cst_bp::)),
        )
        .into_iter()
        .next()
        .unwrap();
        let parse_directly = gen_parse_current(
            snake_ident,
            &ccf_tys[0..ccf_tys.len() - 1],
            &cst_bp,
            &cst_ccf,
            &cst,
        );
        quote::quote! {
            #parse_directly
        }
    } else {
        quote::quote! {let parse_directly = |_,_,_,_, ret| ret}
    };
    let ccf_tys_size1 = cst_ccf_sort_tys
        .into_iter()
        .zip(ccf_snakes.iter())
        .filter_map(|(it, snake)| if snake.len() == 1 { Some(it) } else { None });
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
    syn::parse_quote! {
        #byline
        impl parse::Parse for #my_ty {
            fn parse(source: &str, offset: parse::miette::SourceOffset, heap: &mut #cst_bp::Heap, errors: &mut Vec<parse::ParseError>) -> Result<(#cst, parse::miette::SourceOffset), parse::ParseError> {
                let mut ute = None;
                #maybe_parse_this ;
                #(
                    match <#untupled_ccf_tys_size1 as parse::Parse>::parse(source, offset, heap, errors) {
                        Ok((cst, offset)) => return Ok((
                            <#my_ty as term::CanonicallyConstructibleFrom<#ccf_tys_size1>>::construct(heap, (tymetafuncspec_core::Either::Left(cst, std::marker::PhantomData), tymetafuncspec_core::Maybe::Nothing)),
                            offset
                        )),
                        Err(e) => {
                            ute = parse::ParseError::merge_over(e, ute);
                        }
                    }
                )*
                parse_directly(source, offset, heap, errors, Err(ute.unwrap()))
            }
        }
    }
}

pub fn gen_parse_current(
    snake_ident: &syn::Ident,
    ccf_tys: &[syn::Type],
    cst_bp: &syn::Path,
    cst_ccf: &syn::Type,
    cst: &syn::Path,
) -> syn::ItemFn {
    let byline = byline!();
    syn::parse_quote! {
        #byline
        fn parse_directly(
            source: &str,
            offset: parse::miette::SourceOffset,
            heap: &mut #cst_bp::Heap,
            errors: &mut Vec<parse::ParseError>,
            default_ret: Result<(#cst, parse::miette::SourceOffset), parse::ParseError>,
        ) -> Result<(#cst, parse::miette::SourceOffset), parse::ParseError> {
            match parse::unicode_segmentation::UnicodeSegmentation::unicode_words(&source[offset.offset()..]).next() {
                Some(stringify!(#snake_ident)) => {
                    let mut offset = offset;
                    let args = (
                        #({
                            let res = <#ccf_tys as parse::Parse>::parse(source, offset, heap, errors);
                            match res {
                                Ok((cst, new_offset)) => {
                                    offset = new_offset;
                                    cst
                                }
                                Err(e) => {
                                    errors.push(e);
                                    tymetafuncspec_core::Either::Right(
                                        <#cst_bp::Error as term::CanonicallyConstructibleFrom<(tymetafuncspec_core::Maybe<#cst_bp::Heap, #cst_bp::Metadata>,)>>::construct(
                                            heap,
                                            (tymetafuncspec_core::Maybe::Nothing,)
                                        ),
                                        std::marker::PhantomData
                                    )
                                }
                            }
                        },)*
                        tymetafuncspec_core::Maybe::Nothing,
                    );
                    Ok((<#cst as term::CanonicallyConstructibleFrom<#cst_ccf>>::construct(heap, args), offset))
                }
                None => Err(parse::ParseError::UnexpectedEndOfInput(offset.into())),
                _ => default_ret,
            }
        }
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
        l1_root: errlang
            .products()
            .find(|it| errlang.product_name(*it).snake == "error")
            .map(|it| SortId::Algebraic(AlgebraicSortId::Product(it)))
            .unwrap()
            .clone(),
    });
    let parse_metadata = arena.alloc(std_parse_metadata::parse_metadata());
    let cst = EverywhereMaybeMore {
        name: Name {
            human: "Cst".into(),
            camel: "Cst".into(),
            snake: "cst".into(),
        },
        l0: fallible_ast,
        l1: parse_metadata,
        l1_root: parse_metadata
            .products()
            .find(|it| parse_metadata.product_name(*it).snake == "metadata")
            .map(|it| SortId::Algebraic(AlgebraicSortId::Product(it)))
            .unwrap()
            .clone(),
    };
    cst
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
