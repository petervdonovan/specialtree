use langspec::{
    langspec::{LangSpec, Name, SortIdOf},
    sublang::Sublang,
};
use langspec_gen_util::{AlgebraicsBasePath, HeapType, LsGen, byline};

pub struct BasePaths {
    pub ext_data_structure: syn::Path,
    pub og_term_trait: syn::Path,
}

pub fn generate<L: LangSpec>(
    ext_lg: &LsGen<L>,
    sublang_name: &Name,
    bps: &BasePaths,
) -> proc_macro2::TokenStream {
    let BasePaths {
        ext_data_structure,
        og_term_trait: _,
    } = bps;
    let sublang = ext_lg
        .bak()
        .sublangs()
        .into_iter()
        .find(|sublang| sublang.name == *sublang_name)
        .expect("Sublang not found");
    let camel_names = sublang
        .ty_names
        .iter()
        .map(|name| syn::Ident::new(&name.camel, proc_macro2::Span::call_site()))
        .collect::<Vec<_>>();
    let image_ty_under_embeddings = sublang
        .ty_names
        .iter()
        .map(&sublang.map)
        .map(|sort| {
            ext_lg.sort2rs_ty(
                sort,
                &HeapType(syn::parse_quote! {#ext_data_structure::Heap}),
                &AlgebraicsBasePath::new(quote::quote! {#ext_data_structure::}),
            )
        })
        .collect::<Vec<_>>();
    let heap = generate_heap(&camel_names, &image_ty_under_embeddings, bps);
    let owned_impls = generate_owned_impls(&camel_names, &image_ty_under_embeddings, bps);
    let words_impls = bridge_words_impls(bps, ext_lg);
    let maps_tmf_impls = generate_maps_tmf_impls(ext_lg, &sublang, bps);
    quote::quote! {
        #heap
        #words_impls
        #owned_impls
        #maps_tmf_impls
    }
}

pub(crate) fn bridge_words_impls<L: LangSpec>(
    BasePaths {
        ext_data_structure,
        og_term_trait,
    }: &BasePaths,
    elsg: &LsGen<L>,
) -> syn::ItemMod {
    let og_words_base_path: syn::Path = syn::parse_quote! {
        #og_term_trait::words
    };
    let impls = elsg
        .ty_gen_datas(Some(og_words_base_path.clone()))
        .map(|tgd| -> syn::ItemImpl {
            let camel_ident = &tgd.camel_ident;
            let ty: syn::Type = syn::parse_quote! {
                <#ext_data_structure::Heap as #og_term_trait::Heap>::#camel_ident
            };
            syn::parse_quote! {
                impl words::Implements<#og_words_base_path::L> for #ty {
                    type LWord = #og_words_base_path::sorts::#camel_ident;
                }
            }
        });
    let byline = byline!();
    syn::parse_quote! {
        #byline
        pub mod words_impls {
            #(#impls)*
        }
    }
}

pub(crate) fn generate_heap(
    camel_names: &[syn::Ident],
    image_ty_under_embeddings: &[syn::Type],
    BasePaths {
        ext_data_structure,
        og_term_trait,
    }: &BasePaths,
) -> syn::ItemImpl {
    let byline = langspec_gen_util::byline!();
    syn::parse_quote! {
        #byline
        impl #og_term_trait::Heap for #ext_data_structure::Heap {
            #(
                type #camel_names = #image_ty_under_embeddings;
            )*
        }
    }
}

pub(crate) fn generate_owned_impls(
    camel_names: &[syn::Ident],
    image_ty_under_embeddings: &[syn::Type],
    BasePaths {
        ext_data_structure: _,
        og_term_trait,
    }: &BasePaths,
) -> syn::ItemMod {
    let byline = langspec_gen_util::byline!();
    syn::parse_quote! {
        #byline
        pub mod owned_impls {
            #(
                impl #og_term_trait::owned::#camel_names for #image_ty_under_embeddings {}
            )*
        }
    }
}

pub(crate) fn generate_maps_tmf_impls<L: LangSpec>(
    ext_lg: &LsGen<L>,
    sublang: &Sublang<SortIdOf<L>>,
    BasePaths {
        ext_data_structure,
        og_term_trait,
    }: &BasePaths,
) -> syn::ItemMod {
    let byline = langspec_gen_util::byline!();
    let (ogtys, mapped_tys) = sublang
        .tems
        .iter()
        .map(|tmf| {
            let ogty = ext_lg.sort2rs_ty(
                tmf.from.clone(),
                &HeapType(syn::parse_quote! {#ext_data_structure::Heap}),
                &AlgebraicsBasePath::new(quote::quote! {#ext_data_structure::}),
            );
            let mapped_ty = ext_lg.sort2rs_ty(
                tmf.to.clone(),
                &HeapType(syn::parse_quote! {#ext_data_structure::Heap}),
                &AlgebraicsBasePath::new(quote::quote! {#ext_data_structure::}),
            );
            (ogty, mapped_ty)
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();
    syn::parse_quote! {
        #byline
        pub mod maps_tmf_impls {
            #(
                impl term::MapsTmf<#og_term_trait::words::L, #ogtys> for #ext_data_structure::Heap {
                    type Tmf = #mapped_tys;
                }
            )*
        }
    }
}
