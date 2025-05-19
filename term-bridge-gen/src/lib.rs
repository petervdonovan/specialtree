use langspec::{
    langspec::{LangSpec, Name, SortIdOf},
    sublang::Sublang,
};
use langspec_gen_util::{AlgebraicsBasePath, HeapType, LsGen, byline};

pub struct BasePaths {
    pub ext_data_structure: syn::Path,
    pub og_term_trait: syn::Path,
    pub og_words_base_path: syn::Path,
}

pub fn generate<L: LangSpec, LSub: LangSpec>(
    ext_lg: &LsGen<L>,
    oglsg: &LsGen<LSub>,
    sublang_name: &Name,
    bps: &BasePaths,
) -> syn::ItemMod {
    let BasePaths {
        ext_data_structure,
        og_term_trait: _,
        og_words_base_path: _,
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
    let words_impls = bridge_words_impls(bps, &sublang, oglsg, ext_lg);
    let maps_tmf_impls = generate_maps_tmf_impls(ext_lg, &sublang, bps);
    syn::parse_quote! {
        mod bridge {
            #heap
            #owned_impls
            #words_impls
            #maps_tmf_impls
        }
    }
}

pub(crate) fn bridge_words_impls<L: LangSpec, LSub: LangSpec>(
    BasePaths {
        ext_data_structure,
        og_term_trait,
        og_words_base_path,
    }: &BasePaths,
    sublang: &Sublang<'_, SortIdOf<L>>,
    oglsg: &LsGen<LSub>,
    elsg: &LsGen<L>,
) -> syn::ItemMod {
    // let impls = elsg
    //     .ty_gen_datas(Some(og_words_base_path.clone()))
    //     .map(|tgd| -> syn::ItemImpl {
    //         let camel_ident = &tgd.camel_ident;
    //         let ty: syn::Type = syn::parse_quote! {
    //             <#ext_data_structure::Heap as #og_term_trait::Heap>::#camel_ident
    //         };
    //         syn::parse_quote! {
    //             impl words::Implements<#ext_data_structure::Heap, #og_words_base_path::L> for #ty {
    //                 type LWord = #og_words_base_path::sorts::#camel_ident;
    //             }
    //         }
    //     });
    let impls = oglsg
        .bak()
        .all_sort_ids()
        .map(|sid| {
            let skeleton = oglsg.sort2rs_ty(
                sid.clone(),
                &HeapType(syn::parse_quote! { () }),
                &AlgebraicsBasePath::new(syn::parse_quote! { #og_words_base_path::sorts:: }),
            );
            let ty = elsg.sort2rs_ty(
                (sublang.map)(todo!()),
                &HeapType(syn::parse_quote! { #ext_data_structure::Heap }),
                &AlgebraicsBasePath::new(syn::parse_quote! { #ext_data_structure::Heap:: }),
            );
            (ty, skeleton)
        })
        .map(|(ty, skeleton)| -> syn::ItemImpl {
            syn::parse_quote! {
                impl words::Implements<#ext_data_structure::Heap, #og_words_base_path::L> for #ty {
                    type LWord = #skeleton;
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
        og_words_base_path: _,
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
        ext_data_structure,
        og_term_trait,
        og_words_base_path: _,
    }: &BasePaths,
) -> syn::ItemMod {
    let byline = langspec_gen_util::byline!();
    syn::parse_quote! {
        #byline
        pub mod owned_impls {
            #(
                impl #og_term_trait::owned::#camel_names<#ext_data_structure::Heap> for #image_ty_under_embeddings {}
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
        og_words_base_path,
    }: &BasePaths,
) -> syn::ItemMod {
    let byline = langspec_gen_util::byline!();
    let (ogtys, mapped_tys) = sublang
        .tems
        .iter()
        .map(|tmf| {
            let ogty_shallow = ext_lg.sort2rs_ty(
                tmf.fromshallow.clone(),
                &HeapType(syn::parse_quote! {#ext_data_structure::Heap}),
                &AlgebraicsBasePath::new(quote::quote! { #ext_data_structure:: }),
            );
            let ogty_rec = ext_lg.sort2rs_ty(
                tmf.fromrec.clone(),
                &HeapType(syn::parse_quote! {#ext_data_structure::Heap}),
                &AlgebraicsBasePath::new(
                    quote::quote! {<#ext_data_structure::Heap as #og_term_trait::Heap>::},
                ),
            );
            let mapped_ty = ext_lg.sort2rs_ty(
                tmf.to.clone(),
                &HeapType(syn::parse_quote! {#ext_data_structure::Heap}),
                &AlgebraicsBasePath::new(quote::quote! {#ext_data_structure::}),
            );
            ((ogty_shallow, ogty_rec), mapped_ty)
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();
    let (ogtys_shallow, ogtys_rec) = ogtys.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();
    syn::parse_quote! {
        #byline
        pub mod maps_tmf_impls {
            #(
                impl term::MapsTmf<#og_words_base_path::L, #ogtys_rec> for #ext_data_structure::Heap {
                    type TmfFrom = #ogtys_shallow;
                    type TmfTo = #mapped_tys;
                }
            )*
        }
    }
}

pub mod targets {
    use std::path::Path;

    use codegen_component::{CgDepList, CodegenInstance, bumpalo};
    use langspec_gen_util::kebab_id;

    pub fn default<'langs, L: super::LangSpec, LSub: super::LangSpec>(
        arena: &'langs bumpalo::Bump,
        mut codegen_deps: CgDepList<'langs>,
        l: &'langs L,
        lsublang: &'langs LSub,
    ) -> CodegenInstance<'langs> {
        let ext_lg = super::LsGen::from(l);
        let oglsg = super::LsGen::from(l);
        CodegenInstance {
            id: kebab_id!(l),
            generate: {
                let sublang_name = lsublang.name().clone();
                let ext_data_structure = codegen_deps.add(term_specialized_gen::targets::default(
                    arena,
                    codegen_deps.subtree(),
                    l,
                ));
                let og_term_trait = codegen_deps.add(term_trait_gen::targets::default(
                    arena,
                    codegen_deps.subtree(),
                    lsublang,
                ));
                let og_words_base_path = codegen_deps.add(words::targets::words_mod(
                    arena,
                    codegen_deps.subtree(),
                    lsublang,
                ));
                // let _ = codegen_deps.add(term_specialized_impl_gen::targets::default(
                //     arena,
                //     codegen_deps.subtree(),
                //     l,
                // ));
                let _ =
                    codegen_deps.add(term_specialized_impl_gen::targets::term_specialized_impl(
                        arena,
                        codegen_deps.subtree(),
                        l,
                        &[l.name().clone(), lsublang.name().clone()],
                    ));
                Box::new(move |c2sp, _| {
                    super::generate(
                        &ext_lg,
                        &oglsg,
                        &sublang_name,
                        &crate::BasePaths {
                            ext_data_structure: ext_data_structure(c2sp),
                            og_term_trait: og_term_trait(c2sp),
                            og_words_base_path: og_words_base_path(c2sp),
                        },
                    )
                })
            },
            external_deps: vec![],
            workspace_deps: vec![
                ("term-bridge-gen", Path::new(".")),
                ("words", Path::new(".")),
            ],
            codegen_deps,
        }
    }
}
