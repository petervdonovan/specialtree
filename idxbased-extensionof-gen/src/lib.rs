use langdatastructure_gen::idxbased::sort2rs_idx_type;
use langspec::{
    flat::LangSpecFlat,
    humanreadable::LangSpecHuman,
    langspec::{LangSpec, TerminalLangSpec as _},
};
use langspec_gen_util::{transpose, LangSpecGen, ProdGenData, SumGenData};

pub struct BasePaths {
    extension_of: syn::Path,
    data_structure: syn::Path,
}

pub fn gen<L: LangSpec>(bps: &BasePaths, ls: &L) -> syn::Item {
    let ls = LangSpecGen {
        bak: ls,
        sort2rs_type: sort2rs_idx_type,
        type_base_path: bps.data_structure.clone(),
    };
    let limpl = limpl(bps, &ls);
    let owned = owned::gen(bps, &ls);
    let reference = reference::gen(bps, &ls);
    let mut_reference = mut_reference::gen(bps, &ls);
    let byline = langspec_gen_util::byline!();
    syn::parse_quote!(
        #byline
        pub mod idxbased_extension_of {
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
        let BasePaths {
            extension_of,
            data_structure,
            ..
        } = bps;
        let ls_camel_ident = ls.camel_ident();
        let byline = langspec_gen_util::byline!();
        syn::parse_quote!(
            #byline
            pub mod owned {
                impl #extension_of::owned::NatLit for #data_structure::NatLit {
                    type LImpl = #data_structure::#ls_camel_ident;
                }
                impl std::convert::From<u64> for #data_structure::NatLit {
                    fn from(x: u64) -> Self {
                        Self(x)
                    }
                }
                impl std::convert::From<#data_structure::NatLit> for u64 {
                    fn from(x: #data_structure::NatLit) -> Self {
                        x.0
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
                     snake_ident,
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
                            fn new(l: &mut Self::LImpl, args: (#(<Self::LImpl as #extension_of::LImpl>::#sort_rs_camel_idents,)*)) -> Self {
                                // hash-consing not yet implemented
                                let data = #data_structure::data::#camel_ident(#(args.#arg_idxs),*);
                                let ret = l.#snake_ident.len();
                                l.#snake_ident.push(data);
                                Self(ret)
                            }
                            fn get_ref<'a, 'b: 'a>(&'a self, _l: &'b Self::LImpl) -> impl #extension_of::reference::#camel_ident<'a, LImpl = Self::LImpl> {
                                *self
                            }
                            fn get_mut<'a, 'b: 'a>(&'a mut self, _l: &'b mut Self::LImpl) -> impl #extension_of::mut_reference::#camel_ident<'a, LImpl = Self::LImpl> {
                                *self
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
                sort_rs_snake_idents, .. }| {
                let byline = langspec_gen_util::byline!();
                syn::parse_quote! {
                    #byline
                    impl #extension_of::owned::#camel_ident for #data_structure::#camel_ident {
                        type LImpl = #data_structure::#ls_camel_ident;
                        #(
                            fn #sort_rs_snake_idents(l: &mut Self::LImpl, from: <Self::LImpl as #extension_of::LImpl>::#sort_rs_camel_idents) -> Self {
                                let data = #data_structure::data::#camel_ident::#sort_rs_camel_idents(from);
                                let ret = l.#snake_ident.len();
                                l.#snake_ident.push(data);
                                Self(ret)
                            }
                        )*
                        fn get_ref(&self, _l: &Self::LImpl) -> impl #extension_of::reference::#camel_ident<'_, LImpl = Self::LImpl> {
                            *self
                        }
                        fn get_mut(&mut self, _l: &mut Self::LImpl) -> impl #extension_of::mut_reference::#camel_ident<'_, LImpl = Self::LImpl> {
                            *self
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
        data_structure: syn::parse_quote!(crate::data_structure::idxbased),
    };
    let m = gen(&bps, &lsf);
    let extension_of = extensionof_gen::gen(&bps.extension_of, &lsf);
    let data_structure =
        langdatastructure_gen::idxbased::gen(&syn::parse_quote!(crate::data_structure), &lsf);
    prettyplease::unparse(&syn::parse_quote!(
        #m
        #extension_of
        pub mod data_structure {
            #data_structure
        }
    ))
}

mod reference {

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
                        self.0 == other.0
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
                    impl<'a> #extension_of::reference::#camel_ident<'a> for #data_structure::#rs_ty {
                        type LImpl = #data_structure::#ls_camel_ident;
                        #(
                            type #ty_idx = #data_structure::#sort_rs_camel_idents;
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
                 ..
             }| {
                let ls_camel_ident = ls.camel_ident();
                idx.zip(sort_rs_camel_idents).map(
                    move |(idx, sort_rs_camel_idents)|{
                        let ret: syn::ItemImpl = syn::parse_quote! {
                            impl #extension_of::Projection<#data_structure::#ls_camel_ident, #idx> for #data_structure::#rs_ty {
                                type To = #data_structure::#sort_rs_camel_idents;
                                fn project(self, l: &#data_structure::#ls_camel_ident) -> Self::To {
                                    let my_data = &l.#snake_ident[self.0];
                                    my_data.#idx
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
                 ..
             }| {
                let byline = langspec_gen_util::byline!();
                collect!(sort_rs_camel_idents);
                let ret = quote::quote! {
                    #byline
                    impl<'a> #extension_of::reference::#camel_ident<'a> for #data_structure::#rs_ty {
                        type LImpl = #data_structure::#ls_camel_ident;
                        #(
                            type #sort_rs_camel_idents = #data_structure::#sort_rs_camel_idents;
                        )*
                        #(
                            fn #sort_rs_snake_idents<'b: 'a>(self, l: &'b Self::LImpl) -> Option<Self::#sort_rs_camel_idents> {
                                let my_data = &l.#snake_ident[self.0];
                                match my_data {
                                    #data_structure::data::#rs_ty::#sort_rs_camel_idents(a) => Some(*a),
                                    _ => None,
                                }
                            }
                        )*
                        fn is_eq<'b: 'a>(self, l: &'b Self::LImpl, other: Self) -> bool {
                            #(
                                use #extension_of::reference::#sort_rs_camel_idents;
                            )*
                            match (&l.#snake_ident[self.0], &l.#snake_ident[other.0]) {
                                #(
                                    (#data_structure::data::#rs_ty::#sort_rs_camel_idents(a), #data_structure::data::#rs_ty::#sort_rs_camel_idents(b)) => {
                                        a.is_eq(l, *b)
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
        let BasePaths {
            extension_of,
            data_structure,
            ..
        } = bps;
        let byline = langspec_gen_util::byline!();
        syn::parse_quote!(
            #byline
            pub mod mut_reference {
                impl<'a> #extension_of::mut_reference::NatLit<'a> for #data_structure::NatLit {
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
                    impl<'a> #extension_of::mut_reference::#camel_ident<'a> for #data_structure::#rs_ty {
                        type LImpl = #data_structure::#ls_camel_ident;
                        #(
                            type #ty_idx = #data_structure::#sort_rs_camel_idents;
                        )*
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
                impl<'a> #extension_of::mut_reference::#camel_ident<'a> for #data_structure::#rs_ty {
                    type LImpl = #data_structure::#ls_camel_ident;
                    type Owned = #data_structure::#camel_ident;
                    #(
                        type #sort_rs_camel_idents = #data_structure::#sort_rs_camel_idents;
                    )*
                    #(
                        fn #sort_rs_snake_idents<'b: 'a>(self, l: &'b Self::LImpl) -> Option<Self::#sort_rs_camel_idents> {
                            let my_data = &l.#snake_ident[self.0];
                            match my_data {
                                #data_structure::data::#rs_ty::#sort_rs_camel_idents(a) => Some(*a),
                                _ => None,
                            }
                        }
                    )*
                    fn set<'b: 'a>(
                        self,
                        l: &'b mut Self::LImpl,
                        value: Self::Owned,
                    ) {
                        let their_data = l.#snake_ident[value.0];
                        let my_data = &mut l.#snake_ident[self.0];
                        *my_data = their_data;
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
    fn test_idxbased_extension_of() {
        let formatted = formatted(&langspec_examples::fib());
        let expected = expect_test::expect![[r#"
            /// generated by idxbased_extensionof_gen
            pub mod idxbased_extension_of {
                impl crate::extension_of::LImpl for Fib {
                    type NatLit = crate::data_structure::NatLit;
                    type Plus = crate::data_structure::Plus;
                    type F = crate::data_structure::F;
                    type Nat = crate::data_structure::Nat;
                }
                pub mod owned {
                    impl crate::extension_of::owned::Plus for Plus {
                        type LImpl = crate::data_structure::LImpl;
                        type Ref<'a> = Self;
                        type RefMut<'a> = Self;
                        fn new(
                            l: &mut Self::LImpl,
                            args: (
                                <Self::LImpl as crate::extension_of::LImpl>::Nat,
                                <Self::LImpl as crate::extension_of::LImpl>::Nat,
                            ),
                        ) -> Self {
                            let ret = Self(args);
                            l.plus.push(Self(args));
                            ret
                        }
                        fn get_ref(self, l: &Self::LImpl) -> Self::Ref<'_> {
                            self
                        }
                        fn get_mut(self, l: &mut Self::LImpl) -> Self::RefMut<'_> {
                            self
                        }
                    }
                    impl crate::extension_of::owned::F for F {
                        type LImpl = crate::data_structure::LImpl;
                        type Ref<'a> = Self;
                        type RefMut<'a> = Self;
                        fn new(
                            l: &mut Self::LImpl,
                            args: (<Self::LImpl as crate::extension_of::LImpl>::Nat),
                        ) -> Self {
                            let ret = Self(args);
                            l.f.push(Self(args));
                            ret
                        }
                        fn get_ref(self, l: &Self::LImpl) -> Self::Ref<'_> {
                            self
                        }
                        fn get_mut(self, l: &mut Self::LImpl) -> Self::RefMut<'_> {
                            self
                        }
                    }
                }
            }
        "#]];
        expected.assert_eq(&formatted);
    }
}
