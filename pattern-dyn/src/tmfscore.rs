use crate::visitor::PatternBuilder;
use ccf::CanonicallyConstructibleFrom;
use names_langspec_sort::NamesLangspecSort;
use pmsp::TmfMetadata;
use to_literal::ToLiteral as _;
use tymetafuncspec_core::{BoundedNat, IdxBox, IdxBoxHeapBak, Set, SetHeapBak};
use visit::{Visit, visiteventsink::VisitEventSink};
use words::Implements;

impl<SortId, Heap, L, LSub, MappedBNat: Copy>
    Visit<TmfMetadata<BoundedNat<Heap>, ()>, MappedBNat, Heap, L>
    for PatternBuilder<L, LSub, SortId>
where
    MappedBNat: CanonicallyConstructibleFrom<Heap, (BoundedNat<Heap>, ())>,
    MappedBNat: Implements<Heap, LSub>,
    <MappedBNat as Implements<Heap, LSub>>::LWord: NamesLangspecSort<LSub>,
    SortId: Clone,
{
    fn visit(&mut self, heap: &Heap, t: &MappedBNat) {
        // todo: generalize over all opaque tmfs
        let (nat, ()) = (*t).deconstruct(heap);
        self.literal::<Heap, MappedBNat>(nat.to_literal())
    }
}

impl<SortId: Clone, Heap, L, LSub, Elem, MappedSet: Copy, ElemTmfMetadata>
    Visit<TmfMetadata<Set<Heap, Elem>, (ElemTmfMetadata, ())>, MappedSet, Heap, L>
    for PatternBuilder<L, LSub, SortId>
where
    Heap: term::SuperHeap<SetHeapBak<Heap, Elem>>,
    MappedSet: CanonicallyConstructibleFrom<Heap, (Set<Heap, Elem>, ())>,
    MappedSet: Implements<Heap, LSub>,
    <MappedSet as Implements<Heap, LSub>>::LWord: NamesLangspecSort<LSub>,
    PatternBuilder<L, LSub, SortId>: Visit<ElemTmfMetadata, Elem, Heap, L>,
{
    fn visit(&mut self, heap: &Heap, t: &MappedSet) {
        // todo: generalize over all collections
        let (set, ()) = (*t).deconstruct(heap);
        let total = set.len(heap) as u32;
        self.push(heap, t, total);
        for (idx, elem) in set.iter(heap).enumerate() {
            VisitEventSink::<MappedSet, Heap>::proceed(self, idx as u32, total);
            self.visit(heap, elem);
        }
        VisitEventSink::<MappedSet, Heap>::pop(self, total);
    }
}

impl<SortId, Heap, L, LSub, Elem, MappedIdxBox: Copy, ElemTmfMetadata>
    Visit<TmfMetadata<IdxBox<Heap, Elem>, (ElemTmfMetadata, ())>, MappedIdxBox, Heap, L>
    for PatternBuilder<L, LSub, SortId>
where
    Heap: term::SuperHeap<IdxBoxHeapBak<Heap, Elem>>,
    MappedIdxBox: CanonicallyConstructibleFrom<Heap, (IdxBox<Heap, Elem>, ())>,
    PatternBuilder<L, LSub, SortId>: Visit<ElemTmfMetadata, Elem, Heap, L>,
{
    fn visit(&mut self, heap: &Heap, t: &MappedIdxBox) {
        // todo: generalize over all transparent tmfs
        let (ib, ()) = (*t).deconstruct(heap);
        let elem = ib.get(heap);
        self.visit(heap, elem);
    }
}
