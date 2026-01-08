use crate::type_path::{
    AlgebraicsBasePath, HeapType, sort2implementor_from_word_rs_ty, sort2rs_ty,
};
use aspect::VisitationAspect;
use langspec::langspec::MappedType;
use langspec::langspec::{
    AlgebraicSortId, LangSpec, MappedTypeOf, SortId, SortIdOf, call_on_all_tmf_monomorphizations,
};
use langspec::tymetafunc::{IdentifiedBy, RustTyMap, Transparency, TyMetaFuncData, TyMetaFuncSpec};
use memo::memo;
use memo::memo_cache::thread_local_cache;
use rustgen_utils::cons_list;
use tree_identifier::Identifier;

type HgdTyArgs<'a> =
    Box<dyn Fn(HeapType, AlgebraicsBasePath, Option<&syn::Path>) -> Vec<syn::Type> + 'a>;
type HgdHeapBak<'a> =
    Box<dyn Fn(&HeapType, &AlgebraicsBasePath, Option<&syn::Path>) -> syn::Type + 'a>;

pub struct TyGenData<'a, L: LangSpec> {
    pub id: AlgebraicSortId<L::ProductId, L::SumId>,
    pub snake_ident: syn::Ident,
    pub camel_ident: syn::Ident,
    pub ccf_sortses: Vec<Vec<SortIdOf<L>>>,
    pub ccf: CanonicallyConstructibleFromGenData<'a>,
    pub transparency: Transparency,
}
pub struct HeapbakGenData<'a> {
    pub identifiers: Vec<syn::Ident>,
    // pub ty_func: RustTyMap,
    pub heapbak: HgdHeapBak<'a>,
    pub ty_arg_camels: Vec<syn::Ident>,
    pub ty_args: HgdTyArgs<'a>,
    pub heap_sort_camel_ident: syn::Ident,
}

pub struct CanonicallyConstructibleFromGenData<'a> {
    pub ccf_sort_tys: Box<dyn Fn(HeapType, AlgebraicsBasePath) -> Vec<syn::Type> + 'a>,
    pub heap_sort_tys: Box<dyn Fn(HeapType, AlgebraicsBasePath) -> Vec<HstData> + 'a>,
    // pub ccf_sort_tyses: Box<dyn Fn(HeapType, AlgebraicsBasePath) -> Vec<Vec<syn::Type>> + 'a>,
    pub ccf_sort_transparencies: Box<dyn Fn() -> Vec<Vec<Transparency>> + 'a>,
    pub ccf_sort_camel_idents: Box<dyn Fn() -> Vec<Vec<syn::Ident>> + 'a>,
    pub ccf_sort_snake_idents: Box<dyn Fn() -> Vec<Vec<syn::Ident>> + 'a>,
}

pub struct HstData {
    pub heap_sort_ty: syn::Type,
    pub heap_sort_snake_ident: syn::Ident,
    pub heap_sort_camel_ident: syn::Ident,
}
#[memo('a)]
pub fn tmfs_monomorphizations<'a, L: LangSpec>(
    ls: &'a L,
) -> Vec<MappedType<L::ProductId, L::SumId, <L::Tmfs as TyMetaFuncSpec>::TyMetaFuncId>> {
    let mut ret = vec![];
    call_on_all_tmf_monomorphizations(ls, &mut |it| ret.push(it.clone()));
    ret
}
#[memo('a)]
pub fn ty_gen_datas<'a, L: LangSpec>(
    ls: &'a L,
    words_path: Option<syn::Path>,
) -> Vec<TyGenData<'a, L>> {
    let sort2rs_ty_fn = move |ht: HeapType, abp: AlgebraicsBasePath| {
        let words_path = words_path.clone();
        move |sort: SortIdOf<L>| match words_path {
            Some(ref words_path) => {
                sort2implementor_from_word_rs_ty(ls, sort, &ht, words_path, &VisitationAspect)
            }
            None => sort2rs_ty(ls, sort, &ht, &abp),
        }
    };
    ls.products()
        .map({
            let sort2rs_ty_fn = sort2rs_ty_fn.clone();
            move |pid| {
                let ccf_sortses = vec![ls.product_sorts(pid.clone()).collect()];
                TyGenData {
                    id: AlgebraicSortId::Product(pid.clone()),
                    snake_ident: ls.product_name(pid.clone()).snake_ident(),
                    camel_ident: ls.product_name(pid.clone()).camel_ident(),
                    ccf_sortses,
                    ccf: CanonicallyConstructibleFromGenData {
                        ccf_sort_tys: Box::new({
                            let pid = pid.clone();
                            let sort2rs_ty_fn = sort2rs_ty_fn.clone();
                            move |ht, abp| {
                                let fields_tys: Vec<syn::Type> = ls
                                    .product_sorts(pid.clone())
                                    .map(sort2rs_ty_fn(ht.clone(), abp.clone()))
                                    .collect();
                                vec![cons_list(fields_tys.into_iter())]
                            }
                        }),
                        heap_sort_tys: Box::new({
                            let pid = pid.clone();
                            move |ht, abp| {
                                ls.product_sorts(pid.clone())
                                    .zip(
                                        (ls.product_sorts(pid.clone()).map(|sort| {
                                            sort_ident(ls, sort, |name| {
                                                name.camel_str().to_string()
                                            })
                                        }))
                                        .zip(
                                            ls.product_sorts(pid.clone()).map(|sort| {
                                                sort_ident(ls, sort, |name| {
                                                    name.snake_str().to_string()
                                                })
                                            }),
                                        ),
                                    )
                                    .filter_map(|(sort, (camel, snake))| {
                                        // For now, return None since we don't have sort2heap_ty implemented
                                        None::<HstData>
                                    })
                                    .collect()
                            }
                        }),
                        ccf_sort_transparencies: Box::new({
                            let pid = pid.clone();
                            move || {
                                vec![
                                    ls.product_sorts(pid.clone())
                                        .map(|sort| sort2transparency::<L>(sort))
                                        .collect(),
                                ]
                            }
                        }),
                        ccf_sort_camel_idents: Box::new({
                            let pid = pid.clone();
                            move || {
                                vec![
                                    ls.product_sorts(pid.clone())
                                        .map(|sort| {
                                            sort_ident(ls, sort, |name| {
                                                name.camel_str().to_string()
                                            })
                                        })
                                        .collect(),
                                ]
                            }
                        }),
                        ccf_sort_snake_idents: Box::new({
                            let pid = pid.clone();
                            move || {
                                vec![
                                    ls.product_sorts(pid.clone())
                                        .map(|sort| {
                                            sort_ident(ls, sort, |name| {
                                                name.snake_str().to_string()
                                            })
                                        })
                                        .collect(),
                                ]
                            }
                        }),
                    },
                    transparency: Transparency::Visible,
                }
            }
        })
        .chain(ls.sums().map({
            let sort2rs_ty_fn = sort2rs_ty_fn.clone();
            move |sid| {
                let ccf_sortses = ls.sum_sorts(sid.clone()).map(|it| vec![it]).collect();
                TyGenData {
                    id: AlgebraicSortId::Sum(sid.clone()),
                    snake_ident: ls.sum_name(sid.clone()).snake_ident(),
                    camel_ident: ls.sum_name(sid.clone()).camel_ident(),
                    ccf_sortses,
                    ccf: CanonicallyConstructibleFromGenData {
                        ccf_sort_tys: Box::new({
                            let sid = sid.clone();
                            let sort2rs_ty_fn = sort2rs_ty_fn.clone();
                            move |ht, abp| {
                                ls.sum_sorts(sid.clone())
                                    .map(sort2rs_ty_fn(ht.clone(), abp.clone()))
                                    .map(|ty| syn::parse_quote! { (#ty,()) })
                                    .collect()
                            }
                        }),
                        heap_sort_tys: Box::new({
                            let sid = sid.clone();
                            move |ht, abp| {
                                ls.sum_sorts(sid.clone())
                                    .zip(
                                        (ls.sum_sorts(sid.clone()).map(|sort| {
                                            sort_ident(ls, sort, |name| {
                                                name.camel_str().to_string()
                                            })
                                        }))
                                        .zip(
                                            ls.sum_sorts(sid.clone()).map(|sort| {
                                                sort_ident(ls, sort, |name| {
                                                    name.snake_str().to_string()
                                                })
                                            }),
                                        ),
                                    )
                                    .filter_map(|(sort, (camel, snake))| {
                                        // For now, return None since we don't have sort2heap_ty implemented
                                        None::<HstData>
                                    })
                                    .collect()
                            }
                        }),
                        ccf_sort_transparencies: Box::new({
                            let sid = sid.clone();
                            move || {
                                ls.sum_sorts(sid.clone())
                                    .map(|sort| vec![sort2transparency::<L>(sort)])
                                    .collect()
                            }
                        }),
                        ccf_sort_camel_idents: Box::new({
                            let sid = sid.clone();
                            move || {
                                ls.sum_sorts(sid.clone())
                                    .map(|sort| {
                                        vec![sort_ident(ls, sort, |name| {
                                            name.camel_str().to_string()
                                        })]
                                    })
                                    .collect()
                            }
                        }),
                        ccf_sort_snake_idents: Box::new({
                            let sid = sid.clone();
                            move || {
                                ls.sum_sorts(sid.clone())
                                    .map(|sort| {
                                        vec![sort_ident(ls, sort, |name| {
                                            name.snake_str().to_string()
                                        })]
                                    })
                                    .collect()
                            }
                        }),
                    },
                    transparency: Transparency::Visible,
                }
            }
        }))
        .collect()
}

fn sortid_polish_name<L: LangSpec>(ls: &L, sort: &SortIdOf<L>) -> Vec<Identifier> {
    match sort {
        SortId::Algebraic(asi) => vec![ls.algebraic_sort_name(asi.clone()).clone()],
        SortId::TyMetaFunc(MappedType { f, a, .. }) => {
            let TyMetaFuncData { name, idby: _, .. } = L::Tmfs::ty_meta_func_data(f);
            let mut ret = vec![name.clone()];
            for arg in a {
                ret.extend(sortid_polish_name(ls, arg));
            }
            ret
        }
    }
}
#[memo('a)]
pub fn heapbak_gen_datas<'a, L: LangSpec>(ls: &'a L) -> Vec<HeapbakGenData<'a>> {
    tmfs_monomorphizations(thread_local_cache(), ls)
        .iter()
        .map(|tmf_m| {
            let tymetafuncdata = L::Tmfs::ty_meta_func_data(&tmf_m.f);
            // Recreate the original sortid_polish_name logic
            let sort_id = SortIdOf::<L>::TyMetaFunc(tmf_m.clone());
            let polish_name = sortid_polish_name(ls, &sort_id);
            let identifiers: Vec<syn::Ident> =
                polish_name.iter().map(|name| name.snake_ident()).collect();
            HeapbakGenData {
                identifiers,
                heapbak: Box::new({
                    let tymetafuncdata = tymetafuncdata.clone();
                    let tmf_m = tmf_m.clone();
                    move |ht, abp, words_path| {
                        let RustTyMap { ty_func } = &tymetafuncdata.heapbak;
                        let args = tmf_m.a.iter().map(|arg| match words_path {
                            Some(wp) => sort2implementor_from_word_rs_ty(
                                ls,
                                arg.clone(),
                                ht,
                                wp,
                                &VisitationAspect,
                            ),
                            None => sort2rs_ty(ls, arg.clone(), ht, abp),
                        });
                        let ht = &ht.0;
                        syn::parse_quote! { #ty_func<#ht, #( #args, )* > }
                    }
                }),
                ty_arg_camels: tmf_m
                    .a
                    .iter()
                    .map(|_| syn::Ident::new("T", proc_macro2::Span::call_site()))
                    .collect(),
                ty_args: Box::new({
                    let tmf_m = tmf_m.clone();
                    move |ht, abp, words_path| {
                        tmf_m
                            .a
                            .iter()
                            .map(|arg| match words_path {
                                Some(wp) => sort2implementor_from_word_rs_ty(
                                    ls,
                                    arg.clone(),
                                    &ht,
                                    wp,
                                    &VisitationAspect,
                                ),
                                None => sort2rs_ty(ls, arg.clone(), &ht, &abp),
                            })
                            .collect()
                    }
                }),
                heap_sort_camel_ident: tymetafuncdata.name.camel_ident(),
            }
        })
        .collect()
}

/// Get the CCF sortses for a type meta function
pub fn tmf_ccf_sortses<L: LangSpec>(mt: &MappedTypeOf<L>) -> Vec<Vec<SortIdOf<L>>> {
    let tmfd = <L::Tmfs as TyMetaFuncSpec>::ty_meta_func_data(&mt.f);
    tmfd.canonical_froms
        .iter()
        .map(|argids| argids.iter().map(|argid| mt.a[argid.0].clone()).collect())
        .collect()
}

fn sort2transparency<L: LangSpec>(sort: SortIdOf<L>) -> Transparency {
    match sort {
        langspec::langspec::SortId::Algebraic(AlgebraicSortId::Sum(_)) => Transparency::Transparent,
        langspec::langspec::SortId::Algebraic(AlgebraicSortId::Product(_)) => Transparency::Visible,
        langspec::langspec::SortId::TyMetaFunc(MappedType { f, a: _ }) => {
            L::Tmfs::ty_meta_func_data(&f).transparency
        }
    }
}

fn sort_ident<L: LangSpec>(
    ls: &L,
    sort: SortIdOf<L>,
    get_ident: fn(&Identifier) -> String,
) -> proc_macro2::Ident {
    match &sort {
        langspec::langspec::SortId::Algebraic(asi) => syn::Ident::new(
            &get_ident(ls.algebraic_sort_name(asi.clone())),
            proc_macro2::Span::call_site(),
        ),
        langspec::langspec::SortId::TyMetaFunc(MappedType { f, a }) => {
            let TyMetaFuncData { name, idby, .. } = L::Tmfs::ty_meta_func_data(f);
            syn::Ident::new(
                &match idby {
                    IdentifiedBy::Tmf => get_ident(&name),
                    IdentifiedBy::FirstTmfArg => {
                        let arg = a.first().unwrap();
                        let name = sortid_name(ls, arg);
                        get_ident(&name)
                    }
                },
                proc_macro2::Span::call_site(),
            )
        }
    }
}

fn sortid_name<L: LangSpec>(ls: &L, sort: &SortIdOf<L>) -> Identifier {
    match sort {
        langspec::langspec::SortId::Algebraic(asi) => ls.algebraic_sort_name(asi.clone()).clone(),
        langspec::langspec::SortId::TyMetaFunc(MappedType { f, a, .. }) => {
            let TyMetaFuncData { name, idby, .. } = L::Tmfs::ty_meta_func_data(f);
            match idby {
                IdentifiedBy::Tmf => name,
                IdentifiedBy::FirstTmfArg => sortid_name(ls, a.first().unwrap()),
            }
        }
    }
}
