use langspec::langspec::LangSpec;
use langspec_gen_util::{transpose, LangSpecGen, ProdGenData, SumGenData};
use syn::parse_quote;

pub fn gen<L: LangSpec>(base_path: &syn::Path, ls: &L) -> syn::Item {
    fn gen_extension_of() -> syn::Item {
        parse_quote!(
            pub trait ExtensionOf {
                type Any;
                fn take_reduct<L: ExtensionOf>(&self, term: &Self::Any) -> (L, L::Any);
            }
        )
    }
    fn limpl_trait<L: LangSpec>(base_path: &syn::Path, ls: &LangSpecGen<L>) -> syn::Item {
        transpose!(ls.prod_gen_datas(), camel_ident);
        let prod_types = camel_ident;
        transpose!(ls.sum_gen_datas(), camel_ident);
        let sum_types = camel_ident;
        parse_quote!(
            pub trait LImpl {
                type NatLit: #base_path::owned::NatLit;
                #(
                    type #prod_types: #base_path::owned::#prod_types;
                )*
                #(
                    type #sum_types: #base_path::owned::#sum_types;
                )*
            }
        )
    }
    fn gen_owned<L: LangSpec>(base_path: &syn::Path, ls: &LangSpecGen<L>) -> syn::Item {
        fn prods<L: LangSpec>(
            ret: &mut Vec<syn::Item>,
            base_path: &syn::Path,
            ls: &LangSpecGen<L>,
        ) {
            for ProdGenData {
                camel_ident: camel_name,
                sort_rs_camel_idents,
                ..
            } in ls.prod_gen_datas()
            {
                let gen = quote::quote!(
                    pub trait #camel_name: Eq {
                        type LImpl: #base_path::LImpl;
                        type Ref<'a>;
                        type RefMut<'a>;
                        fn new(l: &mut Self::LImpl, args: (#(<Self::LImpl as #base_path::LImpl>::#sort_rs_camel_idents),*)) -> Self;
                        fn get_ref(self, l: &Self::LImpl) -> Self::Ref<'_>;
                        fn get_mut(self, l: &mut Self::LImpl) -> Self::RefMut<'_>;
                    }
                );
                ret.push(parse_quote!(#gen));
            }
        }
        fn sums<L: LangSpec>(ret: &mut Vec<syn::Item>, base_path: &syn::Path, ls: &LangSpecGen<L>) {
            for SumGenData {
                camel_ident: camel_name,
                // sort_rs_types,
                sort_rs_camel_idents,
                sort_rs_snake_idents,
                ..
            } in ls.sum_gen_datas()
            {
                ret.push(parse_quote!(
                    pub trait #camel_name: Eq {
                        type LImpl: #base_path::LImpl;
                        type Ref<'a>;
                        type RefMut<'a>;
                        #(
                            fn #sort_rs_snake_idents(l: &mut Self::LImpl, from: <Self::LImpl as #base_path::LImpl>::#sort_rs_camel_idents) -> Self;
                        )*
                    }
                ));
            }
        }
        let mut ret: Vec<syn::Item> = vec![];
        prods(&mut ret, base_path, ls);
        sums(&mut ret, base_path, ls);
        parse_quote!(
            pub mod owned {
                pub trait NatLit: Into<usize> + From<usize> {
                    type Ref<'a>;
                    type RefMut<'a>;
                }
                #(#ret)*
            }
        )
    }
    fn projection_trait() -> syn::ItemTrait {
        parse_quote!(
            pub trait Projection<LImpl, const N: u8> {
                type To;
                fn project(self, l: &LImpl) -> Self::To;
            }
        )
    }
    fn gen_reference<L: LangSpec>(base_path: &syn::Path, ls: &LangSpecGen<L>) -> syn::Item {
        fn prods<L: LangSpec>(
            ret: &mut Vec<syn::Item>,
            base_path: &syn::Path,
            ls: &LangSpecGen<L>,
        ) {
            for ProdGenData {
                camel_ident,
                n_sorts,
                sort_rs_camel_idents,
                ..
            } in ls.prod_gen_datas()
            {
                let idx = (0..n_sorts)
                    .map(|i| syn::LitInt::new(&i.to_string(), proc_macro2::Span::call_site()));
                let gen = quote::quote!(
                    pub trait #camel_ident<'a>: Copy #(+ #base_path::Projection<Self::LImpl, #idx, To=<<Self::LImpl as #base_path::LImpl>::#sort_rs_camel_idents as #base_path::owned::#sort_rs_camel_idents>::Ref<'a>>)* {
                        type LImpl: #base_path::LImpl;
                    }
                );
                ret.push(parse_quote!(#gen));
            }
        }
        fn sums<L: LangSpec>(ret: &mut Vec<syn::Item>, base_path: &syn::Path, ls: &LangSpecGen<L>) {
            for SumGenData {
                camel_ident,
                sort_rs_camel_idents,
                sort_rs_snake_idents,
                ..
            } in ls.sum_gen_datas()
            {
                ret.push(parse_quote!(
                    pub trait #camel_ident<'a>: Copy {
                        type LImpl: #base_path::LImpl;
                        #(
                            fn #sort_rs_snake_idents<'b: 'a>(self, l: &'b Self::LImpl) -> Option<<<Self::LImpl as #base_path::LImpl>::#sort_rs_camel_idents as #base_path::owned::#sort_rs_camel_idents>::Ref<'a>>;
                        )*
                    }
                ));
            }
        }
        let mut ret: Vec<syn::Item> = vec![];
        prods(&mut ret, base_path, ls);
        sums(&mut ret, base_path, ls);
        parse_quote!(
            pub mod reference {
                #(#ret)*
            }
        )
    }
    fn gen_mut_reference<L: LangSpec>(base_path: &syn::Path, ls: &LangSpecGen<L>) -> syn::Item {
        fn prods<L: LangSpec>(
            ret: &mut Vec<syn::Item>,
            base_path: &syn::Path,
            ls: &LangSpecGen<L>,
        ) {
            for ProdGenData {
                camel_ident,
                n_sorts,
                sort_rs_camel_idents,
                ..
            } in ls.prod_gen_datas()
            {
                let idx = (0..n_sorts)
                    .map(|i| syn::LitInt::new(&i.to_string(), proc_macro2::Span::call_site()));
                let gen = quote::quote!(
                    pub trait #camel_ident<'a>: Copy #(+ #base_path::Projection<Self::LImpl, #idx, To=<<Self::LImpl as #base_path::LImpl>::#sort_rs_camel_idents as #base_path::owned::#sort_rs_camel_idents>::RefMut<'a>>)* {
                        type LImpl: #base_path::LImpl;
                    }
                );
                ret.push(parse_quote!(#gen));
            }
        }
        fn sums<L: LangSpec>(ret: &mut Vec<syn::Item>, base_path: &syn::Path, ls: &LangSpecGen<L>) {
            for SumGenData {
                camel_ident,
                // sort_rs_types,
                sort_rs_camel_idents,
                sort_rs_snake_idents,
                ..
            } in ls.sum_gen_datas()
            {
                ret.push(parse_quote!(
                    pub trait #camel_ident<'a>: Copy {
                        type LImpl: #base_path::LImpl;
                        #(
                            fn #sort_rs_snake_idents<'b: 'a>(self, l: &'b mut Self::LImpl) -> Option<<<Self::LImpl as #base_path::LImpl>::#sort_rs_camel_idents as #base_path::owned::#sort_rs_camel_idents>::RefMut<'b>>;
                        )*
                        fn set<'b: 'a, 'c>(
                            self,
                            l: &'b mut Self::LImpl,
                            value: <<Self::LImpl as #base_path::LImpl>::#camel_ident as #base_path::owned::#camel_ident>::Ref<'c>,
                        );
                    }
                ));
            }
        }
        let mut ret: Vec<syn::Item> = vec![];
        prods(&mut ret, base_path, ls);
        sums(&mut ret, base_path, ls);
        parse_quote!(
            pub mod mut_reference {
                #(#ret)*
            }
        )
    }
    fn fail(_: langspec::langspec::SortId<syn::Type>) -> syn::Type {
        panic!("must be type-agnostic");
    }
    let lg = LangSpecGen {
        bak: ls,
        sort2rs_type: fail,
    };
    let extension_of = gen_extension_of();
    let owned = gen_owned(base_path, &lg);
    let reference = gen_reference(base_path, &lg);
    let mut_reference = gen_mut_reference(base_path, &lg);
    let projection_trait = projection_trait();
    let limpl_trait = limpl_trait(base_path, &lg);
    parse_quote!(
        pub mod extension_of {
            #projection_trait
            #limpl_trait
            #extension_of
            #owned
            #reference
            #mut_reference
        }
    )
}

pub fn formatted(base_path: &syn::Path, lsh: &langspec::humanreadable::LangSpecHuman) -> String {
    let lsf: langspec::flat::LangSpecFlat =
        <langspec::flat::LangSpecFlat as langspec::langspec::TerminalLangSpec>::canonical_from(lsh);
    let gen_result = gen(base_path, &lsf);
    prettyplease::unparse(&syn::parse_quote! {
        #gen_result
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_data_structure() {
        let formatted = formatted(&parse_quote!(crate), &langspec_examples::fib());
        let expected = expect_test::expect![[r#"
            pub mod extension_of {
                pub trait Projection<LImpl, const N: u8> {
                    type To;
                    fn project(self, l: &LImpl) -> Self::To;
                }
                pub trait LImpl {
                    type NatLit: crate::owned::NatLit;
                    type Plus: crate::owned::Plus;
                    type F: crate::owned::F;
                    type Nat: crate::owned::Nat;
                }
                pub trait ExtensionOf {
                    type Any;
                    fn take_reduct<L: ExtensionOf>(&self, term: &Self::Any) -> (L, L::Any);
                }
                pub mod owned {
                    pub trait NatLit: Into<usize> + From<usize> {
                        type Ref<'a>;
                        type RefMut<'a>;
                    }
                    pub trait Plus: Eq {
                        type LImpl: crate::LImpl;
                        type Ref<'a>;
                        type RefMut<'a>;
                        fn new(
                            l: &mut Self::LImpl,
                            args: (
                                <Self::LImpl as crate::LImpl>::Nat,
                                <Self::LImpl as crate::LImpl>::Nat,
                            ),
                        ) -> Self;
                        fn get_ref(self, l: &Self::LImpl) -> Self::Ref<'_>;
                        fn get_mut(self, l: &mut Self::LImpl) -> Self::RefMut<'_>;
                    }
                    pub trait F: Eq {
                        type LImpl: crate::LImpl;
                        type Ref<'a>;
                        type RefMut<'a>;
                        fn new(
                            l: &mut Self::LImpl,
                            args: (<Self::LImpl as crate::LImpl>::Nat),
                        ) -> Self;
                        fn get_ref(self, l: &Self::LImpl) -> Self::Ref<'_>;
                        fn get_mut(self, l: &mut Self::LImpl) -> Self::RefMut<'_>;
                    }
                    pub trait Nat: Eq {
                        type LImpl: crate::LImpl;
                        type Ref<'a>;
                        type RefMut<'a>;
                        fn nat_lit(
                            l: &mut Self::LImpl,
                            from: <Self::LImpl as crate::LImpl>::NatLit,
                        ) -> Self;
                        fn f(l: &mut Self::LImpl, from: <Self::LImpl as crate::LImpl>::F) -> Self;
                        fn plus(
                            l: &mut Self::LImpl,
                            from: <Self::LImpl as crate::LImpl>::Plus,
                        ) -> Self;
                    }
                }
                pub mod reference {
                    pub trait Plus<
                        'a,
                    >: Copy + crate::Projection<
                            Self::LImpl,
                            0,
                            To = <<Self::LImpl as crate::LImpl>::Nat as crate::owned::Nat>::Ref<'a>,
                        > + crate::Projection<
                            Self::LImpl,
                            1,
                            To = <<Self::LImpl as crate::LImpl>::Nat as crate::owned::Nat>::Ref<'a>,
                        > {
                        type LImpl: crate::LImpl;
                    }
                    pub trait F<
                        'a,
                    >: Copy + crate::Projection<
                            Self::LImpl,
                            0,
                            To = <<Self::LImpl as crate::LImpl>::Nat as crate::owned::Nat>::Ref<'a>,
                        > {
                        type LImpl: crate::LImpl;
                    }
                    pub trait Nat<'a>: Copy {
                        type LImpl: crate::LImpl;
                        fn nat_lit<'b: 'a>(
                            self,
                            l: &'b Self::LImpl,
                        ) -> Option<
                            <<Self::LImpl as crate::LImpl>::NatLit as crate::owned::NatLit>::Ref<'a>,
                        >;
                        fn f<'b: 'a>(
                            self,
                            l: &'b Self::LImpl,
                        ) -> Option<<<Self::LImpl as crate::LImpl>::F as crate::owned::F>::Ref<'a>>;
                        fn plus<'b: 'a>(
                            self,
                            l: &'b Self::LImpl,
                        ) -> Option<
                            <<Self::LImpl as crate::LImpl>::Plus as crate::owned::Plus>::Ref<'a>,
                        >;
                    }
                }
                pub mod mut_reference {
                    pub trait Plus<
                        'a,
                    >: Copy + crate::Projection<
                            Self::LImpl,
                            0,
                            To = <<Self::LImpl as crate::LImpl>::Nat as crate::owned::Nat>::RefMut<
                                'a,
                            >,
                        > + crate::Projection<
                            Self::LImpl,
                            1,
                            To = <<Self::LImpl as crate::LImpl>::Nat as crate::owned::Nat>::RefMut<
                                'a,
                            >,
                        > {
                        type LImpl: crate::LImpl;
                    }
                    pub trait F<
                        'a,
                    >: Copy + crate::Projection<
                            Self::LImpl,
                            0,
                            To = <<Self::LImpl as crate::LImpl>::Nat as crate::owned::Nat>::RefMut<
                                'a,
                            >,
                        > {
                        type LImpl: crate::LImpl;
                    }
                    pub trait Nat<'a>: Copy {
                        type LImpl: crate::LImpl;
                        fn nat_lit<'b: 'a>(
                            self,
                            l: &'b mut Self::LImpl,
                        ) -> Option<
                            <<Self::LImpl as crate::LImpl>::NatLit as crate::owned::NatLit>::RefMut<
                                'b,
                            >,
                        >;
                        fn f<'b: 'a>(
                            self,
                            l: &'b mut Self::LImpl,
                        ) -> Option<
                            <<Self::LImpl as crate::LImpl>::F as crate::owned::F>::RefMut<'b>,
                        >;
                        fn plus<'b: 'a>(
                            self,
                            l: &'b mut Self::LImpl,
                        ) -> Option<
                            <<Self::LImpl as crate::LImpl>::Plus as crate::owned::Plus>::RefMut<'b>,
                        >;
                        fn set<'b: 'a, 'c>(
                            self,
                            l: &'b mut Self::LImpl,
                            value: <<Self::LImpl as crate::LImpl>::Nat as crate::owned::Nat>::Ref<'c>,
                        );
                    }
                }
            }
        "#]];
        expected.assert_eq(&formatted);
    }
}
