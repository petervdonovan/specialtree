use functor_derive::Functor;

use crate::langspec::{AlgebraicSortId, LangSpec, Name, SortId, SortIdOf};

pub struct Sublang<'a, SortIdSelf> {
    pub name: Name,
    pub image: Vec<SortIdSelf>,
    pub ty_names: Vec<Name>,
    pub map: Box<dyn Fn(&Name) -> SortIdSelf + 'a>,
    pub tems: Vec<TmfEndoMapping<SortIdSelf>>,
}
#[derive(Debug, Functor)]
pub struct TmfEndoMapping<SortIdSelf> {
    pub fromrec: SortIdSelf,
    pub fromshallow: SortIdSelf,
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
            let mut tmfs = vec![];
            crate::langspec::call_on_all_tmf_monomorphizations(l, &mut |it| {
                tems.push(TmfEndoMapping {
                    fromshallow: SortId::TyMetaFunc(it.clone()),
                    fromrec: SortId::TyMetaFunc(it.clone()),
                    to: SortId::TyMetaFunc(it.clone()),
                });
                tmfs.push(it.clone());
            });
            // for mt in l.tmf_roots() {
            //     if !tmfs.contains(&mt) {
            //         let tmf = SortId::TyMetaFunc(mt);
            //         tems.push(TmfEndoMapping {
            //             fromrec: tmf.clone(),
            //             fromshallow: tmf.clone(),
            //             to: tmf,
            //         });
            //     }
            // }
            tems
        },
    }
}
