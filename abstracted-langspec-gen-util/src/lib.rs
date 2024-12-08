use langspec::langspec::{AlgebraicSortId, LangSpec, SortId, SortShape};

pub struct TyGenData<'a, L: LangSpec> {
    pub id: Option<AlgebraicSortId<L::ProductId, L::SumId>>,
    pub snake_ident: syn::Ident,
    pub camel_ident: syn::Ident,
    pub cmt: CanonicallyMaybeToGenData<'a>,
    pub ccf: CanonicallyConstructibleFromGenData<'a>,
}
pub struct CanonicallyMaybeToGenData<'a> {
    pub cmt_sort_camels: Box<dyn Iterator<Item = syn::Type> + 'a>,
}
pub type TyRelPath2Ty<'a> = &'a dyn Fn(syn::Type, SortShape) -> syn::Type;
pub struct CanonicallyConstructibleFromGenData<'a> {
    pub ccf_sort_tys: Box<dyn Fn(TyRelPath2Ty<'_>) -> Vec<syn::Type> + 'a>,
}
pub struct AbstractedLSGen<'a, L: LangSpec> {
    pub bak: &'a L,
}
impl<L: LangSpec> AbstractedLSGen<'_, L> {
    pub fn ty_gen_datas(&self) -> impl Iterator<Item = TyGenData<'_, L>> {
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
        .chain(self.bak.products().map(move |pid| {
            TyGenData {
                id: Some(AlgebraicSortId::Product(pid.clone())),
                snake_ident: syn::Ident::new(
                    &self.bak.product_name(pid.clone()).snake.clone(),
                    proc_macro2::Span::call_site(),
                ),
                camel_ident: syn::Ident::new(
                    &self.bak.product_name(pid.clone()).camel.clone(),
                    proc_macro2::Span::call_site(),
                ),
                cmt: CanonicallyMaybeToGenData {
                    cmt_sort_camels: Box::new(
                        self.bak
                            .product_sorts(pid.clone())
                            .map(|sort| self.sort2rs_ty_relpath(self.bak.sid_convert(sort))),
                    ),
                },
                ccf: CanonicallyConstructibleFromGenData {
                    ccf_sort_tys: Box::new(move |f| {
                        let fields_tys = self.bak.product_sorts(pid.clone()).map(|sort| {
                            f(
                                self.sort2rs_ty_relpath(self.bak.sid_convert(sort.clone())),
                                SortShape::project(self.bak, sort),
                            )
                        });
                        vec![syn::parse_quote! {
                            (#( #fields_tys, )*)
                        }]
                    }),
                },
            }
        }))
        .chain(self.bak.sums().map(move |sid| {
            TyGenData {
                id: Some(AlgebraicSortId::Sum(sid.clone())),
                snake_ident: syn::Ident::new(
                    &self.bak.sum_name(sid.clone()).snake.clone(),
                    proc_macro2::Span::call_site(),
                ),
                camel_ident: syn::Ident::new(
                    &self.bak.sum_name(sid.clone()).camel.clone(),
                    proc_macro2::Span::call_site(),
                ),
                cmt: CanonicallyMaybeToGenData {
                    cmt_sort_camels: Box::new(
                        self.bak
                            .sum_sorts(sid.clone())
                            .map(|sort| self.sort2rs_ty_relpath(self.bak.sid_convert(sort))),
                    ),
                },
                ccf: CanonicallyConstructibleFromGenData {
                    ccf_sort_tys: Box::new(move |f| {
                        self.bak
                            .sum_sorts(sid.clone())
                            .map(|sort| {
                                let sole_arg_ty = f(
                                    self.sort2rs_ty_relpath(self.bak.sid_convert(sort.clone())),
                                    SortShape::project(self.bak, sort),
                                );
                                syn::parse_quote! {
                                    ( #sole_arg_ty, )
                                }
                            })
                            .collect()
                    }),
                },
            }
        }))
    }
    pub fn sort2rs_ty_relpath(
        &self,
        sort: SortId<AlgebraicSortId<L::ProductId, L::SumId>>,
    ) -> syn::Type {
        match sort {
            SortId::NatLiteral => syn::parse_quote! { NatLit },
            SortId::Algebraic(asi) => {
                let name = self.bak.algebraic_sort_name(asi);
                let ident = syn::Ident::new(&name.camel, proc_macro2::Span::call_site());
                syn::parse_quote! { #ident }
            }
            SortId::Set(asi) => {
                let name = self.bak.algebraic_sort_name(asi);
                let ident = syn::Ident::new(&name.camel, proc_macro2::Span::call_site());
                syn::parse_quote! { SetOf<#ident> }
            }
            SortId::Sequence(asi) => {
                let name = self.bak.algebraic_sort_name(asi);
                let ident = syn::Ident::new(&name.camel, proc_macro2::Span::call_site());
                syn::parse_quote! { SeqOf<#ident> }
            }
        }
    }
}
