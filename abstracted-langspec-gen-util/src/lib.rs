use langspec::langspec::{AlgebraicSortId, LangSpec, SortShape};
use langspec_gen_util::LangSpecGen;

pub struct TyGenData<'a, L: LangSpec> {
    pub id: Option<AlgebraicSortId<L::ProductId, L::SumId>>,
    pub snake_ident: syn::Ident,
    pub camel_ident: syn::Ident,
    pub cmt: CanonicallyMaybeToGenData<'a>,
    pub ccf: CanonicallyConstructibleFromGenData<'a>,
}
pub struct CanonicallyMaybeToGenData<'a> {
    pub cmt_sort_camels: Box<dyn Iterator<Item = syn::Ident> + 'a>,
}
pub type TyCamelIdent2Ty<'a> = &'a dyn Fn(syn::Ident, SortShape) -> syn::Type;
pub struct CanonicallyConstructibleFromGenData<'a> {
    pub ccf_sort_tys: Box<dyn Fn(TyCamelIdent2Ty<'_>) -> Vec<syn::Type> + 'a>,
}
pub struct AbstractedLSGen<'a, L: LangSpec> {
    pub bak: &'a L,
}
impl<L: LangSpec> AbstractedLSGen<'_, L> {
    pub fn ty_gen_datas(&self) -> impl Iterator<Item = TyGenData<'_, L>> {
        fn lsg<L: LangSpec>(bak: &L) -> LangSpecGen<'_, L> {
            LangSpecGen {
                bak,
                sort2rs_type: |_, _| panic!("must be type-agnostic"),
                type_base_path: syn::parse_quote!(should_not_be_used),
            }
        }
        std::iter::once(TyGenData {
            id: None,
            snake_ident: syn::Ident::new("natlit", proc_macro2::Span::call_site()),
            camel_ident: syn::Ident::new("NatLit", proc_macro2::Span::call_site()),
            cmt: CanonicallyMaybeToGenData {
                cmt_sort_camels: Box::new(std::iter::empty()),
            },
            ccf: CanonicallyConstructibleFromGenData {
                ccf_sort_tys: Box::new(|_| vec![]),
            },
        })
        .chain(self.bak.products().map(move |pid| TyGenData {
            id: Some(AlgebraicSortId::Product(pid.clone())),
            snake_ident: syn::Ident::new(
                &self.bak.product_name(pid.clone()).snake.clone(),
                proc_macro2::Span::call_site(),
            ),
            camel_ident: syn::Ident::new(
                &self.bak.product_name(pid.clone()).camel.clone(),
                proc_macro2::Span::call_site(),
            ),
            cmt:
                CanonicallyMaybeToGenData {
                    cmt_sort_camels:
                        Box::new(
                            self.bak.product_sorts(pid.clone()).map(|sort| {
                                lsg(self.bak).sort2rs_ident(self.bak.sid_convert(sort))
                            }),
                        ),
                },
            ccf: CanonicallyConstructibleFromGenData {
                ccf_sort_tys: Box::new(move |f| {
                    let fields_tys = self.bak.product_sorts(pid.clone()).map(|sort| {
                        f(
                            lsg(self.bak).sort2rs_ident(self.bak.sid_convert(sort.clone())),
                            SortShape::project(self.bak, sort),
                        )
                    });
                    vec![syn::parse_quote! {
                        (#( #fields_tys, )*)
                    }]
                }),
            },
        }))
        .chain(self.bak.sums().map(move |sid| TyGenData {
            id: Some(AlgebraicSortId::Sum(sid.clone())),
            snake_ident: syn::Ident::new(
                &self.bak.sum_name(sid.clone()).snake.clone(),
                proc_macro2::Span::call_site(),
            ),
            camel_ident: syn::Ident::new(
                &self.bak.sum_name(sid.clone()).camel.clone(),
                proc_macro2::Span::call_site(),
            ),
            cmt:
                CanonicallyMaybeToGenData {
                    cmt_sort_camels:
                        Box::new(
                            self.bak.sum_sorts(sid.clone()).map(|sort| {
                                lsg(self.bak).sort2rs_ident(self.bak.sid_convert(sort))
                            }),
                        ),
                },
            ccf: CanonicallyConstructibleFromGenData {
                ccf_sort_tys: Box::new(move |f| {
                    self.bak
                        .sum_sorts(sid.clone())
                        .map(|sort| {
                            let sole_arg_ty = f(
                                lsg(self.bak).sort2rs_ident(self.bak.sid_convert(sort.clone())),
                                SortShape::project(self.bak, sort),
                            );
                            syn::parse_quote! {
                                ( #sole_arg_ty, )
                            }
                        })
                        .collect()
                }),
            },
        }))
    }
}
