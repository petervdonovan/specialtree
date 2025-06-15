use crate::visitor::PatternBuilder;
use ccf::CanonicallyConstructibleFrom;
use names_langspec_sort::NamesLangspecSort;
use pmsp::Visitation;
use to_literal::ToLiteral as _;
use tymetafuncspec_core::{BoundedNat, IdxBox, IdxBoxHeapBak, Set, SetHeapBak};
use visit::{Visit, visiteventsink::VisitEventSink};
use words::{AdtLikeOrNot, Adtishness, NotAdtLike};
use words::{Implements, InverseImplements};

impl<SortId, Heap, L, LSub, MappedBNat: Copy>
    Visit<BoundedNat<()>, LSub, MappedBNat, Heap, NotAdtLike> for PatternBuilder<L, LSub, SortId>
where
    MappedBNat: CanonicallyConstructibleFrom<Heap, (BoundedNat<Heap>, ())>,
    MappedBNat: Implements<Heap, LSub>,
    <MappedBNat as Implements<Heap, LSub>>::LWord: NamesLangspecSort<LSub>,
    <MappedBNat as Implements<Heap, LSub>>::LWord: Adtishness<Visitation, X = NotAdtLike>,
    SortId: Clone,
{
    fn visit(&mut self, heap: &Heap, t: &MappedBNat) {
        // todo: generalize over all opaque tmfs
        let (nat, ()) = (*t).deconstruct(heap);
        self.literal::<Heap, MappedBNat>(nat.to_literal())
    }
}

impl<SortId: Clone, Heap, L, LSub, ElemLWord, MappedSet: Copy, MappedElem: Copy>
    Visit<Set<(), ElemLWord>, LSub, MappedSet, Heap, NotAdtLike> for PatternBuilder<L, LSub, SortId>
where
    Heap: term::SuperHeap<SetHeapBak<Heap, MappedElem>>,
    Heap: InverseImplements<LSub, ElemLWord>,
    Heap: InverseImplements<
            LSub,
            Set<(), ElemLWord>,
            ExternBehavioralImplementor = Set<Heap, MappedElem>,
        >,
    MappedSet: CanonicallyConstructibleFrom<Heap, (Set<Heap, MappedElem>, ())>,
    MappedElem: CanonicallyConstructibleFrom<
            Heap,
            (
                <Heap as InverseImplements<LSub, ElemLWord>>::StructuralImplementor,
                (),
            ),
        >,
    MappedSet: Implements<Heap, LSub, LWord = Set<(), ElemLWord>>,
    <MappedSet as Implements<Heap, LSub>>::LWord: NamesLangspecSort<LSub>,
    ElemLWord: Adtishness<Visitation>,
    PatternBuilder<L, LSub, SortId>: Visit<
            ElemLWord,
            LSub,
            <Heap as InverseImplements<LSub, ElemLWord>>::StructuralImplementor,
            Heap,
            <ElemLWord as Adtishness<Visitation>>::X,
        >,
{
    fn visit(&mut self, heap: &Heap, t: &MappedSet) {
        // todo: generalize over all collections
        let (set, ()) = (*t).deconstruct(heap);
        let total = set.len(heap) as u32;
        self.push(heap, t, total);
        for (idx, elem) in set.iter(heap).enumerate() {
            VisitEventSink::<MappedSet, Heap>::proceed(self, idx as u32, total);
            let elem = elem.deconstruct(heap).0;
            self.visit(heap, &elem);
        }
        VisitEventSink::<MappedSet, Heap>::pop(self, total);
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
