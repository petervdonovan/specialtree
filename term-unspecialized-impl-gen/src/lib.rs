use langspec::{
    flat::LangSpecFlat,
    humanreadable::LangSpecHuman,
    langspec::{AlgebraicSortId, LangSpec, TerminalLangSpec as _},
    tymetafunc::TyMetaFuncSpec,
};
use langspec_gen_util::{byline, number_range, AlgebraicsBasePath, HeapType, LsGen, TyGenData};

pub struct BasePaths {
    pub data_structure: syn::Path,
    pub term_trait: syn::Path,
    pub term_unspecialized: syn::Path,
}

pub fn generate<L: LangSpec>(
    bps: &BasePaths,
    data_structure_lsg: &LsGen<L>,
    term_trait_lsg: &LsGen<L>,
) -> syn::ItemMod {
    let owned_mod = gen_owned_mod(bps, data_structure_lsg);
    let ccf_mod = gen_ccf_mod(bps, data_structure_lsg, term_trait_lsg);
    let mct_mod = gen_mct_mod(bps, data_structure_lsg, term_trait_lsg);
    let heap_impl = gen_heap_impl(bps, data_structure_lsg, term_trait_lsg);
    let byline = byline!();
    syn::parse_quote! {
        #byline
        pub mod term_impls {
            #heap_impl
            #owned_mod
            #ccf_mod
            #mct_mod
        }
    }
}

pub(crate) fn gen_owned_mod<L: LangSpec>(
    BasePaths {
        data_structure,
        term_trait,
        ..
    }: &BasePaths,
    data_structure_lsg: &LsGen<L>,
) -> syn::ItemMod {
    let camels = data_structure_lsg
        .ty_gen_datas()
        .map(|td| td.camel_ident.clone());
    let byline = byline!();
    let tmfs = L::Tmfs::my_type();
    syn::parse_quote! {
        #byline
        pub mod owned_impls {
            #(impl #term_trait::extension_of::owned::#camels for term_unspecialized::Term<#tmfs> {})*
        }
    }
}

pub(crate) fn gen_ccf_mod<L: LangSpec>(
    bps: &BasePaths,
    data_structure_lsg: &LsGen<L>,
    term_trait_lsg: &LsGen<L>,
) -> syn::ItemMod {
    let ccf_impls = term_trait_lsg
        .ty_gen_datas()
        .map(|tgd| gen_ccf_impls(bps, &tgd));
    let byline = byline!();
    syn::parse_quote! {
        #byline
        pub mod ccf_impls {
        }
    }
}

pub(crate) fn gen_ccf_impls<L: LangSpec>(bps: &BasePaths, tgd: &TyGenData<L>) -> syn::ItemMod {
    let byline = byline!();
    let camel = &tgd.camel_ident;
    let snake = &tgd.snake_ident;
    let data_structure = &bps.data_structure;
    let term_trait = &bps.term_trait;
    let ccf_tys = (tgd.ccf.ccf_sort_tys)(
        HeapType(syn::parse_quote!(<Self as term::Heaped>::Heap)),
        AlgebraicsBasePath::new(
            syn::parse_quote!(<term_unspecialized::TermHeap as #term_trait::extension_of::Heap>::),
        ),
    );
    let ccf_tys_flattened = (tgd.ccf.flattened_ccf_sort_tys)(
        HeapType(syn::parse_quote!(<Self as term::Heaped>::Heap)),
        AlgebraicsBasePath::new(syn::parse_quote!(#data_structure::data_structure::)),
    );
    let ccf_sort_snake_idents = (tgd.ccf.ccf_sort_snake_idents)();
    let ccf_sort_camel_idents = (tgd.ccf.ccf_sort_camel_idents)();
    let ccf_impls = match tgd.id {
        None => vec![],
        Some(AlgebraicSortId::Product(_)) => vec![gen_ccf_impl_prod::<L>(
            bps,
            camel,
            ccf_tys.first().unwrap(),
            &ccf_tys_flattened,
            &ccf_sort_snake_idents,
        )],
        Some(AlgebraicSortId::Sum(_)) => ccf_tys
            .iter()
            .zip(ccf_sort_camel_idents.iter())
            .map(|(ccfty, ccf_camel)| gen_ccf_impl_sum::<L>(bps, camel, ccfty, ccf_camel))
            .collect(),
    };
    syn::parse_quote! {
        #byline
        pub mod #snake {
            #(#ccf_impls)*
        }
    }
}

pub(crate) fn gen_ccf_impl_prod<L: LangSpec>(
    BasePaths {
        data_structure,
        term_trait: _,
        ..
    }: &BasePaths,
    camel: &syn::Ident,
    ccf_ty: &syn::Type,
    ccf_tys_flattened: &[syn::Type],
    ccf_ty_snakes: &[syn::Ident],
) -> syn::ItemImpl {
    let number_range = number_range(ccf_tys_flattened.len());
    let byline = byline!();
    let tmfs = L::Tmfs::my_type();
    let ret = quote::quote! {
        #byline
        impl
            term::CanonicallyConstructibleFrom<#ccf_ty> for term_unspecialized::Term<#tmfs>
        {
            fn construct(
                heap: &mut Self::Heap,
                t: #ccf_ty,
            ) -> Self {
                #data_structure::data_structure::#camel {
                    #(
                        #ccf_ty_snakes: t.#number_range,
                    )*
                }
            }

            fn deconstruct(
                self,
            ) -> #ccf_ty {
                (#(
                    self.#ccf_ty_snakes,
                )*)
            }
        }
    };
    syn::parse_quote!(#ret)
}
pub(crate) fn gen_ccf_impl_sum<L: LangSpec>(
    BasePaths {
        data_structure,
        term_trait: _,
        ..
    }: &BasePaths,
    camel: &syn::Ident,
    ccf_ty: &syn::Type,
    ccf_ty_camel: &syn::Ident,
) -> syn::ItemImpl {
    let tmfs = L::Tmfs::my_type();
    let byline = byline!();
    syn::parse_quote! {
        #byline
        impl
            term::CanonicallyConstructibleFrom<#ccf_ty> for term_unspecialized::Term<#tmfs>
        {
            fn construct(
                heap: &mut Self::Heap,
                t: #ccf_ty,
            ) -> Self {
                #data_structure::data_structure::#camel::#ccf_ty_camel(t.0)
            }

            fn deconstruct(
                self,
            ) -> #ccf_ty {
                match self {
                    #data_structure::data_structure::#camel::#ccf_ty_camel(t) => (t,),
                    _ => panic!("conversion failure"),
                }
            }
        }
    }
}

pub(crate) fn gen_mct_mod<L: LangSpec>(
    bps: &BasePaths,
    data_structure_lsg: &LsGen<L>,
    term_trait_lsg: &LsGen<L>,
) -> syn::ItemMod {
    let byline = byline!();
    syn::parse_quote! {
        #byline
        pub mod mct_impls {}
    }
}

pub(crate) fn gen_heap_impl<L: LangSpec>(
    BasePaths {
        data_structure,
        term_trait,
        ..
    }: &BasePaths,
    _data_structure_lsg: &LsGen<L>,
    term_trait_lsg: &LsGen<L>,
) -> syn::ItemImpl {
    let byline = byline!();
    let camels = term_trait_lsg
        .ty_gen_datas()
        .map(|td| td.camel_ident.clone());
    let tmfs = L::Tmfs::my_type();
    let lit_fingerprints = term_trait_lsg
        .ty_gen_datas()
        .map(|td| td.fingerprint.lit_int());
    syn::parse_quote! {
        #byline
        impl #term_trait::extension_of::Heap for term_unspecialized::TermHeap {
            #(type #camels = term_unspecialized::Term<#lit_fingerprints, #tmfs>;)*
        }
    }
}

pub fn formatted<Tmfs: TyMetaFuncSpec>(lsh: &LangSpecHuman<Tmfs>) -> String {
    let lsf: LangSpecFlat<Tmfs> = LangSpecFlat::canonical_from(lsh);
    let lsg = LsGen::from(&lsf);
    let bps = BasePaths {
        data_structure: syn::parse_quote!(crate),
        term_trait: syn::parse_quote!(crate),
        term_unspecialized: syn::parse_quote!(crate),
    };
    let m = generate(&bps, &lsg, &lsg);
    let tt = term_trait_gen::generate(&bps.term_trait, &lsf);
    prettyplease::unparse(&syn::parse_quote! {
        #m
        #tt
    })
}
