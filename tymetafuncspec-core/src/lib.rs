use derivative::Derivative;
use langspec::{
    langspec::{Name, ToLiteral},
    tymetafunc::{IdentifiedBy, RustTyMap, TyMetaFuncData, TyMetaFuncSpec},
};
use serde::{Deserialize, Serialize};
use term::{CanonicallyConstructibleFrom, Heaped, SuperHeap, TyFingerprint};
use term_unspecialized::{MaybeOpaqueTerm, Term, TermRoundTrip};

pub struct Core;
#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct CoreTmfId(usize);

impl ToLiteral for CoreTmfId {
    fn to_literal(&self) -> syn::Expr {
        let lit = self.0;
        syn::parse_quote!(CoreTmfId(#lit))
    }
}

macro_rules! empty_heap_bak {
    ($name:ident $(,)? $($ty_args:ident),*) => {
        #[derive(derivative::Derivative)]
        #[derivative(Default(bound=""))]
        pub struct $name<Heap: ?Sized, $($ty_args),*> {
            phantom: std::marker::PhantomData<($($ty_args,)* Heap,)>,
        }
    };
}

thread_local! {
    static CORE_BAK: once_cell::sync::Lazy<[TyMetaFuncData; 4]> =
    once_cell::sync::Lazy::new(|| {
        [TyMetaFuncData {
            name: Name {
                human: "nat-lit".into(),
                camel: "NatLit".into(),
                snake: "nat_lit".into(),
            },
            imp: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::BoundedNat),
            },
            args: vec![]
            .into_boxed_slice(),
            idby: IdentifiedBy::Tmf,
            heapbak: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::BoundedNatHeapBak),
            },
            maybe_conversions: vec![].into_boxed_slice(),
            canonical_froms: vec![].into_boxed_slice(),
        },
        TyMetaFuncData {
            name: Name {
                human: "set".into(),
                camel: "Set".into(),
                snake: "set".into(),
            },
            imp: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::Set),
            },
            args: vec![
                Name {
                    human: "elem".into(),
                    camel: "Elem".into(),
                    snake: "elem".into(),
                }
            ]
            .into_boxed_slice(),
            idby: IdentifiedBy::Tmf,
            heapbak: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::SetHeapBak),
            },
            maybe_conversions: vec![].into_boxed_slice(),
            canonical_froms: vec![].into_boxed_slice(),
        },
        TyMetaFuncData {
            name: Name {
                human: "seq".into(),
                camel: "Seq".into(),
                snake: "seq".into(),
            },
            imp: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::Seq),
            },
            args: vec![
                Name {
                    human: "elem".into(),
                    camel: "Elem".into(),
                    snake: "elem".into(),
                }
            ]
            .into_boxed_slice(),
            idby: IdentifiedBy::Tmf,
            heapbak: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::SeqHeapBak),
            },
            maybe_conversions: vec![].into_boxed_slice(),
            canonical_froms: vec![].into_boxed_slice(),
        },
        TyMetaFuncData {
            name: Name {
                human: "idx_box".into(),
                camel: "IdxBox".into(),
                snake: "idx_box".into(),
            },
            imp: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::IdxBox),
            },
            args: vec![
                Name {
                    human: "elem".into(),
                    camel: "Elem".into(),
                    snake: "elem".into(),
                }
            ]
            .into_boxed_slice(),
            idby: IdentifiedBy::FirstTmfArg,
            heapbak: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::IdxBoxHeapBak),
            },
            maybe_conversions: vec![].into_boxed_slice(),
            canonical_froms: vec![].into_boxed_slice(),
        }]
    });
}

impl TyMetaFuncSpec for Core {
    type TyMetaFuncId = CoreTmfId;
    type OpaqueTerm = usize;

    fn ty_meta_func_data(id: &Self::TyMetaFuncId) -> TyMetaFuncData {
        CORE_BAK.with(|cb| cb[id.0].clone())
    }

    fn my_type() -> syn::Type {
        syn::parse_quote!(tymetafuncspec_core::Core)
    }
}
pub struct BoundedNat<Heap> {
    heap: std::marker::PhantomData<Heap>,
    pub n: usize,
}
impl<Heap> Heaped for BoundedNat<Heap> {
    type Heap = Heap;
}
empty_heap_bak!(BoundedNatHeapBak);
impl<Heap: SuperHeap<BoundedNatHeapBak<Heap>>> TermRoundTrip<0, Core> for BoundedNat<Heap> {
    fn to_term(self, _: &mut Heap) -> Term<0, Core, Heap> {
        let fingerprint = TyFingerprint::from(stringify!(BoundedNat));
        Term::new(fingerprint, vec![MaybeOpaqueTerm::Opaque(self.n)])
    }

    fn from_term(_: &mut Heap, t: Term<0, Core, Heap>) -> Self {
        let n = match t.args[0] {
            MaybeOpaqueTerm::Opaque(n) => n,
            _ => panic!("Dynamic cast failed"),
        };
        Self {
            heap: std::marker::PhantomData,
            n,
        }
    }
}

pub struct Set<Heap, Elem> {
    heap: std::marker::PhantomData<Heap>,
    pub items: std::vec::Vec<Elem>,
}
impl<Heap, Elem: Heaped<Heap = Heap>> Heaped for Set<Heap, Elem> {
    type Heap = Heap;
}
empty_heap_bak!(SetHeapBak, Elem);
macro_rules! collection_term_round_trip_impl {
    ($collection:ty, $globally_unique_name:path) => {
        impl<Heap, Elem: TermRoundTrip<0, Core> + Heaped<Heap = Heap>> TermRoundTrip<0, Core>
            for $collection
        {
            fn to_term(self, heap: &mut Self::Heap) -> Term<0, Core, Heap> {
                let items_terms: Vec<Term<0, Core, Heap>> = self
                    .items
                    .into_iter()
                    .map(|item| item.to_term(heap))
                    .collect();
                let mut fingerprint = TyFingerprint::from(stringify!($globally_unique_name));
                for item in &items_terms {
                    fingerprint = fingerprint.combine(&item.tyfingerprint);
                }
                Term::new(
                    fingerprint,
                    items_terms.into_iter().map(MaybeOpaqueTerm::Term).collect(),
                )
            }
            fn from_term(heap: &mut Self::Heap, t: Term<0, Core, Heap>) -> Self {
                let mut items = Vec::new();
                let mut fingerprint = TyFingerprint::from(stringify!($globally_unique_name));
                for item in t.args.into_iter().filter_map(|arg| match arg {
                    MaybeOpaqueTerm::Term(t) => Some(t),
                    _ => None,
                }) {
                    fingerprint = fingerprint.combine(&item.tyfingerprint);
                    items.push(Elem::from_term(heap, item));
                }
                if fingerprint != t.tyfingerprint {
                    panic!("Dynamic cast failed");
                }
                Self {
                    items,
                    heap: std::marker::PhantomData,
                }
            }
        }
    };
}
collection_term_round_trip_impl!(Set<Heap, Elem>, tymetafuncspec_core::Set);

pub struct Seq<Heap, Elem> {
    heap: std::marker::PhantomData<Heap>,
    pub items: std::vec::Vec<Elem>,
}
impl<Heap, Elem> Heaped for Seq<Heap, Elem> {
    type Heap = Heap;
}
empty_heap_bak!(SeqHeapBak, Elem);
collection_term_round_trip_impl!(Seq<Heap, Elem>, tymetafuncspec_core::Seq);

pub struct IdxBox<Heap, Elem> {
    phantom: std::marker::PhantomData<(Heap, Elem)>,
    pub idx: u32,
}
impl<Heap: SuperHeap<IdxBoxHeapBak<Heap, Elem>>, Elem> Heaped for IdxBox<Heap, Elem> {
    type Heap = Heap;
}
impl<Heap: SuperHeap<IdxBoxHeapBak<Heap, Elem>>, Elem> CanonicallyConstructibleFrom<Elem>
    for IdxBox<Heap, Elem>
{
    fn construct(heap: &mut Self::Heap, t: Elem) -> Self {
        todo!()
    }

    fn deconstruct(self, heap: &mut Self::Heap) -> Elem {
        todo!()
    }
}
impl<
        Heap: SuperHeap<IdxBoxHeapBak<Heap, Elem>>,
        Elem: TermRoundTrip<0, Core> + Heaped<Heap = Heap>,
    > TermRoundTrip<0, Core> for IdxBox<Heap, Elem>
{
    fn to_term(self, heap: &mut Heap) -> Term<0, Core, Heap> {
        let item_term = heap
            .subheap_mut::<IdxBoxHeapBak<Heap, Elem>>()
            .elems
            .get_mut(self.idx as usize)
            .unwrap()
            .take()
            .unwrap()
            .to_term(heap);
        let mut fingerprint = TyFingerprint::from(stringify!(IdxBox));
        fingerprint = fingerprint.combine(&item_term.tyfingerprint);
        Term::new(fingerprint, vec![MaybeOpaqueTerm::Term(item_term)])
    }

    fn from_term(heap: &mut Heap, t: Term<0, Core, Heap>) -> Self {
        let item = Elem::from_term(heap, t.args[0].clone().to_term());
        let idx = heap.subheap::<IdxBoxHeapBak<Heap, Elem>>().elems.len();
        heap.subheap_mut::<IdxBoxHeapBak<Heap, Elem>>()
            .elems
            .push(Some(item));
        Self {
            phantom: std::marker::PhantomData,
            idx: idx as u32,
        }
    }
}
#[derive(Derivative)]
#[derivative(Default(bound = ""))]
pub struct IdxBoxHeapBak<Heap: ?Sized, Elem> {
    phantom: std::marker::PhantomData<(Elem, Heap)>,
    elems: std::vec::Vec<Option<Elem>>, // None is a tombstone
}
