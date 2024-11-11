use langspec::langspec::LangSpec;
use langspec_gen_util::{transpose, LangSpecGen, ProdGenData, SumGenData};
use syn::parse_quote;

pub fn gen<L: LangSpec>(base_path: &syn::Path, ls: &L) -> syn::Item {
    fn fail(_: &syn::Path, _: langspec::langspec::SortId<syn::Type>) -> syn::Type {
        panic!("must be type-agnostic");
    }
    let lg = LangSpecGen {
        bak: ls,
        sort2rs_type: fail,
        type_base_path: base_path.clone(),
    };
    let extension_of = gen_extension_of();
    let owned = owned::gen(base_path, &lg);
    let reference = reference::gen(base_path, &lg);
    let mut_reference = mut_reference::gen(base_path, &lg);
    let projection_trait = projection_trait();
    let limpl_trait = limpl_trait(base_path, &lg);
    let byline = langspec_gen_util::byline!();
    let any_type = any_type(base_path, &lg);
    parse_quote!(
        #byline
        pub mod extension_of {
            #limpl_trait
            #any_type
            #projection_trait
            #extension_of
            #owned
            #reference
            #mut_reference
        }
    )
}

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
            fn convert<
                LImpl: #base_path::LImpl<#(#prod_types = #prod_types,)* #(#sum_types = #sum_types,)*>,
                NatLit, #(#prod_types: #base_path::owned::#prod_types<LImpl = LImpl>,)* #(#sum_types: #base_path::owned::#sum_types<LImpl = LImpl>,)*
            >(&mut self, lo: &LImpl, from: #base_path::Any<
                NatLit, #(#prod_types,)* #(#sum_types,)*
            >) -> #base_path::Any<
                NatLit, #(Self::#prod_types,)* #(Self::#sum_types,)*
            > {
                #(
                    use #base_path::reference::#prod_types;
                )*
                #(
                    use #base_path::reference::#sum_types;
                )*
                match from {
                    #base_path::Any::NatLit(nat_lit) => #base_path::Any::NatLit(nat_lit),
                    #(
                        #base_path::Any::#prod_types(p) => #base_path::Any::#prod_types(p.get_ref(lo).convert(lo, self)),
                    )*
                    #(
                        #base_path::Any::#sum_types(s) => #base_path::Any::#sum_types(s.get_ref(lo).convert(lo, self)),
                    )*
                }
            }
        }
    )
}
fn any_type<L: LangSpec>(base_path: &syn::Path, ls: &LangSpecGen<L>) -> syn::Item {
    transpose!(ls.prod_gen_datas(), camel_ident);
    let prod_types = camel_ident;
    transpose!(ls.sum_gen_datas(), camel_ident);
    let sum_types = camel_ident;
    let byline = langspec_gen_util::byline!();
    parse_quote!(
        #byline
        pub enum Any<NatLit, #(#prod_types: #base_path::owned::#prod_types,)* #(#sum_types: #base_path::owned::#sum_types,)*> {
            NatLit(NatLit),
            #(
                #prod_types(#prod_types),
            )*
            #(
                #sum_types(#sum_types),
            )*
        }
    )
}
mod owned {
    use super::*;
    pub fn gen<L: LangSpec>(base_path: &syn::Path, ls: &LangSpecGen<L>) -> syn::Item {
        let mut ret: Vec<syn::Item> = vec![];
        prods(&mut ret, base_path, ls);
        sums(&mut ret, base_path, ls);
        let byline = langspec_gen_util::byline!();
        parse_quote!(
            #byline
            pub mod owned {
                pub trait NatLit: From<u64> {
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
            camel_ident: camel_name,
            sort_rs_camel_idents,
            ..
        } in ls.prod_gen_datas()
        {
            let gen = quote::quote!(
                #byline
                pub trait #camel_name {
                    type LImpl: #base_path::LImpl;
                    fn new(l: &mut Self::LImpl, args: (#(<Self::LImpl as #base_path::LImpl>::#sort_rs_camel_idents,)*)) -> Self;
                    fn get_ref<'a, 'b: 'a>(&'a self, l: &'b Self::LImpl) -> impl #base_path::reference::#camel_name<'a, LImpl = Self::LImpl>;
                    fn get_mut<'a, 'b: 'a>(&'a mut self, l: &'b mut Self::LImpl) -> impl #base_path::mut_reference::#camel_name<'a, LImpl = Self::LImpl>;
                }
            );
            ret.push(parse_quote!(#gen));
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
    pub fn gen<L: LangSpec>(base_path: &syn::Path, ls: &LangSpecGen<L>) -> syn::Item {
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
            let gen = quote::quote!(
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
            ret.push(parse_quote!(#gen));
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
    pub fn gen<L: LangSpec>(base_path: &syn::Path, ls: &LangSpecGen<L>) -> syn::Item {
        let mut ret: Vec<syn::Item> = vec![];
        prods(&mut ret, base_path, ls);
        sums(&mut ret, base_path, ls);
        parse_quote!(
            pub mod mut_reference {
                pub trait NatLit<'a> {
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
            let gen = quote::quote!(
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
            ret.push(parse_quote!(#gen));
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
