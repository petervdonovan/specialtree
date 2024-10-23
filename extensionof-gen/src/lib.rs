use langspec::langspec::LangSpec;
use langspec_gen_util::{LangSpecGen, ProdGenData, SumGenData};
use syn::parse_quote;

pub fn gen<L: LangSpec>(ls: &L) -> syn::Item {
    fn gen_extension_of() -> syn::Item {
        parse_quote!(
            pub trait ExtensionOf {
                type Any;
                fn take_reduct<L: ExtensionOf>(&self, term: &Self::Any) -> (L, L::Any);
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
                camel_name,
                sort_rs_idents,
                ..
            } in ls.prod_gen_datas()
            {
                let gen = quote::quote!(
                    pub trait #camel_name<LImpl>: Eq + From<#base_path::reference::CamelName> {
                        type Ref: Into<Self> + From<Self>;
                        type Mut: Into<Self::Ref>;
                        fn new(l: &mut LImpl, args: (#(LImpl::#sort_rs_idents),*)) -> Self;
                    }
                );
                ret.push(parse_quote!(#gen));
            }
        }
        fn sums<L: LangSpec>(ret: &mut Vec<syn::Item>, base_path: &syn::Path, ls: &LangSpecGen<L>) {
            for SumGenData {
                camel_name,
                // sort_rs_types,
                sort_rs_idents,
                ..
            } in ls.sum_gen_datas()
            {
                ret.push(parse_quote!(
                    pub trait #camel_name<LImpl>: Eq + From<#base_path::reference::CamelName> {
                        type Ref: Into<Self> + From<Self>;
                        type Mut: Into<Ref>;
                        #(
                            fn #sort_rs_idents(l: &mut LImpl, from: LImpl::#sort_rs_idents) -> Self;
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
                #(#ret)*
            }
        )
    }
    fn gen_reference<L: LangSpec>(base_path: &syn::Path, ls: &LangSpecGen<L>) -> syn::Item {
        fn projection_trait(base_path: &syn::Path, camel_name: syn::Ident) -> syn::ItemTrait {
            parse_quote!(
                pub trait Projection<LImpl, const N: u8> {
                    fn project(self, l: &LImpl) -> <Self::#camel_name as #base_path::owned::#camel_name>::Ref;
                }
            )
        }
        fn prods<L: LangSpec>(
            ret: &mut Vec<syn::Item>,
            _base_path: &syn::Path,
            ls: &LangSpecGen<L>,
        ) {
            for ProdGenData {
                camel_name,
                sort_rs_idents,
                ..
            } in ls.prod_gen_datas()
            {
                let idx = (0..sort_rs_idents.count())
                    .map(|i| syn::LitInt::new(&i.to_string(), proc_macro2::Span::call_site()));
                let gen = quote::quote!(
                    pub trait #camel_name: Copy #(+ Projection<LImpl, #idx>)* {
                    }
                );
                ret.push(parse_quote!(#gen));
            }
        }
        fn sums<L: LangSpec>(
            ret: &mut Vec<syn::Item>,
            _base_path: &syn::Path,
            ls: &LangSpecGen<L>,
        ) {
            for SumGenData {
                camel_name,
                // sort_rs_types,
                sort_rs_idents,
                ..
            } in ls.sum_gen_datas()
            {
                ret.push(parse_quote!(
                    pub trait #camel_name: Copy {
                        type Ref;
                        type Mut;
                        #(
                            fn #sort_rs_idents(self, l: &LImpl) -> Option<LImpl::#sort_rs_idents>;
                        )*
                    }
                ));
            }
        }
        let mut ret: Vec<syn::Item> = vec![];
        prods(&mut ret, base_path, ls);
        sums(&mut ret, base_path, ls);
        let projection_trait = projection_trait(base_path, parse_quote!(F));
        parse_quote!(
            pub mod reference {
                #projection_trait
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
    let owned = gen_owned(&parse_quote!(crate), &lg);
    let reference = gen_reference(&parse_quote!(crate), &lg);
    parse_quote!(
        pub mod extension_of {
            #extension_of
            #owned
            #reference
        }
    )
}

pub fn formatted(lsh: &langspec::humanreadable::LangSpecHuman) -> String {
    let lsf: langspec::flat::LangSpecFlat =
        <langspec::flat::LangSpecFlat as langspec::langspec::TerminalLangSpec>::canonical_from(lsh);
    let gen_result = gen(&lsf);
    prettyplease::unparse(&syn::parse_quote! {
        #gen_result
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_data_structure() {
        let formatted = formatted(&langspec_examples::fib());
        let expected = expect_test::expect![[r#"
            pub mod extension_of {
                pub trait ExtensionOf {
                    type Any;
                    fn take_reduct<L: ExtensionOf>(&self, term: &Self::Any) -> (L, L::Any);
                }
                pub mod owned {
                    pub trait F<LImpl>: Eq + From<crate::reference::CamelName> {
                        type Ref: Into<Self> + From<Self>;
                        type Mut: Into<Self::Ref>;
                        fn new(l: &mut LImpl, args: (LImpl::Nat, LImpl::Nat)) -> Self;
                    }
                    pub trait Nat<LImpl>: Eq + From<crate::reference::CamelName> {
                        type Ref: Into<Self> + From<Self>;
                        type Mut: Into<Ref>;
                        fn NatLit(l: &mut LImpl, from: LImpl::NatLit) -> Self;
                        fn F(l: &mut LImpl, from: LImpl::F) -> Self;
                    }
                }
                pub mod reference {
                    pub trait Projection<LImpl, const N: u8> {
                        fn project(self, l: &LImpl) -> <Self::F as crate::owned::F>::Ref;
                    }
                    pub trait F: Copy + Projection<LImpl, 0> + Projection<LImpl, 1> {}
                    pub trait Nat: Copy {
                        type Ref;
                        type Mut;
                        fn NatLit(self, l: &LImpl) -> Option<LImpl::NatLit>;
                        fn F(self, l: &LImpl) -> Option<LImpl::F>;
                    }
                }
            }
        "#]];
        expected.assert_eq(&formatted);
    }
}
