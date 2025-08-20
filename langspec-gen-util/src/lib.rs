use langspec::langspec::{MappedTypeOf, Name, call_on_all_tmf_monomorphizations};
use langspec::sublang::Sublangs;
use langspec::tymetafunc::{IdentifiedBy, RustTyMap, Transparency, TyMetaFuncSpec};
use langspec::{
    langspec::{AlgebraicSortId, LangSpec, MappedType, SortId, SortIdOf},
    tymetafunc::TyMetaFuncData,
};
use rustgen_utils::{combinations, cons_list};
use term::CcfRelation;

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
pub struct CcfPaths<SortId> {
    pub units: Vec<TransitiveUnitCcfRelation<SortId>>,
    pub non_units: Vec<TransitiveNonUnitCcfRelation<SortId>>,
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
                        snake_ident: self.bak.product_name(pid.clone()).snake(),
                        camel_ident: self.bak.product_name(pid.clone()).camel(),
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
                    snake_ident: syn::Ident::new(
                        &self.bak.sum_name(sid.clone()).snake.clone(),
                        proc_macro2::Span::call_site(),
                    ),
                    camel_ident: syn::Ident::new(
                        &self.bak.sum_name(sid.clone()).camel.clone(),
                        proc_macro2::Span::call_site(),
                    ),
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
                .map(|name| syn::Ident::new(&name.camel, proc_macro2::Span::call_site()))
                .collect();
            let ty_arg_snakes = polish_name
                .iter()
                .map(|name| syn::Ident::new(&name.snake, proc_macro2::Span::call_site()));
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
                let ident = syn::Ident::new(&name.camel, proc_macro2::Span::call_site());
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
            .map(|sort| sort_ident(self, sort, |name| name.camel.clone()))
            .collect()
    }
    fn product_heap_sort_snake_idents(&self, pid: &L::ProductId) -> Vec<syn::Ident> {
        self.bak
            .product_sorts(pid.clone())
            .map(|sort| sort_ident(self, sort, |name| name.snake.clone()))
            .collect()
    }
    fn sum_heap_sort_camel_idents(&self, sid: &L::SumId) -> Vec<syn::Ident> {
        self.bak
            .sum_sorts(sid.clone())
            .map(|sort| sort_ident(self, sort, |name| name.camel.clone()))
            .collect()
    }
    fn sum_heap_sort_snake_idents(&self, sid: &L::SumId) -> Vec<syn::Ident> {
        self.bak
            .sum_sorts(sid.clone())
            .map(|sort| sort_ident(self, sort, |name| name.snake.clone()))
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
    get_ident: fn(&Name) -> String,
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

pub fn number_range(n: usize) -> impl Iterator<Item = syn::LitInt> {
    (0..n).map(|i| syn::LitInt::new(&i.to_string(), proc_macro2::Span::call_site()))
}
pub fn cons_list_index_range(n: usize, expr: syn::Expr) -> impl Iterator<Item = syn::Expr> {
    (0..n).map(move |i| {
        let ones = (0..i).map(|_| quote::quote! {1});
        syn::parse_quote! {
            #expr #(. #ones)*.0
        }
    })
}
fn sortid_name<L: LangSpec>(ls: &L, sort: &SortIdOf<L>) -> Name {
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
fn sortid_polish_name<L: LangSpec>(ls: &L, sort: &SortIdOf<L>) -> Vec<Name> {
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

fn get_direct_ccf_rels<L>(ls: &L) -> Vec<CcfRelation<SortIdOf<L>>>
where
    L: LangSpec,
{
    let mut ret: Vec<CcfRelation<SortIdOf<L>>> = ls
        .products()
        .map(|pid| CcfRelation {
            from: ls.product_sorts(pid.clone()).collect(),
            to: SortId::Algebraic(AlgebraicSortId::Product(pid)),
        })
        .chain(ls.sums().flat_map(|sid| {
            ls.sum_sorts(sid.clone()).map(move |argsid| CcfRelation {
                from: vec![argsid.clone()],
                to: SortId::Algebraic(AlgebraicSortId::Sum(sid.clone())),
            })
        }))
        .chain(ls.products().map(|sid| CcfRelation {
            from: vec![SortId::Algebraic(AlgebraicSortId::Product(sid.clone()))],
            to: SortId::Algebraic(AlgebraicSortId::Product(sid)),
        }))
        .chain(ls.sums().map(|sid| CcfRelation {
            from: vec![SortId::Algebraic(AlgebraicSortId::Sum(sid.clone()))],
            to: SortId::Algebraic(AlgebraicSortId::Sum(sid)),
        }))
        .collect();
    call_on_all_tmf_monomorphizations(ls, &mut |mt| {
        for argids in L::Tmfs::ty_meta_func_data(&mt.f).canonical_froms {
            ret.push(CcfRelation {
                from: argids.iter().map(|argid| mt.a[argid.0].clone()).collect(),
                to: SortId::TyMetaFunc(mt.clone()),
            });
        }
        ret.push(CcfRelation {
            from: vec![SortId::TyMetaFunc(mt.clone())],
            to: SortId::TyMetaFunc(mt.clone()),
        });
    });
    ret
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TransitiveUnitCcfRelation<SortId> {
    pub to: SortId,
    pub from: SortId,
    pub intermediary: SortId,
}
impl<SortId: Ord> PartialOrd for TransitiveUnitCcfRelation<SortId> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<SortId: Ord> Ord for TransitiveUnitCcfRelation<SortId> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let cmp = self.to.cmp(&other.to);
        match cmp {
            std::cmp::Ordering::Equal => {
                let cmp = self.from.cmp(&other.from);
                if cmp == std::cmp::Ordering::Equal {
                    self.intermediary.cmp(&other.intermediary)
                } else {
                    cmp
                }
            }
            cmp => cmp,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct UnitCcfRel<SortId> {
    from: SortId,
    to: SortId,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct UcpPair<SortId> {
    from: SortId,
    to: SortId,
}
#[derive(Clone, Copy)]
struct Distance(usize);
fn unit_ccf_paths_quadratically_large_closure<
    SortId: std::fmt::Debug + Clone + Eq + std::hash::Hash,
>(
    direct_ccf_rels: &[CcfRelation<SortId>],
    non_transparent_sorts: &[SortId],
) -> Vec<TransitiveUnitCcfRelation<SortId>> {
    use std::collections::HashSet;
    let unit_ccf_rels: Vec<_> = direct_ccf_rels
        .iter()
        .filter(|rel| rel.from.len() == 1)
        .map(|rel| UnitCcfRel {
            from: rel.from.first().unwrap().clone(),
            to: rel.to.clone(),
        })
        .collect();
    let unit_ccf_tos: HashSet<_> = unit_ccf_rels.iter().map(|rel| rel.to.clone()).collect();
    unit_ccf_tos
        .iter()
        .flat_map(|to| get_tucr_for_to::<SortId>(&unit_ccf_rels, to.clone(), non_transparent_sorts))
        .collect()
}

fn unit_ccf_rels<SortId: Clone>(
    direct_ccf_rels: &[CcfRelation<SortId>],
) -> Vec<UnitCcfRel<SortId>> {
    direct_ccf_rels
        .iter()
        .filter(|rel| rel.from.len() == 1)
        .map(|rel| UnitCcfRel {
            from: rel.from.first().unwrap().clone(),
            to: rel.to.clone(),
        })
        .collect()
}

fn find_ucp<SortId: Clone + Eq + std::hash::Hash + std::fmt::Debug>(
    direct_ccf_rels: &[CcfRelation<SortId>],
    pair: UcpPair<SortId>,
) -> Option<TransitiveUnitCcfRelation<SortId>> {
    let tucr = get_tucr_for_to(&unit_ccf_rels(direct_ccf_rels), pair.to.clone(), &[]);
    tucr.into_iter()
        .find(|tucr| tucr.from == pair.from && tucr.to == pair.to)
        .clone()
}

fn get_tucr_for_to<SortId: std::fmt::Debug + Clone + Eq + std::hash::Hash>(
    unit_ccf_rels: &[UnitCcfRel<SortId>],
    to: SortId,
    non_transparent_sorts: &[SortId],
) -> Vec<TransitiveUnitCcfRelation<SortId>> {
    use std::collections::HashMap;
    use std::collections::HashSet;
    let intermediaries = unit_ccf_rels
        .iter()
        .filter(|rel| rel.to == to && rel.from != to)
        .map(|rel| rel.from.clone())
        .collect::<HashSet<_>>();
    enum Ambiguity {
        Ambiguous,
        Unambiguous,
    }
    type CostedTucrs<SortId> =
        HashMap<UcpPair<SortId>, (TransitiveUnitCcfRelation<SortId>, Distance)>;
    type CostedTucrsWithAmbiguity<SortId> =
        HashMap<UcpPair<SortId>, (TransitiveUnitCcfRelation<SortId>, Distance, Ambiguity)>;
    fn get_tucr_for_to_and_intermediary<SortId: Clone + Eq + std::hash::Hash>(
        unit_ccf_rels: &[UnitCcfRel<SortId>],
        to: SortId,
        intermediary: SortId,
        non_transparent_sorts: &[SortId],
    ) -> CostedTucrs<SortId> {
        fn find_all_reachable_from_intermediary<SortId: Clone + Eq + std::hash::Hash>(
            unit_ccf_rels: &[UnitCcfRel<SortId>],
            intermediary: SortId,
            forbidden_node: SortId,
            non_transparent_sorts: &[SortId],
        ) -> HashMap<SortId, Distance> {
            let mut reachable = HashMap::new();
            let mut distance = Distance(0);
            let mut frontier = vec![(
                intermediary.clone(),
                non_transparent_sorts.contains(&forbidden_node),
            )];
            while !frontier.is_empty() {
                distance.0 += 1;
                let mut new_frontier = vec![];
                for sid in frontier.iter() {
                    if reachable.contains_key(&sid.0) || sid.0 == forbidden_node {
                        continue;
                    }
                    reachable.insert(sid.0.clone(), distance);
                    let nt = non_transparent_sorts.contains(&sid.0);
                    if sid.1 && nt {
                        continue;
                    }
                    for ucr in unit_ccf_rels.iter() {
                        if ucr.to == sid.0 {
                            new_frontier.push((ucr.from.clone(), sid.1 || nt));
                        }
                    }
                }
                frontier = new_frontier;
            }
            reachable
        }
        let reachable = find_all_reachable_from_intermediary::<SortId>(
            unit_ccf_rels,
            intermediary.clone(),
            to.clone(),
            non_transparent_sorts,
        );
        reachable
            .into_iter()
            .map(move |(from, distance)| {
                (
                    UcpPair {
                        from: from.clone(),
                        to: to.clone(),
                    },
                    (
                        TransitiveUnitCcfRelation {
                            from: from.clone(),
                            to: to.clone(),
                            intermediary: intermediary.clone(),
                        },
                        distance,
                    ),
                )
            })
            .collect()
    }
    let mut tucrs: CostedTucrsWithAmbiguity<SortId> = HashMap::new();
    for intermediary in intermediaries.iter() {
        let tucrs_intermediary = get_tucr_for_to_and_intermediary::<SortId>(
            unit_ccf_rels,
            to.clone(),
            intermediary.clone(),
            non_transparent_sorts,
        );
        for (pair, (tucr, distance)) in tucrs_intermediary.iter() {
            if let Some(existing) = tucrs.get(pair) {
                if existing.1.0 > distance.0 {
                    tucrs.insert(
                        pair.clone(),
                        (tucr.clone(), *distance, Ambiguity::Unambiguous),
                    );
                } else if existing.1.0 == distance.0 {
                    tucrs.insert(
                        pair.clone(),
                        (tucr.clone(), *distance, Ambiguity::Ambiguous),
                    );
                }
            } else {
                tucrs.insert(
                    pair.clone(),
                    (tucr.clone(), *distance, Ambiguity::Unambiguous),
                );
            }
        }
    }
    tucrs
        .into_values()
        .filter(|it| matches!(it.2, Ambiguity::Unambiguous))
        .map(|v| v.0)
        .collect()
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransitiveNonUnitCcfRelation<SortId> {
    pub from: Vec<SortId>,
    pub to: SortId,
    pub intermediary: CcfRelation<SortId>,
}
fn ccfs_exploded_by_unit_paths<SortId: Clone + Eq>(
    direct_ccf_rels: &[CcfRelation<SortId>],
    unit_ccf_rels: &[TransitiveUnitCcfRelation<SortId>],
    non_transparent_sorts: &[SortId],
) -> Vec<TransitiveNonUnitCcfRelation<SortId>> {
    fn from_sets<SortId: Eq + Clone>(
        froms: Vec<SortId>,
        unit_ccf_rels: &[TransitiveUnitCcfRelation<SortId>],
    ) -> Vec<Vec<SortId>> {
        froms
            .iter()
            .map(|from| {
                unit_ccf_rels
                    .iter()
                    .filter_map(|it| {
                        if &it.to == from {
                            Some(it.from.clone())
                        } else {
                            None
                        }
                    })
                    .chain(std::iter::once(from.clone())) // also allow no intermediaries
                    .collect()
            })
            .collect()
    }
    let unit_ccf_rels_from_nts: Vec<_> = unit_ccf_rels
        .iter()
        .filter(|&rel| non_transparent_sorts.contains(&rel.from))
        .cloned()
        .collect();
    direct_ccf_rels
        .iter()
        .filter(|direct| direct.from.len() != 1) // we only want non-units
        .flat_map(|direct| {
            unit_ccf_rels
                .iter()
                .filter_map(|it| {
                    if it.from == direct.to && non_transparent_sorts.contains(&it.to) {
                        Some(it.to.clone())
                    } else {
                        None
                    }
                })
                .chain(std::iter::once(direct.to.clone())) // also allow no intermediaries
                .flat_map({
                    let unit_ccf_rels_from_nts = unit_ccf_rels_from_nts.clone();
                    move |to| {
                        combinations(from_sets(direct.from.clone(), &unit_ccf_rels_from_nts))
                            .filter({
                                let to = to.clone();
                                move |from| !from.contains(&to)
                            }) // we don't want cycles
                            .map(move |from| TransitiveNonUnitCcfRelation {
                                from: from.clone(),
                                to: to.clone(),
                                intermediary: direct.clone(),
                            })
                    }
                })
        })
        .collect()
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
