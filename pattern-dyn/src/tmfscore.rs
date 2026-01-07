use crate::visitor::PatternBuilder;
use ccf::CanonicallyConstructibleFrom;
use names_langspec_sort::NamesLangspecSort;
use aspect::Visitation;
use to_literal::ToLiteral as _;
use tymetafuncspec_core::{BoundedNat, IdxBox, IdxBoxHeapBak, Set, SetHeapBak};
use visit::{Visit, visiteventsink::VisitEventSink};
use aspect::{AdtLikeOrNot, Adtishness, NotAdtLike};
use words::{Implements, InverseImplements};

impl<SortId, Heap, L, LSub, MappedBNat: Copy> Visit<BoundedNat<()>, L, MappedBNat, Heap, NotAdtLike>
    for PatternBuilder<L, LSub, SortId>
where
    MappedBNat: CanonicallyConstructibleFrom<Heap, (BoundedNat<Heap>, ())>,
    MappedBNat: Implements<Heap, LSub>,
    // BoundedNat<()>: NamesLangspecSort<LSub>,
    SortId: Clone,
{
    fn visit(&mut self, heap: &Heap, t: &MappedBNat) {
        // // todo: generalize over all opaque tmfs
        // let (nat, ()) = (*t).deconstruct(heap);
        // self.literal::<Heap, BoundedNat<()>>(nat.to_literal())
        todo!()
    }
}

impl<SortId: Clone, Heap, L, LSub, ElemLWord, MappedSet: Copy, MappedElem: Copy>
    Visit<Set<(), ElemLWord>, L, MappedSet, Heap, NotAdtLike> for PatternBuilder<L, LSub, SortId>
where
    Heap: term::SuperHeap<SetHeapBak<Heap, MappedElem>>,
    Heap: InverseImplements<L, ElemLWord>,
    // Heap: InverseImplements<LSub, ElemLWord>,
    Heap: InverseImplements<
            L,
            Set<(), ElemLWord>,
            ExternBehavioralImplementor = Set<Heap, MappedElem>,
        >,
    MappedSet: CanonicallyConstructibleFrom<Heap, (Set<Heap, MappedElem>, ())>,
    MappedElem: CanonicallyConstructibleFrom<
            Heap,
            (
                <Heap as InverseImplements<L, ElemLWord>>::StructuralImplementor,
                (),
            ),
        >,
    // MappedSet: Implements<Heap, L, LWord = Set<(), ElemLWord>>,
    // // <MappedSet as Implements<Heap, LSub>>::LWord: NamesLangspecSort<LSub>,
    ElemLWord: Adtishness<Visitation>,
    PatternBuilder<L, LSub, SortId>: Visit<
            ElemLWord,
            L,
            <Heap as InverseImplements<L, ElemLWord>>::StructuralImplementor,
            Heap,
            <ElemLWord as Adtishness<Visitation>>::X,
        >,
{
    fn visit(&mut self, heap: &Heap, t: &MappedSet) {
        // // todo: generalize over all collections
        // let (set, ()) = (*t).deconstruct(heap);
        // let total = set.len(heap) as u32;
        // self.push(heap, t, total);
        // for (idx, elem) in set.iter(heap).enumerate() {
        //     VisitEventSink::<MappedSet, Heap>::proceed(self, idx as u32, total);
        //     let elem = elem.deconstruct(heap).0;
        //     self.visit(heap, &elem);
        // }
        // VisitEventSink::<MappedSet, Heap>::pop(self, total);
        todo!()
    }
}

// impl<SortId, Heap, L, LSub, Elem, MappedIdxBox: Copy, ElemTmfMetadata>
//     Visit<TmfMetadata<IdxBox<Heap, Elem>, (ElemTmfMetadata, ())>, MappedIdxBox, Heap, L>
//     for PatternBuilder<L, LSub, SortId>
// where
//     Heap: term::SuperHeap<IdxBoxHeapBak<Heap, Elem>>,
//     MappedIdxBox: CanonicallyConstructibleFrom<Heap, (IdxBox<Heap, Elem>, ())>,
//     PatternBuilder<L, LSub, SortId>: Visit<ElemTmfMetadata, Elem, Heap, L>,
// {
//     fn visit(&mut self, heap: &Heap, t: &MappedIdxBox) {
//         // todo: generalize over all transparent tmfs
//         let (ib, ()) = (*t).deconstruct(heap);
//         let elem = ib.get(heap);
//         self.visit(heap, elem);
//     }
// }
