use langspec::langspec::LangSpec;
use langspec_gen_util::{LangSpecGen, ProdGenData, SumGenData, transpose};
use syn::parse_quote;

pub fn generate<L: LangSpec>(base_path: &syn::Path, ls: &L) -> syn::Item {
    fn fail(_: &syn::Path, _: langspec::langspec::SortId<syn::Type>) -> syn::Type {
        panic!("must be type-agnostic");
    }
    let lg = LangSpecGen {
        bak: ls,
        sort2rs_type: fail,
        type_base_path: base_path.clone(),
    };
    let owned = owned::generate(base_path, &lg);
    let reference = reference::generate(base_path, &lg);
    let mut_reference = mut_reference::generate(base_path, &lg);
    let projection_trait = projection_trait();
    let limpl_trait = limpl_trait(base_path, &lg);
    let byline = langspec_gen_util::byline!();
    parse_quote!(
        #byline
        pub mod extension_of {
            #limpl_trait
            #projection_trait
            #owned
            #reference
            #mut_reference
        }
    )
}
fn limpl_trait<L: LangSpec>(base_path: &syn::Path, ls: &LangSpecGen<L>) -> syn::Item {
    transpose!(ls.prod_gen_datas(), camel_ident);
    let prod_types = camel_ident;
    transpose!(ls.sum_gen_datas(), camel_ident);
    let sum_types = camel_ident;
    let byline = langspec_gen_util::byline!();
    parse_quote!(
        #byline
        pub trait LImpl: core::default::Default {
            type NatLit: #base_path::owned::NatLit<LImpl = Self>;
            #(
                type #prod_types: #base_path::owned::#prod_types<LImpl = Self>;
            )*
            #(
                type #sum_types: #base_path::owned::#sum_types<LImpl = Self>;
            )*
        }
    )
}
mod owned {
    use super::*;
    pub fn generate<L: LangSpec>(base_path: &syn::Path, ls: &LangSpecGen<L>) -> syn::Item {
        let mut ret: Vec<syn::Item> = vec![];
        prods(&mut ret, base_path, ls);
        sums(&mut ret, base_path, ls);
        let byline = langspec_gen_util::byline!();
        let nat_lit = nat_lit(base_path, ls);
        parse_quote!(
            #byline
            pub mod owned {
                #nat_lit
                #(#ret)*
            }
        )
    }
    pub fn nat_lit<L: LangSpec>(base_path: &syn::Path, _ls: &LangSpecGen<L>) -> syn::Item {
        let byline = langspec_gen_util::byline!();
        parse_quote!(
            #byline
            pub trait NatLit: From<u64> {
                type LImpl: #base_path::LImpl;
                fn get_ref<'a, 'b: 'a>(&'a self, l: &'b Self::LImpl) -> impl #base_path::reference::NatLit<'a, LImpl = Self::LImpl>;
                fn get_mut<'a, 'b: 'a>(&'a mut self, l: &'b mut Self::LImpl) -> impl #base_path::mut_reference::NatLit<'a, LImpl = Self::LImpl>;
            }
        )
    }
    pub fn prods<L: LangSpec>(
        ret: &mut Vec<syn::Item>,
        base_path: &syn::Path,
        ls: &LangSpecGen<L>,
    ) {
        let byline = langspec_gen_util::byline!();
        for ProdGenData {
            camel_ident: camel_name,
            sort_rs_camel_idents,
            ..
        } in ls.prod_gen_datas()
        {
            let generate = quote::quote!(
                #byline
                pub trait #camel_name {
                    type LImpl: #base_path::LImpl;
                    fn new(l: &mut Self::LImpl, args: (#(<Self::LImpl as #base_path::LImpl>::#sort_rs_camel_idents,)*)) -> Self;
                    fn get_ref<'a, 'b: 'a>(&'a self, l: &'b Self::LImpl) -> impl #base_path::reference::#camel_name<'a, LImpl = Self::LImpl>;
                    fn get_mut<'a, 'b: 'a>(&'a mut self, l: &'b mut Self::LImpl) -> impl #base_path::mut_reference::#camel_name<'a, LImpl = Self::LImpl>;
                }
            );
            ret.push(parse_quote!(#generate));
        }
    }
    pub fn sums<L: LangSpec>(ret: &mut Vec<syn::Item>, base_path: &syn::Path, ls: &LangSpecGen<L>) {
        let byline = langspec_gen_util::byline!();
        for SumGenData {
            camel_ident: camel_name,
            sort_rs_camel_idents,
            sort_rs_snake_idents,
            ..
        } in ls.sum_gen_datas()
        {
            ret.push(parse_quote!(
                #byline
                pub trait #camel_name {
                    type LImpl: #base_path::LImpl;
                    #(
                        fn #sort_rs_snake_idents(l: &mut Self::LImpl, from: <Self::LImpl as #base_path::LImpl>::#sort_rs_camel_idents) -> Self;
                    )*
                    fn get_ref(&self, l: &Self::LImpl) -> impl #base_path::reference::#camel_name<'_, LImpl = Self::LImpl>;
                    fn get_mut(&mut self, l: &mut Self::LImpl) -> impl #base_path::mut_reference::#camel_name<'_, LImpl = Self::LImpl>;
                }
            ));
        }
    }
}
fn projection_trait() -> syn::ItemTrait {
    parse_quote!(
        pub trait Projection<LImpl, const N: u8> {
            type To;
            fn project(self, l: &LImpl) -> Self::To;
        }
    )
}
mod reference {
    use langspec_gen_util::collect;

    use super::*;
    pub fn generate<L: LangSpec>(base_path: &syn::Path, ls: &LangSpecGen<L>) -> syn::Item {
        let mut ret: Vec<syn::Item> = vec![];
        prods(&mut ret, base_path, ls);
        sums(&mut ret, base_path, ls);
        let byline = langspec_gen_util::byline!();
        parse_quote!(
            #byline
            pub mod reference {
                pub trait NatLit<'a>: Into<u64> {
                    type LImpl: #base_path::LImpl;
                    fn is_eq<'b: 'a>(self, l: &'b Self::LImpl, other: Self) -> bool;
                    fn convert<'b: 'a, 'c, O: #base_path::owned::NatLit>(
                        self,
                        _l: &'b Self::LImpl,
                        _lo: &'c mut O::LImpl,
                    ) -> O {
                        let intermediate: u64 = self.into();
                        O::from(intermediate)
                    }
                }
                #(#ret)*
            }
        )
    }
    pub fn prods<L: LangSpec>(
        ret: &mut Vec<syn::Item>,
        base_path: &syn::Path,
        ls: &LangSpecGen<L>,
    ) {
        let byline = langspec_gen_util::byline!();
        for ProdGenData {
            camel_ident,
            sort_rs_camel_idents,
            idx,
            ty_idx,
            ..
        } in ls.prod_gen_datas()
        {
            collect!(sort_rs_camel_idents, idx, ty_idx);
            let generate = quote::quote!(
                #byline
                pub trait #camel_ident<'a>: Copy + 'a #(
                    + #base_path::Projection<Self::LImpl, #idx, To=Self::#ty_idx>
                )*
                where #(
                    <Self::LImpl as #base_path::LImpl>::#sort_rs_camel_idents: 'a,
                )*
                {
                    type LImpl: #base_path::LImpl;
                    #(
                        type #ty_idx: #base_path::reference::#sort_rs_camel_idents<'a, LImpl = Self::LImpl>;
                    )*
                    fn is_eq<'b: 'a>(self, l: &'b Self::LImpl, other: Self) -> bool;
                    fn convert<'b: 'a, 'c, O: #base_path::owned::#camel_ident>(
                        self,
                        l: &'b Self::LImpl,
                        lo: &'c mut O::LImpl,
                    ) -> O {
                        let args = (#(
                            <Self as #base_path::Projection<Self::LImpl, #idx>>::project(self, l).convert(l, lo),
                        )*);
                        O::new(lo, args)
                    }
                }
            );
            ret.push(parse_quote!(#generate));
        }
    }
    pub fn sums<L: LangSpec>(ret: &mut Vec<syn::Item>, base_path: &syn::Path, ls: &LangSpecGen<L>) {
        let byline = langspec_gen_util::byline!();
        for SumGenData {
            camel_ident,
            sort_rs_camel_idents,
            sort_rs_snake_idents,
            ..
        } in ls.sum_gen_datas()
        {
            collect!(sort_rs_camel_idents, sort_rs_snake_idents);
            ret.push(parse_quote!(
                #byline
                pub trait #camel_ident<'a>: Copy + 'a {
                    type LImpl: #base_path::LImpl;
                    #(
                        type #sort_rs_camel_idents: #base_path::reference::#sort_rs_camel_idents<'a, LImpl=Self::LImpl>;
                    )*
                    #(
                        fn #sort_rs_snake_idents<'b: 'a>(self, l: &'b Self::LImpl) -> Option<Self::#sort_rs_camel_idents>;
                    )*
                    fn is_eq<'b: 'a>(self, l: &'b Self::LImpl, other: Self) -> bool;
                    fn convert<'b: 'a, 'c, O: #base_path::owned::#camel_ident>(
                        self,
                        l: &'b Self::LImpl,
                        lo: &'c mut O::LImpl,
                    ) -> O {
                        #(
                            if let Some(x) = self.#sort_rs_snake_idents(l) {
                                let arg = x.convert(l, lo);
                                return O::#sort_rs_snake_idents(lo, arg);
                            }
                        )*
                        panic!("unreachable");
                    }
                }
            ));
        }
    }
}

mod mut_reference {
    use langspec_gen_util::collect;

    use super::*;
    pub fn generate<L: LangSpec>(base_path: &syn::Path, ls: &LangSpecGen<L>) -> syn::Item {
        let mut ret: Vec<syn::Item> = vec![];
        prods(&mut ret, base_path, ls);
        sums(&mut ret, base_path, ls);
        parse_quote!(
            pub mod mut_reference {
                pub trait NatLit<'a> {
                    type LImpl: #base_path::LImpl;
                }
                #(#ret)*
            }
        )
    }

    pub fn prods<L: LangSpec>(
        ret: &mut Vec<syn::Item>,
        base_path: &syn::Path,
        ls: &LangSpecGen<L>,
    ) {
        let byline = langspec_gen_util::byline!();
        for ProdGenData {
            camel_ident,
            sort_rs_camel_idents,
            idx,
            ty_idx,
            ..
        } in ls.prod_gen_datas()
        {
            collect!(sort_rs_camel_idents, ty_idx);
            let generate = quote::quote!(
                #byline
                pub trait #camel_ident<'a>: #(#base_path::Projection<Self::LImpl, #idx, To=Self::#ty_idx>)+*
                where #(
                    <Self::LImpl as #base_path::LImpl>::#sort_rs_camel_idents: 'a,
                )*
                {
                    type LImpl: #base_path::LImpl;
                    #(
                        type #ty_idx: #base_path::mut_reference::#sort_rs_camel_idents<'a>;
                    )*
                }
            );
            ret.push(parse_quote!(#generate));
        }
    }
    pub fn sums<L: LangSpec>(ret: &mut Vec<syn::Item>, base_path: &syn::Path, ls: &LangSpecGen<L>) {
        let byline = langspec_gen_util::byline!();
        for SumGenData {
            camel_ident,
            // sort_rs_types,
            sort_rs_camel_idents,
            sort_rs_snake_idents,
            ..
        } in ls.sum_gen_datas()
        {
            collect!(sort_rs_camel_idents);
            ret.push(parse_quote!(
                #byline
                pub trait #camel_ident<'a>: 'a {
                    type LImpl: #base_path::LImpl;
                    type Owned: #base_path::owned::#camel_ident;
                    #(
                        type #sort_rs_camel_idents: #base_path::mut_reference::#sort_rs_camel_idents<'a>;
                    )*
                    #(
                        fn #sort_rs_snake_idents<'b: 'a>(self, l: &'b Self::LImpl) -> Option<Self::#sort_rs_camel_idents>;
                    )*
                    fn set<'b: 'a>(
                        self,
                        l: &'b mut Self::LImpl,
                        value: Self::Owned,
                    );
                }
            ));
        }
    }
}

pub fn formatted(base_path: &syn::Path, lsh: &langspec::humanreadable::LangSpecHuman) -> String {
    let lsf: langspec::flat::LangSpecFlat =
        <langspec::flat::LangSpecFlat as langspec::langspec::TerminalLangSpec>::canonical_from(lsh);
    let gen_result = generate(base_path, &lsf);
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
            /// generated by [related_gen::generate]
            pub mod extension_of {
                /// generated by [related_gen::limpl_trait]
                pub trait LImpl: core::default::Default {
                    type NatLit: crate::owned::NatLit<LImpl = Self>;
                    type Plus: crate::owned::Plus<LImpl = Self>;
                    type F: crate::owned::F<LImpl = Self>;
                    type Nat: crate::owned::Nat<LImpl = Self>;
                }
                pub trait Projection<LImpl, const N: u8> {
                    type To;
                    fn project(self, l: &LImpl) -> Self::To;
                }
                /// generated by [related_gen::owned::generate]
                pub mod owned {
                    /// generated by [related_gen::owned::nat_lit]
                    pub trait NatLit: From<u64> {
                        type LImpl: crate::LImpl;
                        fn get_ref<'a, 'b: 'a>(
                            &'a self,
                            l: &'b Self::LImpl,
                        ) -> impl crate::reference::NatLit<'a, LImpl = Self::LImpl>;
                        fn get_mut<'a, 'b: 'a>(
                            &'a mut self,
                            l: &'b mut Self::LImpl,
                        ) -> impl crate::mut_reference::NatLit<'a, LImpl = Self::LImpl>;
                    }
                    /// generated by [related_gen::owned::prods]
                    pub trait Plus {
                        type LImpl: crate::LImpl;
                        fn new(
                            l: &mut Self::LImpl,
                            args: (
                                <Self::LImpl as crate::LImpl>::Nat,
                                <Self::LImpl as crate::LImpl>::Nat,
                            ),
                        ) -> Self;
                        fn get_ref<'a, 'b: 'a>(
                            &'a self,
                            l: &'b Self::LImpl,
                        ) -> impl crate::reference::Plus<'a, LImpl = Self::LImpl>;
                        fn get_mut<'a, 'b: 'a>(
                            &'a mut self,
                            l: &'b mut Self::LImpl,
                        ) -> impl crate::mut_reference::Plus<'a, LImpl = Self::LImpl>;
                    }
                    /// generated by [related_gen::owned::prods]
                    pub trait F {
                        type LImpl: crate::LImpl;
                        fn new(
                            l: &mut Self::LImpl,
                            args: (<Self::LImpl as crate::LImpl>::Nat,),
                        ) -> Self;
                        fn get_ref<'a, 'b: 'a>(
                            &'a self,
                            l: &'b Self::LImpl,
                        ) -> impl crate::reference::F<'a, LImpl = Self::LImpl>;
                        fn get_mut<'a, 'b: 'a>(
                            &'a mut self,
                            l: &'b mut Self::LImpl,
                        ) -> impl crate::mut_reference::F<'a, LImpl = Self::LImpl>;
                    }
                    /// generated by [related_gen::owned::sums]
                    pub trait Nat {
                        type LImpl: crate::LImpl;
                        fn nat_lit(
                            l: &mut Self::LImpl,
                            from: <Self::LImpl as crate::LImpl>::NatLit,
                        ) -> Self;
                        fn f(l: &mut Self::LImpl, from: <Self::LImpl as crate::LImpl>::F) -> Self;
                        fn plus(
                            l: &mut Self::LImpl,
                            from: <Self::LImpl as crate::LImpl>::Plus,
                        ) -> Self;
                        fn get_ref(
                            &self,
                            l: &Self::LImpl,
                        ) -> impl crate::reference::Nat<'_, LImpl = Self::LImpl>;
                        fn get_mut(
                            &mut self,
                            l: &mut Self::LImpl,
                        ) -> impl crate::mut_reference::Nat<'_, LImpl = Self::LImpl>;
                    }
                }
                /// generated by [related_gen::reference::generate]
                pub mod reference {
                    pub trait NatLit<'a>: Into<u64> {
                        type LImpl: crate::LImpl;
                        fn is_eq<'b: 'a>(self, l: &'b Self::LImpl, other: Self) -> bool;
                        fn convert<'b: 'a, 'c, O: crate::owned::NatLit>(
                            self,
                            _l: &'b Self::LImpl,
                            _lo: &'c mut O::LImpl,
                        ) -> O {
                            let intermediate: u64 = self.into();
                            O::from(intermediate)
                        }
                    }
                    /// generated by [related_gen::reference::prods]
                    pub trait Plus<
                        'a,
                    >: Copy + 'a + crate::Projection<
                            Self::LImpl,
                            0,
                            To = Self::T0,
                        > + crate::Projection<Self::LImpl, 1, To = Self::T1>
                    where
                        <Self::LImpl as crate::LImpl>::Nat: 'a,
                        <Self::LImpl as crate::LImpl>::Nat: 'a,
                    {
                        type LImpl: crate::LImpl;
                        type T0: crate::reference::Nat<'a, LImpl = Self::LImpl>;
                        type T1: crate::reference::Nat<'a, LImpl = Self::LImpl>;
                        fn is_eq<'b: 'a>(self, l: &'b Self::LImpl, other: Self) -> bool;
                        fn convert<'b: 'a, 'c, O: crate::owned::Plus>(
                            self,
                            l: &'b Self::LImpl,
                            lo: &'c mut O::LImpl,
                        ) -> O {
                            let args = (
                                <Self as crate::Projection<Self::LImpl, 0>>::project(self, l)
                                    .convert(l, lo),
                                <Self as crate::Projection<Self::LImpl, 1>>::project(self, l)
                                    .convert(l, lo),
                            );
                            O::new(lo, args)
                        }
                    }
                    /// generated by [related_gen::reference::prods]
                    pub trait F<'a>: Copy + 'a + crate::Projection<Self::LImpl, 0, To = Self::T0>
                    where
                        <Self::LImpl as crate::LImpl>::Nat: 'a,
                    {
                        type LImpl: crate::LImpl;
                        type T0: crate::reference::Nat<'a, LImpl = Self::LImpl>;
                        fn is_eq<'b: 'a>(self, l: &'b Self::LImpl, other: Self) -> bool;
                        fn convert<'b: 'a, 'c, O: crate::owned::F>(
                            self,
                            l: &'b Self::LImpl,
                            lo: &'c mut O::LImpl,
                        ) -> O {
                            let args = (
                                <Self as crate::Projection<Self::LImpl, 0>>::project(self, l)
                                    .convert(l, lo),
                            );
                            O::new(lo, args)
                        }
                    }
                    /// generated by [related_gen::reference::sums]
                    pub trait Nat<'a>: Copy + 'a {
                        type LImpl: crate::LImpl;
                        type NatLit: crate::reference::NatLit<'a, LImpl = Self::LImpl>;
                        type F: crate::reference::F<'a, LImpl = Self::LImpl>;
                        type Plus: crate::reference::Plus<'a, LImpl = Self::LImpl>;
                        fn nat_lit<'b: 'a>(self, l: &'b Self::LImpl) -> Option<Self::NatLit>;
                        fn f<'b: 'a>(self, l: &'b Self::LImpl) -> Option<Self::F>;
                        fn plus<'b: 'a>(self, l: &'b Self::LImpl) -> Option<Self::Plus>;
                        fn is_eq<'b: 'a>(self, l: &'b Self::LImpl, other: Self) -> bool;
                        fn convert<'b: 'a, 'c, O: crate::owned::Nat>(
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
                    pub trait NatLit<'a> {
                        type LImpl: crate::LImpl;
                    }
                    /// generated by [related_gen::mut_reference::prods]
                    pub trait Plus<
                        'a,
                    >: crate::Projection<
                            Self::LImpl,
                            0,
                            To = Self::T0,
                        > + crate::Projection<Self::LImpl, 1, To = Self::T1>
                    where
                        <Self::LImpl as crate::LImpl>::Nat: 'a,
                        <Self::LImpl as crate::LImpl>::Nat: 'a,
                    {
                        type LImpl: crate::LImpl;
                        type T0: crate::mut_reference::Nat<'a>;
                        type T1: crate::mut_reference::Nat<'a>;
                    }
                    /// generated by [related_gen::mut_reference::prods]
                    pub trait F<'a>: crate::Projection<Self::LImpl, 0, To = Self::T0>
                    where
                        <Self::LImpl as crate::LImpl>::Nat: 'a,
                    {
                        type LImpl: crate::LImpl;
                        type T0: crate::mut_reference::Nat<'a>;
                    }
                    /// generated by [related_gen::mut_reference::sums]
                    pub trait Nat<'a>: 'a {
                        type LImpl: crate::LImpl;
                        type Owned: crate::owned::Nat;
                        type NatLit: crate::mut_reference::NatLit<'a>;
                        type F: crate::mut_reference::F<'a>;
                        type Plus: crate::mut_reference::Plus<'a>;
                        fn nat_lit<'b: 'a>(self, l: &'b Self::LImpl) -> Option<Self::NatLit>;
                        fn f<'b: 'a>(self, l: &'b Self::LImpl) -> Option<Self::F>;
                        fn plus<'b: 'a>(self, l: &'b Self::LImpl) -> Option<Self::Plus>;
                        fn set<'b: 'a>(self, l: &'b mut Self::LImpl, value: Self::Owned);
                    }
                }
            }
        "#]];
        expected.assert_eq(&formatted);
    }
}
