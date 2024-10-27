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
        syn::parse_quote! {
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
    fn gen_owned<L: LangSpec>(bps: &BasePaths, ls: &LangSpecGen<L>) -> syn::Item {
        fn gen_prods<L: LangSpec>(
            BasePaths {
                extension_of,
                data_structure,
                ..
            }: &BasePaths,
            ls: &LangSpecGen<L>,
        ) -> Vec<syn::ItemImpl> {
            let ls_camel_ident = ls.camel_ident();
            ls.prod_gen_datas()
                .map(
                    |ProdGenData {
                         snake_ident,
                         camel_ident,
                         rs_ty,
                         n_sorts,
                         sort_rs_camel_idents,
                         sort_rs_snake_idents,
                         sort_rs_types,
                         sort_shapes,
                     }| {
                        let arg_idxs = (0..n_sorts).map(|i| syn::Index::from(i));
                        syn::parse_quote! {
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
                                fn get_ref(self, _l: &Self::LImpl) -> Self::Ref<'_> {
                                    self
                                }
                                fn get_mut(self, _l: &mut Self::LImpl) -> Self::RefMut<'_> {
                                    self
                                }
                            }
                        }
                    },
                )
                .collect()
        }
        fn gen_sums<L: LangSpec>(
            BasePaths {
                extension_of,
                data_structure,
                ..
            }: &BasePaths,
            ls: &LangSpecGen<L>,
        ) -> Vec<syn::ItemImpl> {
            vec![]
        }
        let prods = gen_prods(bps, ls);
        let sums = gen_sums(bps, ls);
        syn::parse_quote!(
            pub mod owned {
                #(
                    #prods
                )*
                #(
                    #sums
                )*
            }
        )
    }
    let ls = LangSpecGen {
        bak: ls,
        sort2rs_type: sort2rs_idx_type,
        type_base_path: bps.data_structure.clone(),
    };
    let limpl = limpl(bps, &ls);
    let owned = gen_owned(bps, &ls);
    let byline = langspec_gen_util::byline!();
    syn::parse_quote!(
        #byline
        pub mod idxbased_extension_of {
            #limpl
            #owned
        }
    )
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
