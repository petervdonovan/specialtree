use langdatastructure_gen::idxbased::sort2rs_idx_type;
use langspec::{
    flat::LangSpecFlat,
    humanreadable::LangSpecHuman,
    langspec::{LangSpec, TerminalLangSpec as _},
};
use langspec_gen_util::{transpose, LangSpecGen, ProdGenData};

pub struct BasePaths {
    extension_of: syn::Path,
    data_structure: syn::Path,
    impls: syn::Path,
}

pub fn gen<L: LangSpec>(bps: &BasePaths, ls: &L) -> syn::Item {
    let ls = LangSpecGen {
        bak: ls,
        sort2rs_type: sort2rs_idx_type,
        type_base_path: bps.data_structure.clone(),
    };
    let limpl = limpl(bps, &ls);
    let owned = owned::gen(bps, &ls);
    let byline = langspec_gen_util::byline!();
    syn::parse_quote!(
        #byline
        pub mod idxbased_extension_of {
            #limpl
            #owned
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
    use langspec_gen_util::SumGenData;

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
            pub mod owned {
                impl #extension_of::owned::NatLit for #data_structure::NatLit {
                    type Ref<'a> = Self;
                    type RefMut<'a> = Self;
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
                            type Ref<'a> = Self;
                            type RefMut<'a> = Self;
                            fn new(l: &mut Self::LImpl, args: (#(<Self::LImpl as #extension_of::LImpl>::#sort_rs_camel_idents,)*)) -> Self {
                                // hash-consing not yet implemented
                                let data = #data_structure::data::#camel_ident(#(args.#arg_idxs),*);
                                let ret = l.#snake_ident.len();
                                l.#snake_ident.push(data);
                                Self(ret)
                            }
                            fn get_ref(&self, _l: &Self::LImpl) -> Self::Ref<'_> {
                                *self
                            }
                            fn get_mut(&mut self, _l: &mut Self::LImpl) -> Self::RefMut<'_> {
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
                sort_rs_snake_idents, sort_shapes, .. }| {
                let maybe_box_from = sort_shapes.map(|shape| {
                    match shape {
                        langspec::langspec::SortId::Algebraic(_) => quote::quote!(Box::new(from)),
                        _ => quote::quote!(from),
                    }
                });
                let byline = langspec_gen_util::byline!();
                syn::parse_quote! {
                    #byline
                    impl #extension_of::owned::#camel_ident for #data_structure::#camel_ident {
                        type LImpl = #data_structure::#ls_camel_ident;
                        type Ref<'a> = Self;
                        type RefMut<'a> = Self;
                        #(
                            fn #sort_rs_snake_idents(l: &mut Self::LImpl, from: <Self::LImpl as #extension_of::LImpl>::#sort_rs_camel_idents) -> Self {
                                let data = #data_structure::data::#camel_ident::#sort_rs_camel_idents(#maybe_box_from);
                                let ret = l.#snake_ident.len();
                                l.#snake_ident.push(data);
                                Self(ret)
                            }
                        )*
                        fn get_ref(&self, _l: &Self::LImpl) -> Self::Ref<'_> {
                            *self
                        }
                        fn get_mut(&mut self, _l: &mut Self::LImpl) -> Self::RefMut<'_> {
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
        impls: syn::parse_quote!(crate::impls),
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
