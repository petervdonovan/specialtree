use langspec::langspec::{MappedTypeOf, call_on_all_tmf_monomorphizations};
use langspec::sublang::Sublangs;
use langspec::tymetafunc::{IdentifiedBy, RustTyMap, Transparency, TyMetaFuncSpec};
use langspec::{
    langspec::{AlgebraicSortId, LangSpec, MappedType, SortId, SortIdOf},
    tymetafunc::TyMetaFuncData,
};
use rustgen_utils::cons_list;
use term::CcfRelation;
use transitive_ccf::{CcfPaths, UcpPair, get_direct_ccf_rels, unit_ccf_paths_quadratically_large_closure, ccfs_exploded_by_unit_paths, find_ucp};
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
}

#[derive(Clone)]
pub struct AlgebraicsBasePath(proc_macro2::TokenStream); // a prefix of a syn::TypePath
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
pub struct LsGen<'a, L: LangSpec> {
    bak: &'a L,
}
impl<'a, L: LangSpec> From<&'a L> for LsGen<'a, L> {
    fn from(bak: &'a L) -> Self {
        Self { bak }
    }
}
// pub struct Implementors {
//     structural_implementor: syn::Type,
//     behavioral_implementor: syn::Type,
//     word: syn::Type,
// }
impl<L: LangSpec> LsGen<'_, L> {
    pub fn bak(&self) -> &L {
        self.bak
    }
    pub fn ccf_paths(
        &self,
        important_sublangs: impl Sublangs<SortIdOf<L>>,
    ) -> CcfPaths<SortIdOf<L>> {
        let direct_ccf_rels = get_direct_ccf_rels(self.bak);
        let mut ucp_acc = std::collections::HashSet::new();
        let mut cebup_acc = std::collections::HashSet::new();
        // let sublangs = self
        //     .bak
        //     .sublangs()
        //     .into_iter()
        //     .filter(|it| important_sublangs.contains(&it.name))
        //     .collect::<Vec<_>>();
        for non_transparent_sorts in important_sublangs.images() {
            let ucp = unit_ccf_paths_quadratically_large_closure::<SortIdOf<L>>(
                direct_ccf_rels.as_slice(),
                &non_transparent_sorts,
            );
            let cebup = ccfs_exploded_by_unit_paths::<SortIdOf<L>>(
                direct_ccf_rels.as_slice(),
                &ucp,
                &non_transparent_sorts,
            );
            let cebup_filtered = cebup
                .into_iter()
                .filter(|it| {
                    it.from.iter().all(|it| non_transparent_sorts.contains(it))
                        && non_transparent_sorts.contains(&it.to)
                })
                .collect::<Vec<_>>();
            ucp_acc.extend(ucp.into_iter().filter(|it| {
                non_transparent_sorts.contains(&it.from)
                    || cebup_filtered
                        .iter()
                        .any(|cebup| cebup.intermediary.to == it.from)
            }));
            cebup_acc.extend(cebup_filtered);
        }
        // for desired_pair in sublangs
        //     .into_iter()
        //     .flat_map(|it| it.tems)
        //     .filter(|it| it.fromshallow != it.to)
        //     .filter(|it| {
        //         !ucp_acc
        //             .iter()
        //             .any(|existing| existing.from == it.fromshallow && existing.to == it.to)
        //     })
        //     .collect::<Vec<_>>()
        //     .into_iter()
        let all_tems = important_sublangs
            .tems()
            .flatten()
            .filter(|it| it.from_extern_behavioral != it.to_structural)
            .filter(|it| {
                !ucp_acc.iter().any(|existing| {
                    existing.from == it.from_extern_behavioral && existing.to == it.to_structural
                })
            })
            .collect::<Vec<_>>();
        for desired_pair in all_tems.into_iter() {
            // todo: iterate over the existing ucps targeting anything under tmfs that don't have a unit ccf,
            // starting of course with the reflexive ucp, until you exhaust the combinations. Only then, panic.
            // Yes I know this is potentially very inefficient. Is it time to rethink this? Is it appropriate, now, to "preserve, don't recover?" Or is it appropriate only to update fromshallow?
            ucp_acc.insert(
                find_ucp(
                    direct_ccf_rels.as_slice(),
                    UcpPair {
                        from: desired_pair.from_extern_behavioral.clone(),
                        to: desired_pair.to_structural.clone(),
                    },
                )
                .unwrap_or_else(|| panic!("Cannot find UCP for {:?}", &desired_pair)),
            );
        }
        let mut units_sorted = ucp_acc.into_iter().collect::<Vec<_>>();
        units_sorted.sort();
        let mut non_units_sorted = cebup_acc.into_iter().collect::<Vec<_>>();
        non_units_sorted.sort();
        CcfPaths {
            units: units_sorted,
            non_units: non_units_sorted,
        }
    }
    pub fn tmfs_monomorphizations(
        &self,
    ) -> impl Iterator<
        Item = MappedType<L::ProductId, L::SumId, <L::Tmfs as TyMetaFuncSpec>::TyMetaFuncId>,
    > {
        let mut ret = vec![];
        call_on_all_tmf_monomorphizations(self.bak, &mut |it| ret.push(it.clone()));
        ret.into_iter()
    }
    pub fn ty_gen_datas(
        &self,
        words_path: Option<syn::Path>,
    ) -> impl Iterator<Item = TyGenData<'_, L>> {
        let sort2rs_ty = move |ht: HeapType, abp: AlgebraicsBasePath| {
            let words_path = words_path.clone();
            move |sort: SortIdOf<L>| match words_path {
                Some(ref words_path) => self.sort2structural_from_word_rs_ty(sort, &ht, words_path),
                None => self.sort2rs_ty(sort, &ht, &abp),
            }
        };
        self.bak
            .products()
            .map({
                let sort2rs_ty = sort2rs_ty.clone();
                move |pid| {
                    let ccf_sortses = vec![self.bak.product_sorts(pid.clone()).collect()];
                    TyGenData {
                        id: AlgebraicSortId::Product(pid.clone()),
                        // fingerprint: pid_fingerprint(self.bak, &pid),
                        snake_ident: self.bak.product_name(pid.clone()).snake_ident(),
                        camel_ident: self.bak.product_name(pid.clone()).camel_ident(),
                        ccf_sortses,
                        ccf: CanonicallyConstructibleFromGenData {
                            ccf_sort_tys: Box::new({
                                let pid = pid.clone();
                                let sort2rs_ty = sort2rs_ty.clone();
                                move |ht, abp| {
                                    let fields_tys = self
                                        .bak
                                        .product_sorts(pid.clone())
                                        .map(sort2rs_ty(ht, abp));
                                    vec![cons_list(fields_tys)]
                                }
                            }),
                            ccf_sort_transparencies: {
                                let pid = pid.clone();
                                Box::new(move || {
                                    vec![
                                        self.bak
                                            .product_sorts(pid.clone())
                                            .map(|sort| self.sort2transparency(sort))
                                            .collect(),
                                    ]
                                })
                            },
                            heap_sort_tys: Box::new({
                                let pid = pid.clone();
                                move |ht, abp| {
                                    self.bak
                                        .product_sorts(pid.clone())
                                        .zip(
                                            (self.product_heap_sort_camel_idents(&pid).into_iter())
                                                .zip(self.product_heap_sort_snake_idents(&pid)),
                                        )
                                        .filter_map(|(sort, (camel, snake))| {
                                            self.sort2heap_ty(sort, &ht, &abp, None).map(
                                                |heap_sort_ty| HstData {
                                                    heap_sort_ty,
                                                    heap_sort_snake_ident: snake,
                                                    heap_sort_camel_ident: camel,
                                                },
                                            )
                                        })
                                        .collect()
                                }
                            }),
                            // ccf_sort_tyses: Box::new({
                            //     let pid = pid.clone();
                            //     let sort2rs_ty = sort2rs_ty.clone();
                            //     move |ht, abp| {
                            //         vec![
                            //             self.bak
                            //                 .product_sorts(pid.clone())
                            //                 .map(sort2rs_ty(ht, abp))
                            //                 .collect(),
                            //         ]
                            //     }
                            // }),
                            ccf_sort_camel_idents: Box::new({
                                let pid = pid.clone();
                                move || vec![self.product_heap_sort_camel_idents(&pid)]
                            }),
                            ccf_sort_snake_idents: Box::new(move || {
                                vec![self.product_heap_sort_snake_idents(&pid)]
                            }),
                        },
                        transparency: Transparency::Visible,
                    }
                }
            })
            .chain(self.bak.sums().map(move |sid| {
                let ccf_sortses = self.bak.sum_sorts(sid.clone()).map(|it| vec![it]).collect();
                TyGenData {
                    id: AlgebraicSortId::Sum(sid.clone()),
                    snake_ident: self.bak.sum_name(sid.clone()).snake_ident(),
                    camel_ident: self.bak.sum_name(sid.clone()).camel_ident(),
                    ccf_sortses,
                    ccf: CanonicallyConstructibleFromGenData {
                        ccf_sort_tys: Box::new({
                            let sid = sid.clone();
                            let sort2rs_ty = sort2rs_ty.clone();
                            move |ht, abp| {
                                self.bak
                                    .sum_sorts(sid.clone())
                                    .map(sort2rs_ty(ht, abp))
                                    .map(|ty| syn::parse_quote! { (#ty,()) })
                                    .collect()
                            }
                        }),
                        heap_sort_tys: Box::new({
                            let sid = sid.clone();
                            move |ht, abp| {
                                self.bak
                                    .sum_sorts(sid.clone())
                                    .zip(
                                        (self.sum_heap_sort_camel_idents(&sid).into_iter())
                                            .zip(self.sum_heap_sort_snake_idents(&sid)),
                                    )
                                    .filter_map(|(sort, (camel, snake))| {
                                        self.sort2heap_ty(sort, &ht, &abp, None).map(
                                            |heap_sort_ty| HstData {
                                                heap_sort_ty,
                                                heap_sort_snake_ident: snake,
                                                heap_sort_camel_ident: camel,
                                            },
                                        )
                                    })
                                    .collect()
                            }
                        }),
                        ccf_sort_transparencies: {
                            let sid = sid.clone();
                            Box::new(move || {
                                self.bak
                                    .sum_sorts(sid.clone())
                                    .map(|it| vec![self.sort2transparency(it)])
                                    .collect()
                            })
                        },
                        // ccf_sort_tyses: Box::new({
                        //     let sid = sid.clone();
                        //     let sort2rs_ty = sort2rs_ty.clone();
                        //     move |ht, abp| {
                        //         self.bak
                        //             .sum_sorts(sid.clone())
                        //             .map(sort2rs_ty(ht, abp))
                        //             .map(|it| vec![it])
                        //             .collect()
                        //     }
                        // }),
                        ccf_sort_camel_idents: Box::new({
                            let sid = sid.clone();
                            move || {
                                self.sum_heap_sort_camel_idents(&sid)
                                    .into_iter()
                                    .map(|it| vec![it])
                                    .collect()
                            }
                        }),
                        ccf_sort_snake_idents: Box::new(move || {
                            self.sum_heap_sort_snake_idents(&sid)
                                .into_iter()
                                .map(|it| vec![it])
                                .collect()
                        }),
                    },
                    transparency: Transparency::Transparent,
                }
            }))
    }
    pub fn heapbak_gen_datas(&self) -> Vec<HeapbakGenData> {
        let mut ret: Vec<HeapbakGenData> = vec![];
        call_on_all_tmf_monomorphizations(self.bak, &mut |mt| {
            let sid = SortId::TyMetaFunc(mt.clone());
            let polish_name = sortid_polish_name(self.bak, &sid);
            let ty_arg_camels = polish_name
                .iter()
                .map(|name| name.camel_ident())
                .collect();
            let ty_arg_snakes = polish_name
                .iter()
                .map(|name| name.snake_ident());
            // let ty_func = <L::Tmfs as TyMetaFuncSpec>::ty_meta_func_data(&mt.f).heapbak;
            ret.push(HeapbakGenData {
                identifiers: ty_arg_snakes.collect(),
                // ty_func,
                heapbak: Box::new(move |ht, abp, words_path| {
                    self.sort2heap_ty(sid.clone(), ht, abp, words_path).unwrap()
                }),
                ty_arg_camels,
                ty_args: Box::new({
                    let a = mt.a.clone();
                    move |ht, abp, words_path| {
                        a.iter()
                            .map(|arg| match words_path {
                                Some(words_path) => self.sort2structural_from_word_rs_ty(
                                    arg.clone(),
                                    &ht,
                                    words_path,
                                ),
                                None => self.sort2rs_ty(arg.clone(), &ht, &abp),
                            })
                            .collect()
                    }
                }),
            });
        });
        ret
    }
    // pub fn implementors<LSub: LangSpec>(
    //     &self,
    //     sublang: Sublang<LSub, SortIdOf<L>>,
    //     og_words_base_path: &syn::Path,
    //     ext_data_structure_path: &syn::Path,
    // ) -> impl Iterator<Item = Implementors> {
    //     sublang.lsub.all_sort_ids().map(move |sid| {
    //         let word = LsGen::from(sublang.lsub).sort2word_rs_ty(sid.clone(), og_words_base_path);
    //         let structural_implementor_sid = (sublang.map)(&sid);
    //         let ht = HeapType(syn::parse_quote! { #ext_data_structure_path::Heap });
    //         let abp = AlgebraicsBasePath::new(syn::parse_quote! { #ext_data_structure_path:: });
    //         let structural_implementor =
    //             self.sort2rs_ty(structural_implementor_sid.clone(), &ht, &abp);
    //         let behavioral_implementor = sublang
    //             .tems
    //             .iter()
    //             .find(|it| it.to_structural == structural_implementor_sid)
    //             .map(|tem| self.sort2rs_ty(tem.from_extern_behavioral.clone(), &ht, &abp))
    //             .unwrap_or_else(|| structural_implementor.clone());
    //         Implementors {
    //             structural_implementor,
    //             behavioral_implementor,
    //             word,
    //         }
    //     })
    // }
    pub fn sort2rs_ty(
        &self,
        sort: SortIdOf<L>,
        ht: &HeapType,
        abp: &AlgebraicsBasePath,
    ) -> syn::Type {
        match sort {
            SortId::Algebraic(asi) => {
                let name = self.bak.algebraic_sort_name(asi);
                let ident = name.camel_ident();
                let abp = &abp.0;
                syn::parse_quote! { #abp #ident }
            }
            SortId::TyMetaFunc(MappedType { f, a }) => {
                let TyMetaFuncData {
                    imp: RustTyMap { ty_func },
                    ..
                } = L::Tmfs::ty_meta_func_data(&f);
                let args = a.iter().map(|arg| self.sort2rs_ty(arg.clone(), ht, abp));
                let ht = &ht.0;
                syn::parse_quote! { #ty_func<#ht, #( #args, )* > }
            }
        }
    }
    pub fn sort2word_rs_ty(&self, sort: SortIdOf<L>, words_path: &syn::Path) -> syn::Type {
        self.sort2rs_ty(
            sort,
            &HeapType(syn::parse_quote! { () }),
            &AlgebraicsBasePath::new(syn::parse_quote! { #words_path::sorts:: }),
        )
    }
    pub fn sort2structural_from_word_rs_ty(
        &self,
        sort: SortIdOf<L>,
        HeapType(ht): &HeapType,
        words_path: &syn::Path,
    ) -> syn::Type {
        let word_rs_ty = self.sort2word_rs_ty(sort, words_path);
        syn::parse_quote! {
            <#ht as words::InverseImplements<#words_path::L, #word_rs_ty>>::StructuralImplementor
        }
    }
    pub fn sort2externbehavioral_from_word_rs_ty(
        &self,
        sort: SortIdOf<L>,
        HeapType(ht): &HeapType,
        words_path: &syn::Path,
    ) -> syn::Type {
        let word_rs_ty = self.sort2word_rs_ty(sort, words_path);
        syn::parse_quote! {
            <#ht as words::InverseImplements<#words_path::L, #word_rs_ty>>::ExternBehavioralImplementor
        }
    }
    pub fn sort2heap_ty(
        &self,
        sort: SortIdOf<L>,
        ht: &HeapType,
        abp: &AlgebraicsBasePath,
        words_path: Option<&syn::Path>,
    ) -> Option<syn::Type> {
        match &sort {
            SortId::Algebraic(_) => None,
            SortId::TyMetaFunc(MappedType { f, a }) => {
                // let TyMetaFuncData {
                //     heapbak: RustTyMap { ty_func },
                //     ..
                // } = L::Tmfs::ty_meta_func_data(&f);
                // // let args = a.iter().map(|arg| self.sort2rs_ty(arg.clone(), ht, abp));
                // let args: Vec<_> = match words_path {
                //     Some(wp) => a
                //         .iter()
                //         .map(|arg| self.sort2tmfmapped_rs_ty(arg.clone(), ht, abp, wp))
                //         .collect(),
                //     None => a
                //         .iter()
                //         .map(|arg| self.sort2rs_ty(arg.clone(), ht, abp))
                //         .collect(),
                // };
                // let ht = &ht.0;
                match words_path {
                    Some(wp) => {
                        let ty = self.sort2externbehavioral_from_word_rs_ty(sort, ht, wp);
                        Some(syn::parse_quote! { <#ty as term::TyMetaFunc>::HeapBak })
                    }
                    None => {
                        let TyMetaFuncData {
                            heapbak: RustTyMap { ty_func },
                            ..
                        } = L::Tmfs::ty_meta_func_data(f);
                        let args = a.iter().map(|arg| self.sort2rs_ty(arg.clone(), ht, abp));
                        let ht = &ht.0;
                        Some(syn::parse_quote! { #ty_func<#ht, #( #args, )* > })
                    }
                }
            }
        }
    }
    pub fn sort2transparency(&self, sort: SortIdOf<L>) -> Transparency {
        match sort {
            SortId::Algebraic(AlgebraicSortId::Sum(_)) => Transparency::Transparent,
            SortId::Algebraic(AlgebraicSortId::Product(_)) => Transparency::Visible,
            SortId::TyMetaFunc(MappedType { f, a: _ }) => {
                L::Tmfs::ty_meta_func_data(&f).transparency
            }
        }
    }
    // pub fn tmf_sort_ids(&self) -> impl Iterator<Item = MappedTypeOf<L>> {
    //     self.bak.all_sort_ids().filter_map(|sid| match sid {
    //         SortId::Algebraic(_) => None,
    //         SortId::TyMetaFunc(mapped_type) => Some(mapped_type),
    //     })
    // }
    fn product_heap_sort_camel_idents(&self, pid: &L::ProductId) -> Vec<syn::Ident> {
        self.bak
            .product_sorts(pid.clone())
            .map(|sort| sort_ident(self, sort, |name| name.camel_str().to_string()))
            .collect()
    }
    fn product_heap_sort_snake_idents(&self, pid: &L::ProductId) -> Vec<syn::Ident> {
        self.bak
            .product_sorts(pid.clone())
            .map(|sort| sort_ident(self, sort, |name| name.snake_str().to_string()))
            .collect()
    }
    fn sum_heap_sort_camel_idents(&self, sid: &L::SumId) -> Vec<syn::Ident> {
        self.bak
            .sum_sorts(sid.clone())
            .map(|sort| sort_ident(self, sort, |name| name.camel_str().to_string()))
            .collect()
    }
    fn sum_heap_sort_snake_idents(&self, sid: &L::SumId) -> Vec<syn::Ident> {
        self.bak
            .sum_sorts(sid.clone())
            .map(|sort| sort_ident(self, sort, |name| name.snake_str().to_string()))
            .collect()
    }
}
#[derive(Clone, Debug)]
pub struct ConversionPath<SortId>(pub Vec<SortId>);
#[derive(Debug)]
pub struct TransitiveCcfConversionPaths<SortId> {
    pub multiway_rel: CcfRelation<SortId>,
    pub paths: Vec<ConversionPath<SortId>>,
}
fn sort_ident<L: LangSpec>(
    alg: &LsGen<L>,
    sort: SortIdOf<L>,
    get_ident: fn(&Identifier) -> String,
) -> proc_macro2::Ident {
    // &sortid_name(&self.bak, sort.clone()).camel.clone(),
    match &sort {
        SortId::Algebraic(asi) => syn::Ident::new(
            &get_ident(alg.bak.algebraic_sort_name(asi.clone())),
            proc_macro2::Span::call_site(),
        ),
        SortId::TyMetaFunc(MappedType { f, a }) => {
            let TyMetaFuncData { name, idby, .. } = L::Tmfs::ty_meta_func_data(f);
            syn::Ident::new(
                &match idby {
                    IdentifiedBy::Tmf => get_ident(&name),
                    IdentifiedBy::FirstTmfArg => {
                        let arg = a.first().unwrap();
                        let name = sortid_name(alg.bak, arg);
                        get_ident(&name)
                    }
                },
                proc_macro2::Span::call_site(),
            )
        }
    }
}

pub fn tmf_ccf_sortses<L: LangSpec>(mt: &MappedTypeOf<L>) -> Vec<Vec<SortIdOf<L>>> {
    let tmfd = <L::Tmfs as TyMetaFuncSpec>::ty_meta_func_data(&mt.f);
    tmfd.canonical_froms
        .iter()
        .map(|argids| argids.iter().map(|argid| mt.a[argid.0].clone()).collect())
        .collect()
}
fn sortid_name<L: LangSpec>(ls: &L, sort: &SortIdOf<L>) -> Identifier {
    match sort {
        SortId::Algebraic(asi) => ls.algebraic_sort_name(asi.clone()).clone(),
        SortId::TyMetaFunc(MappedType { f, a, .. }) => {
            let TyMetaFuncData { name, idby, .. } = L::Tmfs::ty_meta_func_data(f);
            match idby {
                IdentifiedBy::Tmf => name,
                IdentifiedBy::FirstTmfArg => sortid_name(ls, a.first().unwrap()),
            }
        }
    }
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

#[cfg(test)]
mod tests {
    use langspec::{humanreadable::LangSpecHuman, langspec::SortIdOf};

    use crate::{
        ccfs_exploded_by_unit_paths, get_direct_ccf_rels,
        unit_ccf_paths_quadratically_large_closure,
    };

    #[test]
    fn test_gdcr() {
        let ls = langspec_examples::fib();
        type L = LangSpecHuman<tymetafuncspec_core::Core>;
        let dcr = get_direct_ccf_rels(&ls);
        for rel in &dcr {
            println!("{rel:?}\n");
        }
        let non_transparent_sorts = &[
            langspec::humanreadable::SortId::<tymetafuncspec_core::Core>::Algebraic(
                langspec::langspec::AlgebraicSortId::Sum("ℕ".into()),
            ),
        ];
        let ucr = unit_ccf_paths_quadratically_large_closure::<SortIdOf<L>>(
            dcr.as_slice(),
            non_transparent_sorts,
        );
        for rel in &ucr {
            println!("{rel:?}\n");
        }
        let cebup = ccfs_exploded_by_unit_paths(dcr.as_slice(), &ucr, non_transparent_sorts);
        for rel in &cebup {
            println!("{rel:?}\n");
        }
        println!("Direct CCF relations: {}", dcr.len());
        println!("Unit CCF relations: {}", ucr.len());
        println!("Exploded CCF relations: {}", cebup.len());
        // >>> len(["f", "nat", "leftoperand", "rightoperand", "boxf", "boxplus", "plus"]) * (len(["boundednat", "f", "plus", "boxf", "boxplus", "nat", "current"]) ** 2)
        // 343
    }
}
