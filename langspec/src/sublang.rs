use functor_derive::Functor;

use crate::langspec::{AlgebraicSortId, LangSpec, Name, SortId, SortIdOf};

pub struct Sublang<'a, SortIdSelf> {
    pub name: Name,
    pub image: Vec<SortIdSelf>,
    pub ty_names: Vec<Name>,
    pub map: Box<dyn Fn(&Name) -> SortIdSelf + 'a>,
    pub tems: Vec<TmfEndoMappingNonreflexive<SortIdSelf>>,
}
#[derive(Debug, Functor)]
pub struct TmfEndoMappingNonreflexive<SortIdSelf> {
    pub from: SortIdSelf,
    pub to: SortIdSelf,
}

pub fn reflexive_sublang<L: LangSpec>(l: &L) -> Sublang<SortIdOf<L>> {
    Sublang {
        name: l.name().clone(),
        image: l.all_sort_ids().collect(),
        ty_names: l
            .products()
            .map(|p| l.product_name(p.clone()).clone())
            .chain(l.sums().map(|s| l.sum_name(s.clone()).clone()))
            .collect(),
        map: Box::new(|name| {
            l.products()
                .find(|p| l.product_name(p.clone()) == name)
                .map(|p| SortId::Algebraic(AlgebraicSortId::Product(p)))
                .or_else(|| {
                    l.sums()
                        .find(|s| l.sum_name(s.clone()) == name)
                        .map(|s| SortId::Algebraic(AlgebraicSortId::Sum(s)))
                })
                .unwrap()
        }),
        tems: {
            let mut tems = vec![];
            crate::langspec::call_on_all_tmf_monomorphizations(l, &mut |it| {
                tems.push(TmfEndoMappingNonreflexive {
                    from: SortId::TyMetaFunc(it.clone()),
                    to: SortId::TyMetaFunc(it.clone()),
                })
            });
            tems
        },
    }
}
