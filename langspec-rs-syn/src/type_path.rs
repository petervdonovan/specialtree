use aspect::Aspect;
use langspec::langspec::{LangSpec, MappedType, SortId, SortIdOf};
use langspec::tymetafunc::{RustTyMap, TyMetaFuncData, TyMetaFuncSpec};

#[derive(Clone)]
pub struct AlgebraicsBasePath(pub proc_macro2::TokenStream); // a prefix of a syn::TypePath
impl AlgebraicsBasePath {
    pub fn new(bp: proc_macro2::TokenStream) -> Self {
        if !(bp.to_string().is_empty() || bp.to_string().ends_with("::")) {
            panic!("AlgebraicsBasePath must end with '::' but instead is \"{bp}\"");
        }
        Self(bp)
    }
    pub fn to_token_stream(&self) -> proc_macro2::TokenStream {
        self.0.clone()
    }
}

#[derive(Clone)]
pub struct HeapType(pub syn::Type);

pub fn sort2rs_ty<L: LangSpec>(
    ls: &L,
    sort: SortIdOf<L>,
    ht: &HeapType,
    abp: &AlgebraicsBasePath,
) -> syn::Type {
    match sort {
        SortId::Algebraic(asi) => {
            let name = ls.algebraic_sort_name(asi);
            let ident = name.camel_ident();
            let abp = &abp.to_token_stream();
            syn::parse_quote! { #abp #ident }
        }
        SortId::TyMetaFunc(MappedType { f, a }) => {
            let TyMetaFuncData {
                imp: RustTyMap { ty_func },
                ..
            } = L::Tmfs::ty_meta_func_data(&f);
            let args = a.iter().map(|arg| sort2rs_ty(ls, arg.clone(), ht, abp));
            let ht = &ht.0;
            syn::parse_quote! { #ty_func<#ht, #( #args, )* > }
        }
    }
}

pub fn sort2word_rs_ty<L: LangSpec>(
    ls: &L,
    sort: SortIdOf<L>,
    words_path: &syn::Path,
) -> syn::Type {
    sort2rs_ty(
        ls,
        sort,
        &HeapType(syn::parse_quote! { () }),
        &AlgebraicsBasePath::new(syn::parse_quote! { #words_path::sorts:: }),
    )
}

// pub fn sort2structural_from_word_rs_ty<L: LangSpec>(
//     ls: &L,
//     sort: SortIdOf<L>,
//     HeapType(ht): &HeapType,
//     words_path: &syn::Path,
// ) -> syn::Type {
//     let word_rs_ty = sort2word_rs_ty(ls, sort, words_path);
//     syn::parse_quote! {
//         <#ht as words::InverseImplements<#words_path::L, #word_rs_ty, aspect::VisitationAspect>>::Implementor
//     }
// }

pub fn sort2implementor_from_word_rs_ty<L: LangSpec>(
    ls: &L,
    sort: SortIdOf<L>,
    HeapType(ht): &HeapType,
    words_path: &syn::Path,
    aspect: &dyn Aspect,
) -> syn::Type {
    let word_rs_ty = sort2word_rs_ty(ls, sort, words_path);
    let aspect_path = aspect.zst_path();
    syn::parse_quote! {
        <#ht as words::InverseImplements<#words_path::L, #word_rs_ty, #aspect_path>>::Implementor
    }
}

pub fn sort2heap_ty<L: LangSpec>(
    ls: &L,
    sort: SortIdOf<L>,
    ht: &HeapType,
    abp: &AlgebraicsBasePath,
    words_path: Option<&syn::Path>,
    aspect: &dyn Aspect,
) -> Option<syn::Type> {
    match &sort {
        SortId::Algebraic(_) => None,
        SortId::TyMetaFunc(MappedType { f, a }) => match words_path {
            Some(wp) => {
                let ty = sort2implementor_from_word_rs_ty(ls, sort, ht, wp, aspect);
                Some(syn::parse_quote! { <#ty as term::TyMetaFunc>::HeapBak })
            }
            None => {
                let TyMetaFuncData {
                    heapbak: RustTyMap { ty_func },
                    ..
                } = L::Tmfs::ty_meta_func_data(f);
                let args = a.iter().map(|arg| sort2rs_ty(ls, arg.clone(), ht, abp));
                let ht = &ht.0;
                Some(syn::parse_quote! { #ty_func<#ht, #( #args, )* > })
            }
        },
    }
}
