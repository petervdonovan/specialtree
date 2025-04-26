use derivative::Derivative;
use langspec::{
    langspec::{Name, ToLiteral},
    tymetafunc::{ArgId, IdentifiedBy, RustTyMap, Transparency, TyMetaFuncData, TyMetaFuncSpec},
};

use serde::{Deserialize, Serialize};
use term::{DirectlyCanonicallyConstructibleFrom, Heaped, SuperHeap, drop::UnsafeHeapDrop};

pub struct Core;
#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialOrd, Ord)]
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

pub const NATLIT: CoreTmfId = CoreTmfId(0);
pub const SET: CoreTmfId = CoreTmfId(1);
pub const SEQ: CoreTmfId = CoreTmfId(2);
pub const IDXBOX: CoreTmfId = CoreTmfId(3);
pub const EITHER: CoreTmfId = CoreTmfId(4);
pub const MAYBE: CoreTmfId = CoreTmfId(5);
pub const PAIR: CoreTmfId = CoreTmfId(6);

thread_local! {
    static CORE_BAK: once_cell::sync::Lazy<[TyMetaFuncData; 7]> =
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
            args: Box::new([]),
            idby: IdentifiedBy::Tmf,
            transparency: Transparency::Visible,
            heapbak: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::BoundedNatHeapBak),
            },
            canonical_froms: Box::new([]),
            size_depends_on: Box::new([]),
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
            args: Box::new([
                Name {
                    human: "elem".into(),
                    camel: "Elem".into(),
                    snake: "elem".into(),
                }
            ]),
            idby: IdentifiedBy::Tmf,
            transparency: Transparency::Visible,
            heapbak: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::SetHeapBak),
            },
            canonical_froms: Box::new([]),
            size_depends_on: Box::new([]),
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
            args: Box::new([
                Name {
                    human: "elem".into(),
                    camel: "Elem".into(),
                    snake: "elem".into(),
                }
            ]),
            idby: IdentifiedBy::Tmf,
            transparency: Transparency::Visible,
            heapbak: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::SeqHeapBak),
            },
            canonical_froms: Box::new([]),
            size_depends_on: Box::new([]),
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
            transparency: Transparency::Visible,
            heapbak: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::IdxBoxHeapBak),
            },
            canonical_froms: Box::new([Box::new([ArgId(0)])]),
            size_depends_on: Box::new([]),
        },
        TyMetaFuncData {
            name: Name {
                human: "either".into(),
                camel: "Either".into(),
                snake: "either".into()
            },
            args: Box::new([
                Name {human: "l".into(), camel: "L".into(), snake: "l".into()},
                Name {human: "r".into(), camel: "R".into(), snake: "r".into()},
            ]),
            imp: RustTyMap { ty_func: syn::parse_quote!(tymetafuncspec_core::Either) },
            idby: IdentifiedBy::FirstTmfArg,
            transparency: Transparency::Transparent,
            heapbak: RustTyMap { ty_func: syn::parse_quote!(tymetafuncspec_core::EitherHeapBak) },
            canonical_froms: Box::new([Box::new([ArgId(0)]), Box::new([ArgId(1)])]),
            size_depends_on: Box::new([ArgId(0), ArgId(1)]),
        },
        TyMetaFuncData {
            name: Name {
                human: "maybe".into(),
                camel: "Maybe".into(),
                snake: "maybe".into()
            },
            args: Box::new([
                Name {human: "t".into(), camel: "T".into(), snake: "t".into()},
            ]),
            imp: RustTyMap { ty_func: syn::parse_quote!(tymetafuncspec_core::Maybe) },
            idby: IdentifiedBy::FirstTmfArg,
            transparency: Transparency::Transparent,
            heapbak: RustTyMap { ty_func: syn::parse_quote!(tymetafuncspec_core::MaybeHeapBak) },
            canonical_froms: Box::new([Box::new([ArgId(0)])]),
            size_depends_on: Box::new([ArgId(0)]),
        },
        TyMetaFuncData {
            name: Name {
                human: "pair".into(),
                camel: "Pair".into(),
                snake: "pair".into()
            },
            args: Box::new([
                Name {human: "l".into(), camel: "L".into(), snake: "l".into()},
                Name {human: "r".into(), camel: "R".into(), snake: "r".into()},
            ]),
            imp: RustTyMap { ty_func: syn::parse_quote!(tymetafuncspec_core::Pair) },
            idby: IdentifiedBy::FirstTmfArg,
            transparency: Transparency::Visible,
            heapbak: RustTyMap { ty_func: syn::parse_quote!(tymetafuncspec_core::PairHeapBak) },
            canonical_froms: Box::new([Box::new([ArgId(0)]), Box::new([ArgId(1)])]),
            size_depends_on: Box::new([ArgId(0), ArgId(1)]),
        }
        ]
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
#[derive(derivative::Derivative)]
#[derivative(Debug(bound = ""))]
#[derivative(Clone(bound = ""))]
#[derivative(Copy(bound = ""))]
pub struct BoundedNat<Heap> {
    heap: std::marker::PhantomData<Heap>,
    pub n: usize,
}
impl<Heap> Heaped for BoundedNat<Heap> {
    type Heap = Heap;
}
impl<Heap> UnsafeHeapDrop<Heap> for BoundedNat<Heap> {
    unsafe fn unsafe_heap_drop(self, _: &mut Heap) {}
}
impl<Heap> BoundedNat<Heap> {
    pub fn new(n: usize) -> Self {
        BoundedNat {
            heap: std::marker::PhantomData,
            n,
        }
    }
}
empty_heap_bak!(BoundedNatHeapBak);
#[derive(derivative::Derivative)]
#[derivative(Debug(bound = ""))]
#[derivative(Clone(bound = ""))]
#[derivative(Copy(bound = ""))]
pub struct Set<Heap, Elem> {
    heap: std::marker::PhantomData<(Heap, Elem)>,
    pub items: usize,
}
impl<Heap, Elem: Heaped<Heap = Heap>> Heaped for Set<Heap, Elem> {
    type Heap = Heap;
}
#[derive(derivative::Derivative)]
#[derivative(Default(bound = ""))]
pub struct SetHeapBak<Heap: ?Sized, Elem> {
    phantom: std::marker::PhantomData<Heap>,
    vecs: slab::Slab<std::vec::Vec<Elem>>,
}
impl<Heap: SuperHeap<SetHeapBak<Heap, Elem>>, Elem> UnsafeHeapDrop<Heap> for Set<Heap, Elem> {
    unsafe fn unsafe_heap_drop(self, _heap: &mut Heap) {
        // let mut subheap = heap.subheap_mut::<SetHeapBak<Heap, Elem>>();
        todo!()
    }
}
impl<Heap, Elem> Set<Heap, Elem>
where
    Heap: SuperHeap<SetHeapBak<Heap, Elem>>,
{
    pub fn new(heap: &mut Heap, elems: Vec<Elem>) -> Self {
        Set {
            items: heap
                .subheap_mut::<SetHeapBak<Heap, Elem>>()
                .vecs
                .insert(elems),
            heap: std::marker::PhantomData,
        }
    }
}
#[derive(Debug)]
pub struct Seq<Heap, Elem> {
    heap: std::marker::PhantomData<Heap>,
    pub items: std::vec::Vec<Elem>,
}
impl<Heap, Elem> Heaped for Seq<Heap, Elem> {
    type Heap = Heap;
}
empty_heap_bak!(SeqHeapBak, Elem);
#[derive(derivative::Derivative)]
#[derivative(Debug(bound = ""))]
#[derivative(Clone(bound = ""))]
#[derivative(Copy(bound = ""))]
pub struct IdxBox<Heap, Elem> {
    phantom: std::marker::PhantomData<(Heap, Elem)>,
    pub idx: u32,
}
impl<Heap: SuperHeap<IdxBoxHeapBak<Heap, Elem>>, Elem> Heaped for IdxBox<Heap, Elem> {
    type Heap = Heap;
}
impl<Heap: SuperHeap<IdxBoxHeapBak<Heap, Elem>>, Elem: Copy> UnsafeHeapDrop<Heap>
    for IdxBox<Heap, Elem>
{
    unsafe fn unsafe_heap_drop(self, _heap: &mut Heap) {
        // let elem = heap.subheap_mut::<IdxBoxHeapBak<Heap, Elem>>().elems[self.idx as usize];
        // drop(Owned::new(elem));
        todo!()
    }
}
impl<Heap: SuperHeap<IdxBoxHeapBak<Heap, Elem>>, Elem>
    DirectlyCanonicallyConstructibleFrom<Heap, (Elem, ())> for IdxBox<Heap, Elem>
{
    fn construct(_heap: &mut Heap, _t: (Elem, ())) -> Self {
        todo!()
    }

    fn deconstruct_succeeds(&self, _heap: &Heap) -> bool {
        todo!()
    }

    fn deconstruct(self, _heap: &Heap) -> (Elem, ()) {
        todo!()
    }
}
#[derive(Derivative)]
#[derivative(Default(bound = ""))]
pub struct IdxBoxHeapBak<Heap: ?Sized, Elem> {
    phantom: std::marker::PhantomData<(Elem, Heap)>,
    elems: slab::Slab<Elem>,
}
impl<Heap, Elem> IdxBox<Heap, Elem>
where
    Heap: SuperHeap<IdxBoxHeapBak<Heap, Elem>>,
{
    pub fn new(heap: &mut Heap, t: Elem) -> Self {
        let subheap = heap.subheap_mut::<IdxBoxHeapBak<Heap, Elem>>();
        let idx = subheap.elems.insert(t);
        IdxBox {
            phantom: std::marker::PhantomData,
            idx: idx as u32,
        }
    }
}

#[derive(derivative::Derivative)]
#[derivative(Clone(bound = "L: Clone, R: Clone"))]
#[derivative(Copy(bound = "L: Copy, R: Copy"))]
pub enum Either<Heap, L, R> {
    Left(L, std::marker::PhantomData<Heap>),
    Right(R, std::marker::PhantomData<Heap>),
}
impl<Heap, L, R> Heaped for Either<Heap, L, R> {
    type Heap = Heap;
}
pub trait DefinedInTmfsCore {}
impl<Heap, L, R> DirectlyCanonicallyConstructibleFrom<Heap, (L, ())> for Either<Heap, L, R> {
    fn construct(_: &mut Heap, t: (L, ())) -> Self {
        Either::Left(t.0, std::marker::PhantomData)
    }

    fn deconstruct_succeeds(&self, _: &Heap) -> bool {
        match self {
            Either::Left(_, _) => true,
            Either::Right(_, _) => false,
        }
    }

    fn deconstruct(self, _: &Heap) -> (L, ()) {
        match self {
            Either::Left(l, _) => (l, ()),
            Either::Right(_, _) => panic!("deconstruct failed"),
        }
    }
}
empty_heap_bak!(EitherHeapBak, L, R);

#[derive(derivative::Derivative)]
#[derivative(Clone(bound = "T: Clone"))]
#[derivative(Copy(bound = "T: Copy"))]
#[derive(Default)]
pub enum Maybe<Heap, T> {
    Just(T, std::marker::PhantomData<Heap>),
    #[default]
    Nothing,
}
impl<Heap, T> Heaped for Maybe<Heap, T> {
    type Heap = Heap;
}
empty_heap_bak!(MaybeHeapBak, T);
#[derive(derivative::Derivative)]
#[derivative(Clone(bound = "L: Clone, R: Clone"))]
#[derivative(Copy(bound = "L: Copy, R: Copy"))]
pub struct Pair<Heap, L, R> {
    pub l: L,
    pub r: R,
    heap: std::marker::PhantomData<Heap>,
}
impl<Heap, L, R> Pair<Heap, L, R> {
    pub fn new(l: L, r: R) -> Self {
        Pair {
            l,
            r,
            heap: std::marker::PhantomData,
        }
    }
}
impl<Heap, L, R> Heaped for Pair<Heap, L, R> {
    type Heap = Heap;
}
impl<Heap, L, R> DirectlyCanonicallyConstructibleFrom<Heap, (L, ())> for Pair<Heap, L, R>
where
    L: Heaped<Heap = Heap>,
    R: Heaped<Heap = Heap> + Default,
{
    fn construct(_: &mut Heap, t: (L, ())) -> Self {
        Pair {
            l: t.0,
            r: Default::default(),
            heap: std::marker::PhantomData,
        }
    }

    fn deconstruct_succeeds(&self, _: &Heap) -> bool {
        true
    }

    fn deconstruct(self, _: &Heap) -> (L, ()) {
        (self.l, ())
    }
}
empty_heap_bak!(PairHeapBak, L, R);
