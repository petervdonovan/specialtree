use derivative::Derivative;
use functor_derive::Functor;
use langspec::tymetafunc::{
    ArgId, IdentifiedBy, RustTyMap, Transparency, TyMetaFuncData, TyMetaFuncSpec,
};
use tree_identifier::Identifier;

use aspect::VisitationAspect;
use aspect::{AdtLike, Adtishness, NotAdtLike};
use ccf::DirectlyCanonicallyConstructibleFrom;
use serde::{Deserialize, Serialize};
use term::{Heaped, SuperHeap, TyMetaFunc};
use to_literal::ToLiteral;

pub struct Core;
#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialOrd, Ord)]
pub struct CoreTmfId(usize);

#[macro_export]
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
            name: Identifier::from_camel_str("NatLit").unwrap(),
            imp: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::BoundedNat),
            },
            heapbak: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::BoundedNatHeapBak),
            },
            args: Box::new([]),
            idby: IdentifiedBy::Tmf,
            transparency: Transparency::Visible,
            canonical_froms: Box::new([]),
            size_depends_on: Box::new([]),
            is_collection_of: Box::new([])
        },
        TyMetaFuncData {
            name: Identifier::from_camel_str("Set").unwrap(),
            imp: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::Set),
            },
                        heapbak: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::SetHeapBak),
            },
            args: Box::new([
                Identifier::from_camel_str("Elem").unwrap()
            ]),
            idby: IdentifiedBy::Tmf,
            transparency: Transparency::Visible,
            canonical_froms: Box::new([]),
            size_depends_on: Box::new([]),
            is_collection_of: Box::new([ArgId(0)])
        },
        TyMetaFuncData {
            name: Identifier::from_camel_str("Seq").unwrap(),
            imp: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::Seq),
            },
                        heapbak: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::SeqHeapBak),
            },
            args: Box::new([
                Identifier::from_camel_str("Elem").unwrap()
            ]),
            idby: IdentifiedBy::Tmf,
            transparency: Transparency::Visible,
            canonical_froms: Box::new([]),
            size_depends_on: Box::new([]),
            is_collection_of: Box::new([ArgId(0)])
        },
        TyMetaFuncData {
            name: Identifier::from_camel_str("IdxBox").unwrap(),
            imp: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::IdxBox),
            },
                        heapbak: RustTyMap {
                ty_func: syn::parse_quote!(tymetafuncspec_core::IdxBoxHeapBak),
            },
            args: vec![
                Identifier::from_camel_str("Elem").unwrap()
            ]
            .into_boxed_slice(),
            idby: IdentifiedBy::FirstTmfArg,
            transparency: Transparency::Visible,
            canonical_froms: Box::new([Box::new([ArgId(0)])]),
            size_depends_on: Box::new([]),
            is_collection_of: Box::new([])
        },
        TyMetaFuncData {
            name: Identifier::from_camel_str("Either").unwrap(),
            args: Box::new([
                Identifier::from_camel_str("L").unwrap(),
                Identifier::from_camel_str("R").unwrap(),
            ]),
            imp: RustTyMap { ty_func: syn::parse_quote!(tymetafuncspec_core::Either) },
            heapbak: RustTyMap { ty_func: syn::parse_quote!(tymetafuncspec_core::EitherHeapBak) },
            idby: IdentifiedBy::FirstTmfArg,
            transparency: Transparency::Transparent,
            canonical_froms: Box::new([Box::new([ArgId(0)]), Box::new([ArgId(1)])]),
            size_depends_on: Box::new([ArgId(0), ArgId(1)]),
            is_collection_of: Box::new([])
        },
        TyMetaFuncData {
            name: Identifier::from_camel_str("Maybe").unwrap(),
            args: Box::new([
                Identifier::from_camel_str("T").unwrap(),
            ]),
            imp: RustTyMap { ty_func: syn::parse_quote!(tymetafuncspec_core::Maybe) },
            heapbak: RustTyMap { ty_func: syn::parse_quote!(tymetafuncspec_core::MaybeHeapBak) },
            idby: IdentifiedBy::FirstTmfArg,
            transparency: Transparency::Transparent,
            canonical_froms: Box::new([Box::new([ArgId(0)])]),
            size_depends_on: Box::new([ArgId(0)]),
            is_collection_of: Box::new([])
        },
        TyMetaFuncData {
            name: Identifier::from_camel_str("Pair").unwrap(),
            args: Box::new([
                Identifier::from_camel_str("L").unwrap(),
                Identifier::from_camel_str("R").unwrap(),
            ]),
            imp: RustTyMap { ty_func: syn::parse_quote!(tymetafuncspec_core::Pair) },
            heapbak: RustTyMap { ty_func: syn::parse_quote!(tymetafuncspec_core::PairHeapBak) },
            idby: IdentifiedBy::FirstTmfArg,
            transparency: Transparency::Visible,
            canonical_froms: Box::new([Box::new([ArgId(0)]), Box::new([ArgId(1)])]),
            size_depends_on: Box::new([ArgId(0), ArgId(1)]),
            is_collection_of: Box::new([])
        }
        ]
    });
}

impl TyMetaFuncSpec for Core {
    type TyMetaFuncId = CoreTmfId;

    fn ty_meta_func_data(id: &Self::TyMetaFuncId) -> TyMetaFuncData {
        CORE_BAK.with(|cb| cb[id.0].clone())
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
impl Adtishness<VisitationAspect> for BoundedNat<()> {
    type X = NotAdtLike;
}
impl<Heap> Heaped for BoundedNat<Heap> {
    type Heap = Heap;
}
// impl<Heap> UnsafeHeapDrop<Heap> for BoundedNat<Heap> {
//     unsafe fn unsafe_heap_drop(self, _: &mut Heap) {}
// }
impl<Heap> BoundedNat<Heap> {
    pub fn new(n: usize) -> Self {
        BoundedNat {
            heap: std::marker::PhantomData,
            n,
        }
    }
}
impl<Heap> ToLiteral for BoundedNat<Heap> {
    fn to_literal(&self) -> syn::Expr {
        self.n.to_literal()
    }
}
empty_heap_bak!(BoundedNatHeapBak);
impl<Heap> TyMetaFunc for BoundedNat<Heap> {
    type HeapBak = BoundedNatHeapBak<Heap>;
}
#[derive(derivative::Derivative)]
#[derivative(Debug(bound = ""))]
#[derivative(Clone(bound = ""))]
#[derivative(Copy(bound = ""))]
pub struct Set<Heap, Elem> {
    heap: std::marker::PhantomData<(Heap, Elem)>,
    items: usize,
}
impl<ElemLWord> Adtishness<VisitationAspect> for Set<(), ElemLWord> {
    type X = NotAdtLike;
}
impl<Heap, Elem: Heaped<Heap = Heap>> Heaped for Set<Heap, Elem> {
    type Heap = Heap;
}
impl<Heap, Elem> TyMetaFunc for Set<Heap, Elem> {
    type HeapBak = SetHeapBak<Heap, Elem>;
}
#[derive(derivative::Derivative)]
#[derivative(Default(bound = ""))]
pub struct SetHeapBak<Heap: ?Sized, Elem> {
    phantom: std::marker::PhantomData<Heap>,
    vecs: slab::Slab<std::vec::Vec<Elem>>,
}
// impl<Heap: SuperHeap<SetHeapBak<Heap, Elem>>, Elem> UnsafeHeapDrop<Heap> for Set<Heap, Elem> {
//     unsafe fn unsafe_heap_drop(self, _heap: &mut Heap) {
//         // let mut subheap = heap.subheap_mut::<SetHeapBak<Heap, Elem>>();
//         todo!()
//     }
// }
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
    pub fn len(&self, heap: &Heap) -> usize {
        self.items(heap).len()
    }
    pub fn iter<'a, 'b>(&'a self, heap: &'b Heap) -> impl Iterator<Item = &'b Elem>
    where
        Elem: 'b,
    {
        self.items(heap).iter()
    }
    fn items<'a, 'b>(&'a self, heap: &'b Heap) -> &'b [Elem]
    where
        Elem: 'b,
    {
        heap.subheap::<SetHeapBak<Heap, Elem>>()
            .vecs
            .get(self.items)
            .unwrap()
            .as_slice()
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
impl<Heap, Elem> TyMetaFunc for Seq<Heap, Elem> {
    type HeapBak = SeqHeapBak<Heap, Elem>;
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
impl<ElemLWord> Adtishness<VisitationAspect> for IdxBox<(), ElemLWord> {
    type X = AdtLike;
}
impl<Heap, Elem> TyMetaFunc for IdxBox<Heap, Elem> {
    type HeapBak = IdxBoxHeapBak<Heap, Elem>;
}
impl<Heap: SuperHeap<IdxBoxHeapBak<Heap, Elem>>, Elem> Heaped for IdxBox<Heap, Elem> {
    type Heap = Heap;
}
// impl<Heap: SuperHeap<IdxBoxHeapBak<Heap, Elem>>, Elem: Copy> UnsafeHeapDrop<Heap>
//     for IdxBox<Heap, Elem>
// {
//     unsafe fn unsafe_heap_drop(self, _heap: &mut Heap) {
//         // let elem = heap.subheap_mut::<IdxBoxHeapBak<Heap, Elem>>().elems[self.idx as usize];
//         // drop(Owned::new(elem));
//         todo!()
//     }
// }
impl<Heap: SuperHeap<IdxBoxHeapBak<Heap, Elem>>, Elem: Copy>
    DirectlyCanonicallyConstructibleFrom<Heap, (Elem, ())> for IdxBox<Heap, Elem>
{
    fn construct(heap: &mut Heap, t: (Elem, ())) -> Self {
        let bak = heap.subheap_mut::<IdxBoxHeapBak<Heap, Elem>>();
        let idx = bak.elems.insert(t.0);
        IdxBox {
            phantom: std::marker::PhantomData,
            idx: idx as u32,
        }
    }

    fn deconstruct_succeeds(&self, _heap: &Heap) -> bool {
        true
    }

    fn deconstruct(self, heap: &Heap) -> (Elem, ()) {
        let bak = heap.subheap::<IdxBoxHeapBak<Heap, Elem>>();
        let elem = bak.elems.get(self.idx as usize).unwrap();
        (*elem, ())
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
    pub fn get<'b>(&self, heap: &'b Heap) -> &'b Elem {
        let subheap = heap.subheap::<IdxBoxHeapBak<Heap, Elem>>();
        subheap.elems.get(self.idx as usize).unwrap()
    }
}

#[derive(derivative::Derivative)]
#[derivative(Clone(bound = "L: Clone, R: Clone"))]
#[derivative(Copy(bound = "L: Copy, R: Copy"))]
#[derive(Functor)]
#[functor(L as l, R as r)]
pub enum Either<Heap, L, R> {
    Left(L, std::marker::PhantomData<Heap>),
    Right(R, std::marker::PhantomData<Heap>),
}
impl<Heap, L, R> Either<Heap, L, R> {
    pub fn fmap_l_fnmut<F, T>(self, mut f: F) -> Either<Heap, T, R>
    where
        F: FnMut(L) -> T,
    {
        match self {
            Either::Left(l, _) => Either::Left(f(l), std::marker::PhantomData),
            Either::Right(r, _) => Either::Right(r, std::marker::PhantomData),
        }
    }
    pub fn fmap_r_fnmut<F, T>(self, mut f: F) -> Either<Heap, L, T>
    where
        F: FnMut(R) -> T,
    {
        match self {
            Either::Left(l, _) => Either::Left(l, std::marker::PhantomData),
            Either::Right(r, _) => Either::Right(f(r), std::marker::PhantomData),
        }
    }
}
impl<Heap, L, R> Heaped for Either<Heap, L, R> {
    type Heap = Heap;
}
impl<Heap, L, R> TyMetaFunc for Either<Heap, L, R> {
    type HeapBak = EitherHeapBak<Heap, L, R>;
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
impl<Heap, T> TyMetaFunc for Maybe<Heap, T> {
    type HeapBak = MaybeHeapBak<Heap, T>;
}
empty_heap_bak!(MaybeHeapBak, T);
#[derive(derivative::Derivative)]
#[derivative(Clone(bound = "L: Clone, R: Clone"))]
#[derivative(Copy(bound = "L: Copy, R: Copy"))]
#[derive(Functor)]
#[functor(L as l, R as r)]
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
    pub fn fmap_l_fnmut<F, T>(self, mut f: F) -> Pair<Heap, T, R>
    where
        F: FnMut(L) -> T,
    {
        Pair {
            l: f(self.l),
            r: self.r,
            heap: std::marker::PhantomData,
        }
    }
    pub fn fmap_r_fnmut<F, T>(self, mut f: F) -> Pair<Heap, L, T>
    where
        F: FnMut(R) -> T,
    {
        Pair {
            l: self.l,
            r: f(self.r),
            heap: std::marker::PhantomData,
        }
    }
}
impl<Heap, L, R> Heaped for Pair<Heap, L, R> {
    type Heap = Heap;
}
impl<Heap, L, R> TyMetaFunc for Pair<Heap, L, R> {
    type HeapBak = PairHeapBak<Heap, L, R>;
}
impl<Heap, L, R> DirectlyCanonicallyConstructibleFrom<Heap, (L, ())> for Pair<Heap, L, R>
where
    R: Default,
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
