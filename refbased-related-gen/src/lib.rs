use langdatastructure_gen::refbased::sort2rs_type;
use langspec::{
    flat::LangSpecFlat,
    humanreadable::LangSpecHuman,
    langspec::{LangSpec, TerminalLangSpec as _},
};
use langspec_gen_util::{transpose, LangSpecGen, ProdGenData, SumGenData};

pub struct BasePaths {
    pub extension_of: syn::Path,
    pub data_structure: syn::Path,
}

pub fn generate<L: LangSpec>(bps: &BasePaths, ls: &L) -> syn::Item {
    let ls = LangSpecGen {
        bak: ls,
        sort2rs_type,
        type_base_path: bps.data_structure.clone(),
    };
    let limpl = limpl(bps, &ls);
    let owned = owned::generate(bps, &ls);
    let reference = reference::generate(bps, &ls);
    let mut_reference = mut_reference::generate(bps, &ls);
    let byline = langspec_gen_util::byline!();
    syn::parse_quote!(
        #byline
        pub mod refbased_extension_of {
            #limpl
            #owned
            #reference
            #mut_reference
        }
    )
}

fn limpl<L: LangSpec>(
    BasePaths {
        extension_of,
        data_structure,
        ..
    }: &BasePaths,
    ls: &LangSpecGen<L>,
) -> syn::Item {
    transpose!(ls.prod_gen_datas(), camel_ident, rs_ty);
    let (prod_idents, prod_rs_tys) = (camel_ident, rs_ty);
    transpose!(ls.sum_gen_datas(), camel_ident, rs_ty);
    let (sum_idents, sum_rs_tys) = (camel_ident, rs_ty);
    let camel_ident = ls.camel_ident();
    let byline = langspec_gen_util::byline!();
    syn::parse_quote! {
        #byline
        impl #extension_of::LImpl for #data_structure::#camel_ident {
            type NatLit = #data_structure::NatLit;
            #(
                type #prod_idents = #data_structure::#prod_rs_tys;
            )*
            #(
                type #sum_idents = #data_structure::#sum_rs_tys;
            )*
        }
    }
}
mod owned {

    use super::*;
    pub fn generate<L: LangSpec>(bps: &BasePaths, ls: &LangSpecGen<L>) -> syn::Item {
        let prods = gen_prods(bps, ls);
        let sums = gen_sums(bps, ls);
        let nat_lit = gen_nat_lit(bps, ls);
        let byline = langspec_gen_util::byline!();
        syn::parse_quote!(
            #byline
            pub mod owned {
                #nat_lit
                #(
                    #prods
                )*
                #(
                    #sums
                )*
            }
        )
    }
    pub fn gen_nat_lit<'c: 'd, 'd, L: LangSpec>(
        BasePaths {
            extension_of,
            data_structure,
            ..
        }: &'d BasePaths,
        ls: &'c LangSpecGen<L>,
    ) -> syn::ItemImpl {
        let ls_camel_ident = ls.camel_ident();
        let byline = langspec_gen_util::byline!();
        syn::parse_quote! {
            #byline
            impl #extension_of::owned::NatLit for #data_structure::NatLit {
                type LImpl = #data_structure::#ls_camel_ident;
                fn get_ref<'a, 'b: 'a>(&'a self, _l: &'b Self::LImpl) -> impl #extension_of::reference::NatLit<'a, LImpl = Self::LImpl> {
                    *self
                }
                fn get_mut<'a, 'b: 'a>(&'a mut self, _l: &'b mut Self::LImpl) -> impl #extension_of::mut_reference::NatLit<'a, LImpl = Self::LImpl> {
                    self
                }
            }
        }
    }
    pub fn gen_prods<'c: 'd, 'd, L: LangSpec>(
        BasePaths {
            extension_of,
            data_structure,
            ..
        }: &'d BasePaths,
        ls: &'c LangSpecGen<L>,
    ) -> impl Iterator<Item = syn::ItemImpl> + 'd {
        let ls_camel_ident = ls.camel_ident();
        ls.prod_gen_datas()
            .map(
                move |ProdGenData {
                     camel_ident,
                     rs_ty,
                     n_sorts,
                     sort_rs_camel_idents,
                     ..
                 }| {
                    let arg_idxs = (0..n_sorts).map(syn::Index::from);
                    let byline = langspec_gen_util::byline!();
                    syn::parse_quote! {
                        #byline
                        impl #extension_of::owned::#camel_ident for #data_structure::#rs_ty {
                            type LImpl = #data_structure::#ls_camel_ident;
                            fn new(_l: &mut Self::LImpl, args: (#(<Self::LImpl as #extension_of::LImpl>::#sort_rs_camel_idents,)*)) -> Self {
                                Self(#(args.#arg_idxs),*)
                            }
                            fn get_ref<'a, 'b: 'a>(&'a self, _l: &'b Self::LImpl) -> impl #extension_of::reference::#camel_ident<'a, LImpl = Self::LImpl> {
                                self
                            }
                            fn get_mut<'a, 'b: 'a>(&'a mut self, _l: &'b mut Self::LImpl) -> impl #extension_of::mut_reference::#camel_ident<'a, LImpl = Self::LImpl> {
                                self
                            }
                        }
                    }
                },
            )
    }
    pub fn gen_sums<'c: 'd, 'd, L: LangSpec>(
        BasePaths {
            extension_of,
            data_structure,
            ..
        }: &'d BasePaths,
        ls: &'c LangSpecGen<L>,
    ) -> impl Iterator<Item = syn::ItemImpl> + 'd {
        let ls_camel_ident = ls.camel_ident();
        ls.sum_gen_datas()
            .map(move |SumGenData { camel_ident, snake_ident,
                sort_rs_camel_idents,
                sort_rs_snake_idents, sort_shapes, .. }| {
                let byline = langspec_gen_util::byline!();
                let maybe_boxed_from = sort_shapes.map(|shape| match shape {
                    langspec::langspec::SortId::Algebraic(_) => quote::quote!(Box::new),
                    _ => quote::quote!(|a| a),
                });
                syn::parse_quote! {
                    #byline
                    impl #extension_of::owned::#camel_ident for #data_structure::#camel_ident {
                        type LImpl = #data_structure::#ls_camel_ident;
                        #(
                            fn #sort_rs_snake_idents(_l: &mut Self::LImpl, from: <Self::LImpl as #extension_of::LImpl>::#sort_rs_camel_idents) -> Self {
                                Self::#sort_rs_camel_idents((#maybe_boxed_from)(from))
                            }
                        )*
                        fn get_ref(&self, _l: &Self::LImpl) -> impl #extension_of::reference::#camel_ident<'_, LImpl = Self::LImpl> {
                            self
                        }
                        fn get_mut(&mut self, _l: &mut Self::LImpl) -> impl #extension_of::mut_reference::#camel_ident<'_, LImpl = Self::LImpl> {
                            self
                        }
                    }
                }
            })
    }
}

pub fn formatted(lsh: &LangSpecHuman) -> String {
    let lsf: LangSpecFlat = LangSpecFlat::canonical_from(lsh);
    let bps = BasePaths {
        extension_of: syn::parse_quote!(crate::extension_of),
        data_structure: syn::parse_quote!(crate::data_structure::refbased),
    };
    let m = generate(&bps, &lsf);
    let extension_of = related_gen::generate(&bps.extension_of, &lsf);
    let data_structure = langdatastructure_gen::refbased::generate(
        &syn::parse_quote!(crate::data_structure),
        &lsf,
        false,
    );
    prettyplease::unparse(&syn::parse_quote!(
        #m
        #extension_of
        pub mod data_structure {
            #data_structure
        }
    ))
}

mod reference {

    use langspec::langspec::AlgebraicSortId;
    use langspec_gen_util::collect;

    use super::*;
    pub fn generate<L: LangSpec>(bps: &BasePaths, ls: &LangSpecGen<L>) -> syn::Item {
        let prods = gen_prods(bps, ls);
        let sums = gen_sums(bps, ls);
        let BasePaths {
            extension_of,
            data_structure,
            ..
        } = bps;
        let ls_camel_ident = ls.camel_ident();
        let byline = langspec_gen_util::byline!();
        syn::parse_quote!(
            #byline
            pub mod reference {
                impl<'a> #extension_of::reference::NatLit<'a> for #data_structure::NatLit {
                    type LImpl = #data_structure::#ls_camel_ident;
                    fn is_eq<'b: 'a>(self, _l: &'b Self::LImpl, other: Self) -> bool {
                        self == other
                    }
                }
                #(
                    #prods
                )*
                #(
                    #sums
                )*
            }
        )
    }
    pub fn camels2rs_ty(
        data_structure: syn::Path,
        camels: impl Iterator<Item = syn::Ident>,
        shapes: impl Iterator<Item = langspec::langspec::SortId<AlgebraicSortId<(), ()>>>,
    ) -> impl Iterator<Item = syn::Type> {
        camels.zip(shapes).map(move |(camel, shape)| match shape {
            langspec::langspec::SortId::NatLiteral => {
                syn::parse_quote!(#data_structure::NatLit)
            }
            langspec::langspec::SortId::Algebraic(_) => {
                syn::parse_quote!(&'a #data_structure::#camel)
            }
            _ => {
                todo!()
            }
        })
    }
    pub fn gen_prods<'c: 'd, 'd, L: LangSpec>(
        BasePaths {
            extension_of,
            data_structure,
            ..
        }: &'d BasePaths,
        ls: &'c LangSpecGen<L>,
    ) -> impl Iterator<Item = syn::ItemImpl> + 'd {
        let ls_camel_ident = ls.camel_ident();
        ls.prod_gen_datas().map(
            move |ProdGenData {
                 camel_ident,
                 rs_ty,
                 sort_rs_camel_idents,
                 ty_idx,
                 idx,
                 ..
             }| {
                collect!(sort_rs_camel_idents);
                let mut sort_rs_camel_idents_deduplicated = sort_rs_camel_idents.iter().collect::<std::collections::HashSet<_>>().into_iter().collect::<Vec<_>>();
                sort_rs_camel_idents_deduplicated.sort();
                let byline = langspec_gen_util::byline!();
                syn::parse_quote! {
                    #byline
                    impl<'a> #extension_of::reference::#camel_ident<'a> for &'a #data_structure::#rs_ty {
                        type LImpl = #data_structure::#ls_camel_ident;
                        #(
                            type #ty_idx = &'a #data_structure::#sort_rs_camel_idents;
                        )*
                        fn is_eq<'b: 'a>(self, l: &'b Self::LImpl, other: Self) -> bool {
                            #(
                                use #extension_of::reference::#sort_rs_camel_idents_deduplicated;
                            )*
                            let mut ret = true;
                            #({
                                let mine = <Self as #extension_of::Projection<Self::LImpl, #idx>>::project(self, l);
                                let their_data = <Self as #extension_of::Projection<Self::LImpl, #idx>>::project(other, l);
                                ret = ret && mine.is_eq(l, their_data);
                            })*
                            ret
                        }
                    }
                }
            },
        ).chain(ls.prod_gen_datas().flat_map(
            move |ProdGenData {
                 snake_ident,
                 rs_ty,
                 sort_rs_camel_idents,
                 idx,
                 sort_shapes,
                 ..
             }| {
                let ls_camel_ident = ls.camel_ident();
                collect!(idx, sort_shapes, sort_rs_camel_idents);
                let accessors = sort_shapes.iter().zip(idx.clone()).map(|(shape, idx)| -> syn::Expr {
                    match shape {
                        langspec::langspec::SortId::NatLiteral => {
                            syn::parse_quote!(self.idx)
                        }
                        _ => {
                            syn::parse_quote!(&self.#idx)
                        }
                    }
                }).collect::<Vec<_>>();
                let tys = camels2rs_ty(data_structure.clone(), sort_rs_camel_idents.clone().into_iter(), sort_shapes.into_iter());
                idx.into_iter().zip(sort_rs_camel_idents).zip(tys.zip(accessors)).map(
                    move |((idx, sort_rs_camel_idents), (ty, accessor))|{
                        let ret: syn::ItemImpl = syn::parse_quote! {
                            impl<'a> #extension_of::Projection<#data_structure::#ls_camel_ident, #idx> for &'a #data_structure::#rs_ty {
                                type To = #ty;
                                fn project(self, _l: &#data_structure::#ls_camel_ident) -> Self::To {
                                    #accessor
                                }
                            }
                        };
                        ret
                    }
                )
            },
        ))
    }
    pub fn gen_sums<'c: 'd, 'd, L: LangSpec>(
        BasePaths {
            extension_of,
            data_structure,
            ..
        }: &'d BasePaths,
        ls: &'c LangSpecGen<L>,
    ) -> impl Iterator<Item = syn::ItemImpl> + 'd {
        let ls_camel_ident = ls.camel_ident();
        ls.sum_gen_datas().map(
            move |SumGenData {
                 camel_ident,
                 snake_ident,
                 rs_ty,
                 sort_rs_camel_idents,
                 sort_rs_snake_idents,
                    sort_shapes,
                 ..
             }| {
                let byline = langspec_gen_util::byline!();
                collect!(sort_rs_camel_idents, sort_shapes);
                let tys = camels2rs_ty(data_structure.clone(), sort_rs_camel_idents.clone().into_iter(), sort_shapes.clone().into_iter());
                let accessors = sort_shapes.iter().map(|shape| -> syn::Expr {match shape {
                    langspec::langspec::SortId::NatLiteral => {
                        syn::parse_quote!(*a)
                    }
                    _ => {
                        syn::parse_quote!(a)
                    }
                }});
                collect!(accessors);
                let ret = quote::quote! {
                    #byline
                    impl<'a> #extension_of::reference::#camel_ident<'a> for &'a #data_structure::#rs_ty {
                        type LImpl = #data_structure::#ls_camel_ident;
                        #(
                            type #sort_rs_camel_idents = #tys;
                        )*
                        #(
                            fn #sort_rs_snake_idents<'b: 'a>(self, _l: &'b Self::LImpl) -> Option<Self::#sort_rs_camel_idents> {
                                match self {
                                    #data_structure::#rs_ty::#sort_rs_camel_idents(a) => Some(#accessors),
                                    _ => None,
                                }
                            }
                        )*
                        fn is_eq<'b: 'a>(self, l: &'b Self::LImpl, other: Self) -> bool {
                            #(
                                use #extension_of::reference::#sort_rs_camel_idents;
                            )*
                            match (self, other) {
                                #(
                                    (#data_structure::#rs_ty::#sort_rs_camel_idents(b), #data_structure::#rs_ty::#sort_rs_camel_idents(a)) => {
                                        b.is_eq(l, #accessors)
                                    }
                                )*
                                _ => false,
                            }
                        }
                    }
                };
                syn::parse_quote!(#ret)
            },
        )
    }
}

mod mut_reference {

    use langspec_gen_util::collect;

    use super::*;
    pub fn generate<L: LangSpec>(bps: &BasePaths, ls: &LangSpecGen<L>) -> syn::Item {
        let prods = gen_prods(bps, ls);
        let sums = gen_sums(bps, ls);
        let nat_lit = gen_nat_lit(bps, ls);
        let byline = langspec_gen_util::byline!();
        syn::parse_quote!(
            #byline
            pub mod mut_reference {
                #nat_lit
                #(
                    #prods
                )*
                #(
                    #sums
                )*
            }
        )
    }
    pub fn gen_nat_lit<'c: 'd, 'd, L: LangSpec>(
        BasePaths {
            extension_of,
            data_structure,
            ..
        }: &'d BasePaths,
        ls: &'c LangSpecGen<L>,
    ) -> syn::ItemImpl {
        let ls_camel_ident = ls.camel_ident();
        let byline = langspec_gen_util::byline!();
        syn::parse_quote! {
            #byline
            impl<'a> #extension_of::mut_reference::NatLit<'a> for &'a mut #data_structure::NatLit {
                type LImpl = #data_structure::#ls_camel_ident;
            }
        }
    }
    pub fn gen_prods<'c: 'd, 'd, L: LangSpec>(
        BasePaths {
            extension_of,
            data_structure,
            ..
        }: &'d BasePaths,
        ls: &'c LangSpecGen<L>,
    ) -> impl Iterator<Item = syn::ItemImpl> + 'd {
        let ls_camel_ident = ls.camel_ident();
        ls.prod_gen_datas().map(
            move |ProdGenData {
                 camel_ident,
                 rs_ty,
                 sort_rs_camel_idents,
                 ty_idx,
                 ..
             }| {
                let byline = langspec_gen_util::byline!();
                syn::parse_quote! {
                    #byline
                    impl<'a> #extension_of::mut_reference::#camel_ident<'a> for &'a mut #data_structure::#rs_ty {
                        type LImpl = #data_structure::#ls_camel_ident;
                        #(
                            type #ty_idx = &'a mut #data_structure::#sort_rs_camel_idents;
                        )*
                    }
                }
            },
        ).chain(ls.prod_gen_datas().flat_map(
            move |ProdGenData {
                 rs_ty,
                 sort_rs_camel_idents,
                 idx,
                 ..
             }| {
                let ls_camel_ident = ls.camel_ident();
                collect!(idx, sort_rs_camel_idents);
                idx.into_iter().zip(sort_rs_camel_idents).map(
                    move |(idx, sort_rs_camel_idents)|{
                        let ret: syn::ItemImpl = syn::parse_quote! {
                            impl<'a> #extension_of::Projection<#data_structure::#ls_camel_ident, #idx> for &'a mut #data_structure::#rs_ty {
                                type To = &'a mut #data_structure::#sort_rs_camel_idents;
                                fn project(self, _l: &#data_structure::#ls_camel_ident) -> Self::To {
                                    &mut self.#idx
                                }
                            }
                        };
                        ret
                    }
                )
            },
        ))
    }
    pub fn gen_sums<'c: 'd, 'd, L: LangSpec>(
        BasePaths {
            extension_of,
            data_structure,
            ..
        }: &'d BasePaths,
        ls: &'c LangSpecGen<L>,
    ) -> impl Iterator<Item = syn::ItemImpl> + 'd {
        let ls_camel_ident = ls.camel_ident();
        ls.sum_gen_datas().map(move |SumGenData {
            camel_ident,
            snake_ident,
            rs_ty,
            sort_rs_camel_idents,
            sort_rs_snake_idents,
            ..
        }| {
            collect!(sort_rs_camel_idents);
            let byline = langspec_gen_util::byline!();
            syn::parse_quote! {
                #byline
                impl<'a> #extension_of::mut_reference::#camel_ident<'a> for &'a mut #data_structure::#rs_ty {
                    type LImpl = #data_structure::#ls_camel_ident;
                    type Owned = #data_structure::#camel_ident;
                    #(
                        type #sort_rs_camel_idents = &'a mut #data_structure::#sort_rs_camel_idents;
                    )*
                    #(
                        fn #sort_rs_snake_idents<'b: 'a>(self, _l: &'b Self::LImpl) -> Option<Self::#sort_rs_camel_idents> {
                            match self {
                                #data_structure::#rs_ty::#sort_rs_camel_idents(a) => Some(a),
                                _ => None,
                            }
                        }
                    )*
                    fn set<'b: 'a>(
                        self,
                        _l: &'b mut Self::LImpl,
                        value: Self::Owned,
                    ) {
                        *self = value;
                    }
                }
            }
        })
    }
}
