use langspec::langspec::{LangSpec, ToLiteral};
use langspec_gen_util::{AlgebraicsBasePath, HeapType, LsGen, byline};

pub fn generate<L: LangSpec>(
    words_path: &syn::Path,
    lg: &LsGen<L>,
    l_sortid_ty_literal: syn::Type,
) -> syn::ItemMod {
    let byline = byline!();
    let sid_exprs = lg.bak().all_sort_ids().map(|it| it.to_literal());
    let tys = lg.bak().all_sort_ids().map(|sid| {
        lg.sort2rs_ty(
            sid,
            &HeapType(syn::parse_quote! {Heap}),
            &AlgebraicsBasePath::new(syn::parse_quote! {abp::}),
        )
    });
    let impls = tys.zip(sid_exprs).map(|(ty, sid_expr)| -> syn::ItemImpl {
        syn::parse_quote! {
            impl has_own_sort_id::HasOwnSortId<#words_path::L, #l_sortid_ty_literal> for #ty {
                fn own_sort_id() -> #l_sortid_ty_literal {
                    #sid_expr
                }
            }
        }
    });
    syn::parse_quote! {
        #byline
        pub mod has_own_sort {
            #(#impls)*
        }
    }
}
