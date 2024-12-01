use abstracted_langspec_gen::AbstractedLSGen;
use langspec::langspec::LangSpec;
use langspec_gen_util::{byline, transpose};
use syn::parse_quote;

pub fn generate<L: LangSpec>(base_path: &syn::Path, ls: &L) -> syn::ItemMod {
    let lg = AbstractedLSGen { bak: ls };
    let owned = owned::generate(base_path, &lg);
    let reference = reference::generate(base_path, &lg);
    let mut_reference = mut_reference::generate(base_path, &lg);
    let heap_trait = heap_trait(base_path, &lg);
    let byline = langspec_gen_util::byline!();
    parse_quote!(
        #byline
        pub mod extension_of {
            #heap_trait
            #owned
            #reference
            #mut_reference
        }
    )
}
pub(crate) fn heap_trait<L: LangSpec>(
    base_path: &syn::Path,
    ls: &AbstractedLSGen<L>,
) -> syn::ItemTrait {
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
    use abstracted_langspec_gen::{CanonicallyConstructibleFromGenData, TyGenData};

    use super::*;
    pub fn generate<L: LangSpec>(base_path: &syn::Path, ls: &AbstractedLSGen<L>) -> syn::ItemMod {
        let traits = ls.ty_gen_datas().map(
            |TyGenData {
                 camel_ident,
                 ccf: CanonicallyConstructibleFromGenData { ccf_sort_tys },
                 ..
             }|
             -> syn::ItemTrait {
                let ccf_sort_tys = ccf_sort_tys(&|camel_ident, _| {
                    syn::parse_quote! { <<Self as specialized_term::Heaped>::Heap as #base_path::extension_of::Heap>::#camel_ident }
                });
                let ccf_bounds = ccf_sort_tys.iter().map(|ccf| -> syn::TraitBound {
                    syn::parse_quote! {
                        specialized_term::CanonicallyConstructibleFrom<#ccf>
                    }
                });
                parse_quote! {
                    pub trait #camel_ident: specialized_term::Heaped #(+ #ccf_bounds )*
                    where <Self as specialized_term::Heaped>::Heap: #base_path::extension_of::Heap,
                    {
                    }
                }
            },
        );
        let byline = langspec_gen_util::byline!();
        parse_quote! {
            #byline
            mod owned {
                #(
                    #traits
                )*
            }
        }
    }
}
mod reference {
    use abstracted_langspec_gen::{
        CanonicallyConstructibleFromGenData, CanonicallyMaybeToGenData, TyGenData,
    };
    use langspec_gen_util::collect;

    use super::*;
    pub fn generate<L: LangSpec>(base_path: &syn::Path, ls: &AbstractedLSGen<L>) -> syn::ItemMod {
        let traits = ls.ty_gen_datas().map(
            |TyGenData {
                 camel_ident, cmt: CanonicallyMaybeToGenData {
                    cmt_sort_camels,
                 }, ..
             }| -> syn::ItemTrait {
                collect!(cmt_sort_camels);
                let trait_bounds = cmt_sort_camels.iter().map(|cmt_camel| -> syn::TraitBound {
                    syn::parse_quote! {
                        specialized_term::CanonicallyMaybeConvertibleTo<'heap, Self::#cmt_camel, specialized_term::ExpansionMaybeConversionFallibility>
                    }
                });
                syn::parse_quote! {
                    pub trait #camel_ident<'a, 'heap: 'a, Heap: #base_path::extension_of::Heap>: specialized_term::Heaped<Heap = Heap> #( + #trait_bounds )* {
                        #(
                            type #cmt_sort_camels: specialized_term::Heaped<Heap = Heap>;
                        )*
                    }
                }
            },
        );
        let byline = langspec_gen_util::byline!();
        parse_quote! {
            #byline
            mod reference {
                #(
                    #traits
                )*
            }
        }
    }
}
mod mut_reference {
    use super::*;
    pub fn generate<L: LangSpec>(base_path: &syn::Path, ls: &AbstractedLSGen<L>) -> syn::ItemMod {
        parse_quote! {
            mod mut_reference {

            }
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
