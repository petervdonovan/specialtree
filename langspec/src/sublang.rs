use functor_derive::Functor;

use crate::{
    flat::LangSpecFlat,
    langspec::{LangSpec, Name, SortId, SortIdOf},
};

type SublangTyMap<'a, LSub, SortIdSelf> = dyn Fn(&SortIdOf<LSub>) -> SortIdSelf + 'a;

pub struct Sublang<'a, LSub: LangSpec, SortIdSelf> {
    pub lsub: &'a LSub,
    pub map: Box<SublangTyMap<'a, LSub, SortIdSelf>>,
    pub tems: Vec<TmfEndoMapping<SortIdSelf>>,
}
#[derive(Debug, Functor, Clone)]
pub struct TmfEndoMapping<SortIdSelf> {
    // pub fromrec: SortIdSelf,
    pub from_extern_behavioral: SortIdSelf,
    pub to_structural: SortIdSelf,
}

pub fn reflexive_sublang<L: LangSpec>(l: &L) -> Sublang<L, SortIdOf<L>> {
    Sublang {
        // name: l.name().clone(),
        // image: l.all_sort_ids().collect(),
        // ty_names: l
        //     .products()
        //     .map(|p| l.product_name(p.clone()).clone())
        //     .chain(l.sums().map(|s| l.sum_name(s.clone()).clone()))
        //     .collect(),
        lsub: l,
        map: Box::new(|sid| sid.clone()),
        tems: {
            let mut tems = vec![];
            let mut tmfs = vec![];
            crate::langspec::call_on_all_tmf_monomorphizations(l, &mut |it| {
                tems.push(TmfEndoMapping {
                    from_extern_behavioral: SortId::TyMetaFunc(it.clone()),
                    // fromrec: SortId::TyMetaFunc(it.clone()),
                    to_structural: SortId::TyMetaFunc(it.clone()),
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
impl<'a, LSub: LangSpec, SortIdSelf> Sublang<'a, LSub, SortIdSelf> {
    fn push_through<L: LangSpec>(&self, l: &'a L) -> Sublang<'a, impl LangSpec, SortIdOf<L>> {
        l.sublang::<LSub>(self.lsub).unwrap()
    }
}
pub trait Sublangs<SortIdSelf> {
    fn images(&self) -> impl Iterator<Item = Vec<SortIdSelf>>;
    fn tems(&self) -> impl Iterator<Item = Vec<TmfEndoMapping<SortIdSelf>>>;
    fn names(&self) -> impl Iterator<Item = Name>;
    fn kebab(&self, prefix: &str) -> String {
        format!(
            "{prefix}-{}",
            self.names().fold(String::new(), |acc, l| {
                let kebab = l.snake.replace("_", "-");
                if acc.is_empty() {
                    kebab
                } else {
                    format!("{acc}-{kebab}")
                }
            })
        )
    }
}
pub trait SublangsList<'langs, SortIdSelf>: Sublangs<SortIdSelf> {
    type Car: LangSpec;
    type Cdr: SublangsList<'langs, SortIdSelf> + Sublangs<SortIdSelf>;
    const LENGTH: usize;
    fn car<'a>(&'a self) -> &'a Sublang<'langs, Self::Car, SortIdSelf>;
    fn cdr(&self) -> &Self::Cdr;
    fn push_through<L: LangSpec>(
        &self,
        l: &'langs L,
    ) -> impl SublangsList<'langs, SortIdOf<L>> + 'langs;
}
impl<'langs, SortIdSelf: Clone> SublangsList<'langs, SortIdSelf> for () {
    type Car = LangSpecFlat<()>;
    type Cdr = Self;
    const LENGTH: usize = 0;
    fn car<'a>(&'a self) -> &'a Sublang<'langs, Self::Car, SortIdSelf> {
        panic!("out of elements")
    }

    fn cdr(&self) -> &Self::Cdr {
        panic!("out of elements")
    }

    fn push_through<L: LangSpec>(
        &self,
        _l: &'langs L,
    ) -> impl SublangsList<'langs, SortIdOf<L>> + 'langs {
    }
}
// impl<'langs, L, SortIdSelf: Clone> SublangsList<'langs, SortIdSelf>
//     for (Sublang<'langs, L, SortIdSelf>, ())
// where
//     L: LangSpec,
// {
//     type Car = L;
//     type Cdr = Self;
//     const LENGTH: usize = 1;
//     // fn deconstruct<'a>(&'a self) -> (&'a Sublang<'langs, L, SortIdSelf>, &'a Self::Cdr) {
//     //     panic!()
//     // }
//     fn car<'a>(&'a self) -> &'a Sublang<'langs, Self::Car, SortIdSelf> {
//         &self.0
//     }

//     fn cdr(&self) -> &Self::Cdr {
//         panic!("out of elements")
//     }
// }
impl<'langs, SortIdSelf: Clone, Car, Cdr> SublangsList<'langs, SortIdSelf>
    for (Sublang<'langs, Car, SortIdSelf>, Cdr)
where
    Car: LangSpec,
    Cdr: SublangsList<'langs, SortIdSelf> + Sublangs<SortIdSelf>,
{
    type Car = Car;

    type Cdr = Cdr;

    const LENGTH: usize = Cdr::LENGTH + 1;

    fn car<'a>(&'a self) -> &'a Sublang<'langs, Self::Car, SortIdSelf> {
        &self.0
    }

    fn cdr(&self) -> &Self::Cdr {
        &self.1
    }

    fn push_through<L: LangSpec>(
        &self,
        l: &'langs L,
    ) -> impl SublangsList<'langs, SortIdOf<L>> + 'langs {
        (self.0.push_through(l), self.1.push_through(l))
    }
}
pub trait SublangsElement<SortIdSelf> {
    fn image(&self) -> Vec<SortIdSelf>;
    fn tems(&self) -> Vec<TmfEndoMapping<SortIdSelf>>;
    fn name(&self) -> Name;
    // fn push_through<'this, 'other: 'this, L: LangSpec>(
    //     &'this self,
    //     l: &'other L,
    // ) -> impl SublangsElement<SortIdOf<L>> + 'this;
}
impl<'a, LSub: LangSpec, SortIdSelf: Clone> SublangsElement<SortIdSelf>
    for Sublang<'a, LSub, SortIdSelf>
{
    fn image(&self) -> Vec<SortIdSelf> {
        self.lsub.all_sort_ids().map(|it| (self.map)(&it)).collect()
    }

    fn tems(&self) -> Vec<TmfEndoMapping<SortIdSelf>> {
        self.tems.clone()
    }

    fn name(&self) -> Name {
        self.lsub.name().clone()
    }

    // fn push_through<'this, 'other: 'this, L: LangSpec>(
    //     &'this self,
    //     l: &'other L,
    // ) -> impl SublangsElement<SortIdOf<L>> + 'this {
    //     l.sublang::<LSub>(self.lsub).unwrap()
    // }
}
impl<SortIdSelf> Sublangs<SortIdSelf> for () {
    fn images(&self) -> impl Iterator<Item = Vec<SortIdSelf>> {
        std::iter::empty()
    }
    fn tems(&self) -> impl Iterator<Item = Vec<TmfEndoMapping<SortIdSelf>>> {
        std::iter::empty()
    }

    fn names(&self) -> impl Iterator<Item = Name> {
        std::iter::empty()
    }
}
impl<SortIdSelf, Car, Cdr> Sublangs<SortIdSelf> for (Car, Cdr)
where
    Car: SublangsElement<SortIdSelf>,
    Cdr: Sublangs<SortIdSelf>,
{
    fn images(&self) -> impl Iterator<Item = Vec<SortIdSelf>> {
        std::iter::once(self.0.image()).chain(self.1.images())
    }

    fn tems(&self) -> impl Iterator<Item = Vec<TmfEndoMapping<SortIdSelf>>> {
        std::iter::once(self.0.tems()).chain(self.1.tems())
    }

    fn names(&self) -> impl Iterator<Item = Name> {
        std::iter::once(self.0.name()).chain(self.1.names())
    }
}
