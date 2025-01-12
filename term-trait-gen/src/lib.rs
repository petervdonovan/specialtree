use langspec::langspec::LangSpec;
use langspec_gen_util::HeapType;
use langspec_gen_util::{LsGen, byline, transpose};
use syn::parse_quote;

use langspec_gen_util::{
    AlgebraicsBasePath, CanonicallyConstructibleFromGenData, CanonicallyMaybeToGenData, TyGenData,
};

pub fn generate<L: LangSpec>(base_path: &syn::Path, ls: &L) -> syn::ItemMod {
    let lg = LsGen::from(ls);
    let owned = owned::generate(base_path, &lg);
    let reference = reference::generate(base_path, &lg);
    // let mut_reference = mut_referenc::generate(base_path, &lg);
    let heap_trait = heap_trait(base_path, &lg);
    let byline = byline!();
    parse_quote!(
        #byline
        pub mod extension_of {
            #heap_trait
            #owned
            #reference
        }
    )
}
pub(crate) fn heap_trait<L: LangSpec>(base_path: &syn::Path, ls: &LsGen<L>) -> syn::ItemTrait {
    transpose!(ls.ty_gen_datas(), camel_ident);
    let byline = byline!();
    parse_quote! {
        #byline
        pub trait Heap {
            #(
                type #camel_ident: #base_path::extension_of::owned::#camel_ident<Heap = Self>;
            )*
        }
    }
}
mod owned {

    use super::*;
    pub fn generate<L: LangSpec>(base_path: &syn::Path, ls: &LsGen<L>) -> syn::ItemMod {
        let traits = ls.ty_gen_datas().map(
            |TyGenData {
                 camel_ident,
                 ccf: CanonicallyConstructibleFromGenData { ccf_sort_tys, .. },
                 ..
             }|
             -> syn::ItemTrait {
                let path = quote::quote! {
                    <<Self as term::Heaped>::Heap as #base_path::extension_of::Heap>
                };
                let ccf_sort_tys = ccf_sort_tys(
                    HeapType(syn::parse_quote! {<Self as term::Heaped>::Heap}),
                    AlgebraicsBasePath::new(quote::quote! { #path:: }),
                );
                let ccf_bounds = ccf_sort_tys.iter().map(|ccf| -> syn::TraitBound {
                    syn::parse_quote! {
                        term::CanonicallyConstructibleFrom<#ccf>
                    }
                });
                parse_quote! {
                    pub trait #camel_ident: term::Heaped #(+ #ccf_bounds )*
                    where <Self as term::Heaped>::Heap: #base_path::extension_of::Heap,
                    {
                    }
                }
            },
        );
        let byline = byline!();
        parse_quote! {
            #byline
            pub mod owned {
                #(
                    #traits
                )*
            }
        }
    }
}
mod reference {

    use super::*;
    pub fn generate<L: LangSpec>(base_path: &syn::Path, ls: &LsGen<L>) -> syn::ItemMod {
        let traits = ls.ty_gen_datas().map(
            |TyGenData {
                 camel_ident, cmt: CanonicallyMaybeToGenData {
                    cmt_sort_tys,
                    algebraic_cmt_sort_tys,
                 }, ..
             }| -> syn::ItemTrait {
                let cst = cmt_sort_tys(
                    HeapType(syn::parse_quote! {Heap}),
                    AlgebraicsBasePath::new(quote::quote! { Self:: })
                );
                let trait_bounds = cst.iter().map(|cmt| -> syn::TraitBound {
                    syn::parse_quote! {
                        term::CanonicallyMaybeConvertibleTo<'heap, #cmt, term::ExpansionMaybeConversionFallibility>
                    }
                });
                let cst = algebraic_cmt_sort_tys(
                    HeapType(syn::parse_quote! {Heap}),
                    AlgebraicsBasePath::new(syn::parse_quote! { })
                );
                let ret = quote::quote! {
                    pub trait #camel_ident<'a, 'heap: 'a, Heap: #base_path::extension_of::Heap>: term::Heaped<Heap = Heap> #( + #trait_bounds )* {
                        #(
                            type #cst: term::Heaped<Heap = Heap>;
                        )*
                    }
                };
                syn::parse_quote! {
                    #ret
                }
            },
        );
        let byline = byline!();
        parse_quote! {
            #byline
            pub mod reference {
                #(
                    #traits
                )*
            }
        }
    }
}

pub fn formatted(
    base_path: &syn::Path,
    lsh: &langspec::humanreadable::LangSpecHuman<tymetafuncspec_core::Core>,
) -> String {
    let lsf: langspec::flat::LangSpecFlat<tymetafuncspec_core::Core> = <langspec::flat::LangSpecFlat<
        tymetafuncspec_core::Core,
    > as langspec::langspec::TerminalLangSpec>::canonical_from(
        lsh
    );
    let gen_result = generate(base_path, &lsf);
    prettyplease::unparse(&syn::parse_quote! {
        #gen_result
    })
}
