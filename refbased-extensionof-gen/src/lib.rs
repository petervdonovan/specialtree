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

pub fn gen<L: LangSpec>(bps: &BasePaths, ls: &L) -> syn::Item {
    let ls = LangSpecGen {
        bak: ls,
        sort2rs_type,
        type_base_path: bps.data_structure.clone(),
    };
    let limpl = limpl(bps, &ls);
    let owned = owned::gen(bps, &ls);
    let reference = reference::gen(bps, &ls);
    let mut_reference = mut_reference::gen(bps, &ls);
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
    pub fn gen<L: LangSpec>(bps: &BasePaths, ls: &LangSpecGen<L>) -> syn::Item {
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
    let m = gen(&bps, &lsf);
    let extension_of = extensionof_gen::gen(&bps.extension_of, &lsf);
    let data_structure = langdatastructure_gen::refbased::gen(
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
    pub fn gen<L: LangSpec>(bps: &BasePaths, ls: &LangSpecGen<L>) -> syn::Item {
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
    pub fn gen<L: LangSpec>(bps: &BasePaths, ls: &LangSpecGen<L>) -> syn::Item {
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_refbased_extension_of() {
        let formatted = formatted(&langspec_examples::fib());
        let expected = expect_test::expect![[r#"
            /// generated by [refbased_extensionof_gen::gen]
            pub mod refbased_extension_of {
                /// generated by [refbased_extensionof_gen::limpl]
                impl crate::extension_of::LImpl for crate::data_structure::refbased::Fib {
                    type NatLit = crate::data_structure::refbased::NatLit;
                    type Plus = crate::data_structure::refbased::Plus;
                    type F = crate::data_structure::refbased::F;
                    type Nat = crate::data_structure::refbased::Nat;
                }
                /// generated by [refbased_extensionof_gen::owned::gen]
                pub mod owned {
                    impl crate::extension_of::owned::NatLit
                    for crate::data_structure::refbased::NatLit {
                        type LImpl = crate::data_structure::refbased::Fib;
                    }
                    /// generated by [refbased_extensionof_gen::owned::gen_prods]
                    impl crate::extension_of::owned::Plus for crate::data_structure::refbased::Plus {
                        type LImpl = crate::data_structure::refbased::Fib;
                        fn new(
                            _l: &mut Self::LImpl,
                            args: (
                                <Self::LImpl as crate::extension_of::LImpl>::Nat,
                                <Self::LImpl as crate::extension_of::LImpl>::Nat,
                            ),
                        ) -> Self {
                            Self(args.0, args.1)
                        }
                        fn get_ref<'a, 'b: 'a>(
                            &'a self,
                            _l: &'b Self::LImpl,
                        ) -> impl crate::extension_of::reference::Plus<'a, LImpl = Self::LImpl> {
                            self
                        }
                        fn get_mut<'a, 'b: 'a>(
                            &'a mut self,
                            _l: &'b mut Self::LImpl,
                        ) -> impl crate::extension_of::mut_reference::Plus<'a, LImpl = Self::LImpl> {
                            self
                        }
                    }
                    /// generated by [refbased_extensionof_gen::owned::gen_prods]
                    impl crate::extension_of::owned::F for crate::data_structure::refbased::F {
                        type LImpl = crate::data_structure::refbased::Fib;
                        fn new(
                            _l: &mut Self::LImpl,
                            args: (<Self::LImpl as crate::extension_of::LImpl>::Nat,),
                        ) -> Self {
                            Self(args.0)
                        }
                        fn get_ref<'a, 'b: 'a>(
                            &'a self,
                            _l: &'b Self::LImpl,
                        ) -> impl crate::extension_of::reference::F<'a, LImpl = Self::LImpl> {
                            self
                        }
                        fn get_mut<'a, 'b: 'a>(
                            &'a mut self,
                            _l: &'b mut Self::LImpl,
                        ) -> impl crate::extension_of::mut_reference::F<'a, LImpl = Self::LImpl> {
                            self
                        }
                    }
                    /// generated by [refbased_extensionof_gen::owned::gen_sums]
                    impl crate::extension_of::owned::Nat for crate::data_structure::refbased::Nat {
                        type LImpl = crate::data_structure::refbased::Fib;
                        fn nat_lit(
                            _l: &mut Self::LImpl,
                            from: <Self::LImpl as crate::extension_of::LImpl>::NatLit,
                        ) -> Self {
                            Self::NatLit((|a| a)(from))
                        }
                        fn f(
                            _l: &mut Self::LImpl,
                            from: <Self::LImpl as crate::extension_of::LImpl>::F,
                        ) -> Self {
                            Self::F((Box::new)(from))
                        }
                        fn plus(
                            _l: &mut Self::LImpl,
                            from: <Self::LImpl as crate::extension_of::LImpl>::Plus,
                        ) -> Self {
                            Self::Plus((Box::new)(from))
                        }
                        fn get_ref(
                            &self,
                            _l: &Self::LImpl,
                        ) -> impl crate::extension_of::reference::Nat<'_, LImpl = Self::LImpl> {
                            self
                        }
                        fn get_mut(
                            &mut self,
                            _l: &mut Self::LImpl,
                        ) -> impl crate::extension_of::mut_reference::Nat<'_, LImpl = Self::LImpl> {
                            self
                        }
                    }
                }
                /// generated by [refbased_extensionof_gen::reference::gen]
                pub mod reference {
                    impl<'a> crate::extension_of::reference::NatLit<'a>
                    for crate::data_structure::refbased::NatLit {
                        type LImpl = crate::data_structure::refbased::Fib;
                        fn is_eq<'b: 'a>(self, _l: &'b Self::LImpl, other: Self) -> bool {
                            self == other
                        }
                    }
                    /// generated by [refbased_extensionof_gen::reference::gen_prods]
                    impl<'a> crate::extension_of::reference::Plus<'a>
                    for &'a crate::data_structure::refbased::Plus {
                        type LImpl = crate::data_structure::refbased::Fib;
                        type T0 = &'a crate::data_structure::refbased::Nat;
                        type T1 = &'a crate::data_structure::refbased::Nat;
                        fn is_eq<'b: 'a>(self, l: &'b Self::LImpl, other: Self) -> bool {
                            use crate::extension_of::reference::Nat;
                            let mut ret = true;
                            {
                                let mine = <Self as crate::extension_of::Projection<
                                    Self::LImpl,
                                    0,
                                >>::project(self, l);
                                let their_data = <Self as crate::extension_of::Projection<
                                    Self::LImpl,
                                    0,
                                >>::project(other, l);
                                ret = ret && mine.is_eq(l, their_data);
                            }
                            {
                                let mine = <Self as crate::extension_of::Projection<
                                    Self::LImpl,
                                    1,
                                >>::project(self, l);
                                let their_data = <Self as crate::extension_of::Projection<
                                    Self::LImpl,
                                    1,
                                >>::project(other, l);
                                ret = ret && mine.is_eq(l, their_data);
                            }
                            ret
                        }
                    }
                    /// generated by [refbased_extensionof_gen::reference::gen_prods]
                    impl<'a> crate::extension_of::reference::F<'a>
                    for &'a crate::data_structure::refbased::F {
                        type LImpl = crate::data_structure::refbased::Fib;
                        type T0 = &'a crate::data_structure::refbased::Nat;
                        fn is_eq<'b: 'a>(self, l: &'b Self::LImpl, other: Self) -> bool {
                            use crate::extension_of::reference::Nat;
                            let mut ret = true;
                            {
                                let mine = <Self as crate::extension_of::Projection<
                                    Self::LImpl,
                                    0,
                                >>::project(self, l);
                                let their_data = <Self as crate::extension_of::Projection<
                                    Self::LImpl,
                                    0,
                                >>::project(other, l);
                                ret = ret && mine.is_eq(l, their_data);
                            }
                            ret
                        }
                    }
                    impl<'a> crate::extension_of::Projection<crate::data_structure::refbased::Fib, 0>
                    for &'a crate::data_structure::refbased::Plus {
                        type To = &'a crate::data_structure::refbased::Nat;
                        fn project(self, _l: &crate::data_structure::refbased::Fib) -> Self::To {
                            &self.0
                        }
                    }
                    impl<'a> crate::extension_of::Projection<crate::data_structure::refbased::Fib, 1>
                    for &'a crate::data_structure::refbased::Plus {
                        type To = &'a crate::data_structure::refbased::Nat;
                        fn project(self, _l: &crate::data_structure::refbased::Fib) -> Self::To {
                            &self.1
                        }
                    }
                    impl<'a> crate::extension_of::Projection<crate::data_structure::refbased::Fib, 0>
                    for &'a crate::data_structure::refbased::F {
                        type To = &'a crate::data_structure::refbased::Nat;
                        fn project(self, _l: &crate::data_structure::refbased::Fib) -> Self::To {
                            &self.0
                        }
                    }
                    /// generated by [refbased_extensionof_gen::reference::gen_sums]
                    impl<'a> crate::extension_of::reference::Nat<'a>
                    for &'a crate::data_structure::refbased::Nat {
                        type LImpl = crate::data_structure::refbased::Fib;
                        type NatLit = crate::data_structure::refbased::NatLit;
                        type F = &'a crate::data_structure::refbased::F;
                        type Plus = &'a crate::data_structure::refbased::Plus;
                        fn nat_lit<'b: 'a>(self, _l: &'b Self::LImpl) -> Option<Self::NatLit> {
                            match self {
                                crate::data_structure::refbased::Nat::NatLit(a) => Some(*a),
                                _ => None,
                            }
                        }
                        fn f<'b: 'a>(self, _l: &'b Self::LImpl) -> Option<Self::F> {
                            match self {
                                crate::data_structure::refbased::Nat::F(a) => Some(a),
                                _ => None,
                            }
                        }
                        fn plus<'b: 'a>(self, _l: &'b Self::LImpl) -> Option<Self::Plus> {
                            match self {
                                crate::data_structure::refbased::Nat::Plus(a) => Some(a),
                                _ => None,
                            }
                        }
                        fn is_eq<'b: 'a>(self, l: &'b Self::LImpl, other: Self) -> bool {
                            use crate::extension_of::reference::NatLit;
                            use crate::extension_of::reference::F;
                            use crate::extension_of::reference::Plus;
                            match (self, other) {
                                (
                                    crate::data_structure::refbased::Nat::NatLit(b),
                                    crate::data_structure::refbased::Nat::NatLit(a),
                                ) => b.is_eq(l, *a),
                                (
                                    crate::data_structure::refbased::Nat::F(b),
                                    crate::data_structure::refbased::Nat::F(a),
                                ) => b.is_eq(l, a),
                                (
                                    crate::data_structure::refbased::Nat::Plus(b),
                                    crate::data_structure::refbased::Nat::Plus(a),
                                ) => b.is_eq(l, a),
                                _ => false,
                            }
                        }
                    }
                }
                /// generated by [refbased_extensionof_gen::mut_reference::gen]
                pub mod mut_reference {
                    impl<'a> crate::extension_of::mut_reference::NatLit<'a>
                    for &'a mut crate::data_structure::refbased::NatLit {}
                    /// generated by [refbased_extensionof_gen::mut_reference::gen_prods]
                    impl<'a> crate::extension_of::mut_reference::Plus<'a>
                    for &'a mut crate::data_structure::refbased::Plus {
                        type LImpl = crate::data_structure::refbased::Fib;
                        type T0 = &'a mut crate::data_structure::refbased::Nat;
                        type T1 = &'a mut crate::data_structure::refbased::Nat;
                    }
                    /// generated by [refbased_extensionof_gen::mut_reference::gen_prods]
                    impl<'a> crate::extension_of::mut_reference::F<'a>
                    for &'a mut crate::data_structure::refbased::F {
                        type LImpl = crate::data_structure::refbased::Fib;
                        type T0 = &'a mut crate::data_structure::refbased::Nat;
                    }
                    impl<'a> crate::extension_of::Projection<crate::data_structure::refbased::Fib, 0>
                    for &'a mut crate::data_structure::refbased::Plus {
                        type To = &'a mut crate::data_structure::refbased::Nat;
                        fn project(self, _l: &crate::data_structure::refbased::Fib) -> Self::To {
                            &mut self.0
                        }
                    }
                    impl<'a> crate::extension_of::Projection<crate::data_structure::refbased::Fib, 1>
                    for &'a mut crate::data_structure::refbased::Plus {
                        type To = &'a mut crate::data_structure::refbased::Nat;
                        fn project(self, _l: &crate::data_structure::refbased::Fib) -> Self::To {
                            &mut self.1
                        }
                    }
                    impl<'a> crate::extension_of::Projection<crate::data_structure::refbased::Fib, 0>
                    for &'a mut crate::data_structure::refbased::F {
                        type To = &'a mut crate::data_structure::refbased::Nat;
                        fn project(self, _l: &crate::data_structure::refbased::Fib) -> Self::To {
                            &mut self.0
                        }
                    }
                    /// generated by [refbased_extensionof_gen::mut_reference::gen_sums]
                    impl<'a> crate::extension_of::mut_reference::Nat<'a>
                    for &'a mut crate::data_structure::refbased::Nat {
                        type LImpl = crate::data_structure::refbased::Fib;
                        type Owned = crate::data_structure::refbased::Nat;
                        type NatLit = &'a mut crate::data_structure::refbased::NatLit;
                        type F = &'a mut crate::data_structure::refbased::F;
                        type Plus = &'a mut crate::data_structure::refbased::Plus;
                        fn nat_lit<'b: 'a>(self, _l: &'b Self::LImpl) -> Option<Self::NatLit> {
                            match self {
                                crate::data_structure::refbased::Nat::NatLit(a) => Some(a),
                                _ => None,
                            }
                        }
                        fn f<'b: 'a>(self, _l: &'b Self::LImpl) -> Option<Self::F> {
                            match self {
                                crate::data_structure::refbased::Nat::F(a) => Some(a),
                                _ => None,
                            }
                        }
                        fn plus<'b: 'a>(self, _l: &'b Self::LImpl) -> Option<Self::Plus> {
                            match self {
                                crate::data_structure::refbased::Nat::Plus(a) => Some(a),
                                _ => None,
                            }
                        }
                        fn set<'b: 'a>(self, _l: &'b mut Self::LImpl, value: Self::Owned) {
                            *self = value;
                        }
                    }
                }
            }
            /// generated by [extensionof_gen::gen]
            pub mod extension_of {
                /// generated by [extensionof_gen::limpl_trait]
                pub trait LImpl {
                    type NatLit: crate::extension_of::owned::NatLit<LImpl = Self>;
                    type Plus: crate::extension_of::owned::Plus<LImpl = Self>;
                    type F: crate::extension_of::owned::F<LImpl = Self>;
                    type Nat: crate::extension_of::owned::Nat<LImpl = Self>;
                    fn convert<
                        LImpl: crate::extension_of::LImpl<Plus = Plus, F = F, Nat = Nat>,
                        NatLit,
                        Plus: crate::extension_of::owned::Plus<LImpl = LImpl>,
                        F: crate::extension_of::owned::F<LImpl = LImpl>,
                        Nat: crate::extension_of::owned::Nat<LImpl = LImpl>,
                    >(
                        &mut self,
                        lo: &LImpl,
                        from: crate::extension_of::Any<NatLit, Plus, F, Nat>,
                    ) -> crate::extension_of::Any<NatLit, Self::Plus, Self::F, Self::Nat> {
                        use crate::extension_of::reference::Plus;
                        use crate::extension_of::reference::F;
                        use crate::extension_of::reference::Nat;
                        match from {
                            crate::extension_of::Any::NatLit(nat_lit) => {
                                crate::extension_of::Any::NatLit(nat_lit)
                            }
                            crate::extension_of::Any::Plus(p) => {
                                crate::extension_of::Any::Plus(p.get_ref(lo).convert(lo, self))
                            }
                            crate::extension_of::Any::F(p) => {
                                crate::extension_of::Any::F(p.get_ref(lo).convert(lo, self))
                            }
                            crate::extension_of::Any::Nat(s) => {
                                crate::extension_of::Any::Nat(s.get_ref(lo).convert(lo, self))
                            }
                        }
                    }
                }
                /// generated by [extensionof_gen::any_type]
                pub enum Any<
                    NatLit,
                    Plus: crate::extension_of::owned::Plus,
                    F: crate::extension_of::owned::F,
                    Nat: crate::extension_of::owned::Nat,
                > {
                    NatLit(NatLit),
                    Plus(Plus),
                    F(F),
                    Nat(Nat),
                }
                pub trait Projection<LImpl, const N: u8> {
                    type To;
                    fn project(self, l: &LImpl) -> Self::To;
                }
                pub trait ExtensionOf {
                    type Any;
                    fn take_reduct<L: ExtensionOf>(&self, term: &Self::Any) -> (L, L::Any);
                }
                /// generated by [extensionof_gen::owned::gen]
                pub mod owned {
                    pub trait NatLit: From<u64> {
                        type LImpl: crate::extension_of::LImpl;
                    }
                    /// generated by [extensionof_gen::owned::prods]
                    pub trait Plus {
                        type LImpl: crate::extension_of::LImpl;
                        fn new(
                            l: &mut Self::LImpl,
                            args: (
                                <Self::LImpl as crate::extension_of::LImpl>::Nat,
                                <Self::LImpl as crate::extension_of::LImpl>::Nat,
                            ),
                        ) -> Self;
                        fn get_ref<'a, 'b: 'a>(
                            &'a self,
                            l: &'b Self::LImpl,
                        ) -> impl crate::extension_of::reference::Plus<'a, LImpl = Self::LImpl>;
                        fn get_mut<'a, 'b: 'a>(
                            &'a mut self,
                            l: &'b mut Self::LImpl,
                        ) -> impl crate::extension_of::mut_reference::Plus<'a, LImpl = Self::LImpl>;
                    }
                    /// generated by [extensionof_gen::owned::prods]
                    pub trait F {
                        type LImpl: crate::extension_of::LImpl;
                        fn new(
                            l: &mut Self::LImpl,
                            args: (<Self::LImpl as crate::extension_of::LImpl>::Nat,),
                        ) -> Self;
                        fn get_ref<'a, 'b: 'a>(
                            &'a self,
                            l: &'b Self::LImpl,
                        ) -> impl crate::extension_of::reference::F<'a, LImpl = Self::LImpl>;
                        fn get_mut<'a, 'b: 'a>(
                            &'a mut self,
                            l: &'b mut Self::LImpl,
                        ) -> impl crate::extension_of::mut_reference::F<'a, LImpl = Self::LImpl>;
                    }
                    /// generated by [extensionof_gen::owned::sums]
                    pub trait Nat {
                        type LImpl: crate::extension_of::LImpl;
                        fn nat_lit(
                            l: &mut Self::LImpl,
                            from: <Self::LImpl as crate::extension_of::LImpl>::NatLit,
                        ) -> Self;
                        fn f(
                            l: &mut Self::LImpl,
                            from: <Self::LImpl as crate::extension_of::LImpl>::F,
                        ) -> Self;
                        fn plus(
                            l: &mut Self::LImpl,
                            from: <Self::LImpl as crate::extension_of::LImpl>::Plus,
                        ) -> Self;
                        fn get_ref(
                            &self,
                            l: &Self::LImpl,
                        ) -> impl crate::extension_of::reference::Nat<'_, LImpl = Self::LImpl>;
                        fn get_mut(
                            &mut self,
                            l: &mut Self::LImpl,
                        ) -> impl crate::extension_of::mut_reference::Nat<'_, LImpl = Self::LImpl>;
                    }
                }
                /// generated by [extensionof_gen::reference::gen]
                pub mod reference {
                    pub trait NatLit<'a>: Into<u64> {
                        type LImpl: crate::extension_of::LImpl;
                        fn is_eq<'b: 'a>(self, l: &'b Self::LImpl, other: Self) -> bool;
                        fn convert<'b: 'a, 'c, O: crate::extension_of::owned::NatLit>(
                            self,
                            _l: &'b Self::LImpl,
                            _lo: &'c mut O::LImpl,
                        ) -> O {
                            let intermediate: u64 = self.into();
                            O::from(intermediate)
                        }
                    }
                    /// generated by [extensionof_gen::reference::prods]
                    pub trait Plus<
                        'a,
                    >: Copy + 'a + crate::extension_of::Projection<
                            Self::LImpl,
                            0,
                            To = Self::T0,
                        > + crate::extension_of::Projection<Self::LImpl, 1, To = Self::T1>
                    where
                        <Self::LImpl as crate::extension_of::LImpl>::Nat: 'a,
                        <Self::LImpl as crate::extension_of::LImpl>::Nat: 'a,
                    {
                        type LImpl: crate::extension_of::LImpl;
                        type T0: crate::extension_of::reference::Nat<'a, LImpl = Self::LImpl>;
                        type T1: crate::extension_of::reference::Nat<'a, LImpl = Self::LImpl>;
                        fn is_eq<'b: 'a>(self, l: &'b Self::LImpl, other: Self) -> bool;
                        fn convert<'b: 'a, 'c, O: crate::extension_of::owned::Plus>(
                            self,
                            l: &'b Self::LImpl,
                            lo: &'c mut O::LImpl,
                        ) -> O {
                            let args = (
                                <Self as crate::extension_of::Projection<
                                    Self::LImpl,
                                    0,
                                >>::project(self, l)
                                    .convert(l, lo),
                                <Self as crate::extension_of::Projection<
                                    Self::LImpl,
                                    1,
                                >>::project(self, l)
                                    .convert(l, lo),
                            );
                            O::new(lo, args)
                        }
                    }
                    /// generated by [extensionof_gen::reference::prods]
                    pub trait F<
                        'a,
                    >: Copy + 'a + crate::extension_of::Projection<Self::LImpl, 0, To = Self::T0>
                    where
                        <Self::LImpl as crate::extension_of::LImpl>::Nat: 'a,
                    {
                        type LImpl: crate::extension_of::LImpl;
                        type T0: crate::extension_of::reference::Nat<'a, LImpl = Self::LImpl>;
                        fn is_eq<'b: 'a>(self, l: &'b Self::LImpl, other: Self) -> bool;
                        fn convert<'b: 'a, 'c, O: crate::extension_of::owned::F>(
                            self,
                            l: &'b Self::LImpl,
                            lo: &'c mut O::LImpl,
                        ) -> O {
                            let args = (
                                <Self as crate::extension_of::Projection<
                                    Self::LImpl,
                                    0,
                                >>::project(self, l)
                                    .convert(l, lo),
                            );
                            O::new(lo, args)
                        }
                    }
                    /// generated by [extensionof_gen::reference::sums]
                    pub trait Nat<'a>: Copy + 'a {
                        type LImpl: crate::extension_of::LImpl;
                        type NatLit: crate::extension_of::reference::NatLit<'a, LImpl = Self::LImpl>;
                        type F: crate::extension_of::reference::F<'a, LImpl = Self::LImpl>;
                        type Plus: crate::extension_of::reference::Plus<'a, LImpl = Self::LImpl>;
                        fn nat_lit<'b: 'a>(self, l: &'b Self::LImpl) -> Option<Self::NatLit>;
                        fn f<'b: 'a>(self, l: &'b Self::LImpl) -> Option<Self::F>;
                        fn plus<'b: 'a>(self, l: &'b Self::LImpl) -> Option<Self::Plus>;
                        fn is_eq<'b: 'a>(self, l: &'b Self::LImpl, other: Self) -> bool;
                        fn convert<'b: 'a, 'c, O: crate::extension_of::owned::Nat>(
                            self,
                            l: &'b Self::LImpl,
                            lo: &'c mut O::LImpl,
                        ) -> O {
                            if let Some(x) = self.nat_lit(l) {
                                let arg = x.convert(l, lo);
                                return O::nat_lit(lo, arg);
                            }
                            if let Some(x) = self.f(l) {
                                let arg = x.convert(l, lo);
                                return O::f(lo, arg);
                            }
                            if let Some(x) = self.plus(l) {
                                let arg = x.convert(l, lo);
                                return O::plus(lo, arg);
                            }
                            panic!("unreachable");
                        }
                    }
                }
                pub mod mut_reference {
                    pub trait NatLit<'a> {}
                    /// generated by [extensionof_gen::mut_reference::prods]
                    pub trait Plus<
                        'a,
                    >: crate::extension_of::Projection<
                            Self::LImpl,
                            0,
                            To = Self::T0,
                        > + crate::extension_of::Projection<Self::LImpl, 1, To = Self::T1>
                    where
                        <Self::LImpl as crate::extension_of::LImpl>::Nat: 'a,
                        <Self::LImpl as crate::extension_of::LImpl>::Nat: 'a,
                    {
                        type LImpl: crate::extension_of::LImpl;
                        type T0: crate::extension_of::mut_reference::Nat<'a>;
                        type T1: crate::extension_of::mut_reference::Nat<'a>;
                    }
                    /// generated by [extensionof_gen::mut_reference::prods]
                    pub trait F<'a>: crate::extension_of::Projection<Self::LImpl, 0, To = Self::T0>
                    where
                        <Self::LImpl as crate::extension_of::LImpl>::Nat: 'a,
                    {
                        type LImpl: crate::extension_of::LImpl;
                        type T0: crate::extension_of::mut_reference::Nat<'a>;
                    }
                    /// generated by [extensionof_gen::mut_reference::sums]
                    pub trait Nat<'a>: 'a {
                        type LImpl: crate::extension_of::LImpl;
                        type Owned: crate::extension_of::owned::Nat;
                        type NatLit: crate::extension_of::mut_reference::NatLit<'a>;
                        type F: crate::extension_of::mut_reference::F<'a>;
                        type Plus: crate::extension_of::mut_reference::Plus<'a>;
                        fn nat_lit<'b: 'a>(self, l: &'b Self::LImpl) -> Option<Self::NatLit>;
                        fn f<'b: 'a>(self, l: &'b Self::LImpl) -> Option<Self::F>;
                        fn plus<'b: 'a>(self, l: &'b Self::LImpl) -> Option<Self::Plus>;
                        fn set<'b: 'a>(self, l: &'b mut Self::LImpl, value: Self::Owned);
                    }
                }
            }
            pub mod data_structure {
                /// generated by [langdatastructure_gen::refbased::gen]
                pub mod refbased {
                    /// generated by [langdatastructure_gen::refbased::gen_limpl]
                    pub struct Fib;
                    pub type NatLit = u64;
                    pub struct Plus(
                        pub crate::data_structure::refbased::Nat,
                        pub crate::data_structure::refbased::Nat,
                    );
                    pub struct F(pub crate::data_structure::refbased::Nat);
                    pub enum Nat {
                        NatLit(u64),
                        F(Box<crate::data_structure::refbased::F>),
                        Plus(Box<crate::data_structure::refbased::Plus>),
                    }
                }
            }
        "#]];
        expected.assert_eq(&formatted);
    }
}
