use ccf::CanonicallyConstructibleFrom;
use aspect::VisitationAspect;
// use visit::Visit;
use tymetafuncspec_core::{BoundedNat, IdxBox, IdxBoxHeapBak, Set, SetHeapBak};
use aspect::{Adtishness, NotAdtLike};
use words::InverseImplements;

use crate::{Unparser, Visit};

impl<'arena, Heap, L, MappedBNat: Copy> Visit<BoundedNat<()>, L, MappedBNat, Heap, NotAdtLike>
    for Unparser<'arena, L>
where
    MappedBNat: CanonicallyConstructibleFrom<Heap, (BoundedNat<Heap>, ())>,
{
    fn visit(&mut self, heap: &Heap, t: &MappedBNat) {
        let (nat, ()) = (*t).deconstruct(heap);
        self.unparse.dynamic_text(nat.n.to_string());
    }
}

impl<'arena, Heap, L, ElemLWord, MappedSet: Copy, MappedElem>
    Visit<Set<(), ElemLWord>, L, MappedSet, Heap, NotAdtLike> for Unparser<'arena, L>
where
    Heap: term::SuperHeap<SetHeapBak<Heap, MappedElem>>,
    ElemLWord: Adtishness<VisitationAspect>,
    Heap: InverseImplements<L, ElemLWord, VisitationAspect>,
    Heap: InverseImplements<
            L,
            Set<(), ElemLWord>,
            VisitationAspect,
            Implementor = Set<Heap, MappedElem>,
        >,
    MappedSet: CanonicallyConstructibleFrom<Heap, (Set<Heap, MappedElem>, ())>,
    MappedElem: Copy
        + CanonicallyConstructibleFrom<
            Heap,
            (
                <Heap as InverseImplements<L, ElemLWord, VisitationAspect>>::Implementor,
                (),
            ),
        >,
    Unparser<'arena, L>: Visit<
            ElemLWord,
            L,
            <Heap as InverseImplements<L, ElemLWord, VisitationAspect>>::Implementor,
            Heap,
            <ElemLWord as Adtishness<VisitationAspect>>::X,
        >,
{
    fn visit(&mut self, heap: &Heap, t: &MappedSet) {
        let (set, ()) = (*t).deconstruct(heap);
        self.unparse.static_text("{");
        for item in set.iter(heap) {
            self.visit(
                heap,
                &<MappedElem as CanonicallyConstructibleFrom<
                    Heap,
                    (
                        <Heap as InverseImplements<L, ElemLWord, VisitationAspect>>::Implementor,
                        (),
                    ),
                >>::deconstruct(*item, heap)
                .0,
            );
        }
        self.unparse.static_text("}");
    }
}

// impl<'arena, Heap, L, Elem, MappedIdxBox: Copy, ElemTmfMetadata>
//     Visit<TmfMetadata<IdxBox<Heap, Elem>, (ElemTmfMetadata, ())>, MappedIdxBox, Heap, L>
//     for Unparser<'arena, L>
// where
//     Heap: term::SuperHeap<IdxBoxHeapBak<Heap, Elem>>,
//     MappedIdxBox: CanonicallyConstructibleFrom<Heap, (IdxBox<Heap, Elem>, ())>,
//     Unparser<'arena, L>: Visit<ElemTmfMetadata, Elem, Heap, L>,
// {
//     fn visit(&mut self, heap: &Heap, t: &MappedIdxBox) {
//         let (ib, ()) = (*t).deconstruct(heap);
//         let elem = ib.get(heap);
//         self.visit(heap, elem);
//     }
// }
