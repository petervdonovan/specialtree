use langspec::{
    flat::LangSpecFlat,
    humanreadable::LangSpecHuman,
    langspec::{AlgebraicSortId, LangSpec, TerminalLangSpec},
    tymetafunc::TyMetaFuncSpec,
};
use langspec_gen_util::{byline, AlgebraicsBasePath, HeapType, HeapbakGenData, LsGen, TyGenData};
use syn::parse_quote;

pub fn generate<L: LangSpec>(base_path: &syn::Path, lg: &LsGen<L>, serde: bool) -> syn::ItemMod {
    let algebraics = lg
        .ty_gen_datas()
        .filter(|it| it.id.is_some())
        .map(|tgd| alg_dt(serde, base_path, tgd));
    let heaped_impls = gen_heaped_impls(base_path, lg);
    let heap = gen_heap(
        base_path,
        &AlgebraicsBasePath::new(quote::quote! {#base_path ::}),
        lg,
    );
    let byline = byline!();
    parse_quote! {
        #byline
        pub mod data_structure {
            #heap
            #(#algebraics)*
            #heaped_impls
        }
    }
}

pub(crate) fn alg_dt<L: LangSpec>(
    serde: bool,
    base_path: &syn::Path,
    TyGenData {
        id,
        camel_ident,
        ccf,
        ..
    }: TyGenData<L>,
) -> syn::Item {
    let serde = if serde {
        quote::quote!(#[derive(serde::Serialize, serde::Deserialize)])
    } else {
        quote::quote!()
    };
    let sort_rs_types = (ccf.ccf_sort_tyses)(
        HeapType(parse_quote!(#base_path::Heap)),
        AlgebraicsBasePath::new(quote::quote!(#base_path::)),
    )
    .into_iter()
    .flatten();
    let ret = match id.unwrap() {
        AlgebraicSortId::Product(_) => {
            let sort_rs_snake_idents = (ccf.ccf_sort_snake_idents)().into_iter().flatten();
            quote::quote! {
                #serde
                #[derive(Clone, Copy)]
                pub struct #camel_ident {
                    #(pub #sort_rs_snake_idents: #sort_rs_types),*
                }
            }
        }
        _ => {
            let sort_rs_camel_idents = (ccf.ccf_sort_camel_idents)().into_iter().flatten();
            quote::quote! {
                #serde
                #[derive(Clone, Copy)]
                pub enum #camel_ident {
                    #(#sort_rs_camel_idents(#sort_rs_types)),*
                }
            }
        }
    };
    parse_quote!(#ret)
}

pub fn gen_heap<L: LangSpec>(
    base_path: &syn::Path,
    abp: &AlgebraicsBasePath,
    lg: &LsGen<L>,
) -> syn::File {
    let alg_snakes = lg.ty_gen_datas().map(|tgd| tgd.snake_ident);
    // let alg_camels = lg.ty_gen_datas().map(|tgd| tgd.camel_ident);
    // let alg_heapbaks = lg.ty_gen_datas().map(alg_heapbak);
    // let heapbak_paths = heapbak_paths(lg);
    let hgd = lg.heapbak_gen_datas();
    let heapbak_paths = hgd.iter().map(|hgd: &HeapbakGenData| -> syn::Path {
        let identifiers = &hgd.identifiers;
        syn::parse_quote! {#(#identifiers)::*}
    });
    let heapbak_modules = hgd
        .iter()
        .map(|hgd| gen_heapbak_module(base_path, abp, hgd))
        .collect::<Vec<_>>();
    let (module_names, modules_resolved) =
        gen_modules_with_prefix(base_path, &[], &heapbak_modules);
    let superheap_impls = hgd.iter().map(|hgd| {
        let ty_func = &hgd.ty_func.ty_func;
        let identifiers = &hgd.identifiers;
        // let abp = abp.to_token_stream();
        let ty_args = (hgd.ty_args)(HeapType(syn::parse_quote!{#base_path::Heap}), abp.clone());
        quote::quote! {
            term::impl_superheap!(#base_path::Heap ; #ty_func<#base_path::Heap, #(#ty_args),*> ; #(#identifiers)*);
        }
    });
    let byline = byline!();
    parse_quote! {
        #byline
        #[derive(Default)]
        pub struct Heap {
            #(#module_names: #base_path::heap::#module_names::Bak,)*
        }
        #byline
        pub mod heap {
            #(#modules_resolved)*
        }
        #byline
        pub mod superheap {
            #(#superheap_impls)*
        }
    }
}

pub(crate) fn gen_heapbak_module(
    heap_path: &syn::Path,
    abp: &AlgebraicsBasePath,
    hgd: &HeapbakGenData,
) -> (Vec<syn::Ident>, syn::Item) {
    let byline = byline!();
    // let abp = abp.to_token_stream();
    let heapbak_ty_func = &hgd.ty_func.ty_func;
    let heapbak_ty_args =
        (hgd.ty_args)(HeapType(syn::parse_quote! {#heap_path::Heap}), abp.clone());
    let ts = quote::quote! {
        #[derive(Default)]
        pub struct Bak(pub #heapbak_ty_func<#heap_path::Heap, #(#heapbak_ty_args),*>);
    };
    (
        hgd.identifiers.clone(),
        parse_quote! {
            #byline
            #ts
        },
    )
}

pub(crate) fn gen_modules_with_prefix(
    base_path: &syn::Path,
    prefix: &[syn::Ident],
    modules_by_prefix: &[(Vec<syn::Ident>, syn::Item)],
) -> (Vec<syn::Ident>, Vec<syn::Item>) {
    let byline = byline!();
    let matches = modules_by_prefix
        .iter()
        .filter(|(p, _)| p.iter().zip(prefix).all(|(a, b)| b == a))
        .collect::<Vec<_>>();
    let mut ret: Vec<syn::Item> = matches
        .iter()
        .filter_map(|(p, m)| {
            if p.len() == prefix.len() {
                Some(m.clone())
            } else {
                None
            }
        })
        .collect();
    let mut next_prefix_segments = vec![];
    for m in matches.iter().filter(|(p, _)| p.len() > prefix.len()) {
        let next_segment = &m.0[prefix.len()];
        if !next_prefix_segments
            .iter()
            .any(|s: &syn::Ident| s == next_segment)
        {
            next_prefix_segments.push(m.0[prefix.len()].clone());
        }
    }
    // for (p, m) in modules_by_prefix {
    //     if p.len() > prefix.len() {
    //         let subprefix = &p[prefix.len()..];
    //         let submodules = gen_modules_with_prefix(subprefix, modules_by_prefix);
    //         ret.push(parse_quote! {
    //             mod #m {
    //                 #(#submodules)*
    //             }
    //         });
    //     }
    // }
    if next_prefix_segments.is_empty() {
        return (vec![], ret);
    }
    let current_path = quote::quote! {#base_path ::heap #(::#prefix)*};
    ret.push(parse_quote! {
        #byline
        #[derive(Default)]
        pub struct Bak {
            #(pub #next_prefix_segments: #current_path::#next_prefix_segments::Bak,)*
        }
    });
    for next_segment in next_prefix_segments.iter() {
        let subprefix = prefix
            .iter()
            .chain(std::iter::once(next_segment))
            .cloned()
            .collect::<Vec<_>>();
        // ret.extend(gen_modules_with_prefix(&subprefix, modules_by_prefix));
        let (_, submodules) = gen_modules_with_prefix(base_path, &subprefix, modules_by_prefix);
        ret.push(parse_quote! {
            #byline
            pub mod #next_segment {
                #(#submodules)*
            }
        });
    }
    (next_prefix_segments, ret)
}

// pub(crate) fn alg_heapbak<L: LangSpec>(tgd: TyGenData<L>) -> syn::ItemStruct {
//     let camel_ident = tgd.camel_ident;
//     let hst = (tgd.ccf.heap_sort_tys)(
//         HeapType(parse_quote!(crate::data_structure::Heap)),
//         AlgebraicsBasePath::new(parse_quote!(crate::data_structure::heap::)),
//     );
//     transpose!(hst, heap_sort_ty, heap_sort_snake_ident);
//     let byline = byline!();
//     let ret = quote::quote! {
//         #byline
//         #[derive(Default)]
//         pub struct #camel_ident {
//             #(pub #heap_sort_snake_ident: #heap_sort_ty,)*
//         }
//     };
//     parse_quote! {
//         #ret
//     }
// }

pub(crate) fn gen_heaped_impls<L: LangSpec>(base_path: &syn::Path, lg: &LsGen<L>) -> syn::ItemMod {
    fn heaped_impl<L: LangSpec>(base_path: &syn::Path, tgd: TyGenData<L>) -> syn::ItemImpl {
        let camel_ident = tgd.camel_ident;
        let byline = byline!();
        let ret = quote::quote! {
            #byline
            impl term::Heaped for #base_path::#camel_ident {
                type Heap = #base_path::Heap;
            }
        };
        parse_quote! {
            #ret
        }
    }
    let byline = byline!();
    let impls = lg.ty_gen_datas().map(|tgd| heaped_impl(base_path, tgd));
    parse_quote! {
        #byline
        pub mod heaped {
            #(#impls)*
        }
    }
}

pub fn formatted<Tmfs: TyMetaFuncSpec>(lsh: &LangSpecHuman<Tmfs>) -> String {
    let lsf: LangSpecFlat<Tmfs> = LangSpecFlat::canonical_from(lsh);
    let lg = LsGen::from(&lsf);
    let m = generate(&parse_quote!(crate::data_structure), &lg, false);
    prettyplease::unparse(&syn::parse_quote! {
        #m
    })
}
