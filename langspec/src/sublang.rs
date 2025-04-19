use crate::{
    flat::LangSpecFlat,
    langspec::{AlgebraicSortId, LangSpec, Name, SortId, SortIdOf},
};

// pub trait Sublang<L1>
// where
//     L1: LangSpec,
// {
//     type L0: LangSpec;
//     fn map(&self, l0: &SortIdOf<Self::L0>) -> SortIdOf<L1>;
//     fn is_in_image(&self, l1: &SortIdOf<L1>) -> bool;
// }
// pub struct ReflexiveSublang;

// impl<L> Sublang<L> for ReflexiveSublang
// where
//     L: LangSpec,
// {
//     type L0 = L;
//     fn map(&self, l0: &SortIdOf<Self::L0>) -> SortIdOf<L> {
//         l0.clone()
//     }
//     fn is_in_image(&self, _l1: &SortIdOf<L>) -> bool {
//         true
//     }
// }

pub struct Sublang<'a, SortIdSelf> {
    pub name: Name,
    pub image: Vec<SortIdSelf>,
    pub map: Box<dyn Fn(&Name) -> SortIdSelf + 'a>,
    pub tems: Vec<TmfEndoMappingNonreflexive<SortIdSelf>>,
}
#[derive(Debug)]
pub struct TmfEndoMappingNonreflexive<SortIdSelf> {
    pub from: SortIdSelf,
    pub to: SortIdSelf,
}

pub fn reflexive_sublang<L: LangSpec>(l: &L) -> Sublang<SortIdOf<L>> {
    Sublang {
        name: l.name().clone(),
        image: l.all_sort_ids().collect(),
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
