use crate::visitor::PatternBuilder;
use aspect::VisitationAspect;
use aspect::{AdtLikeOrNot, Adtishness, NotAdtLike};
use ccf::CanonicallyConstructibleFrom;
use names_langspec_sort::NamesLangspecSort;
use to_literal::ToLiteral as _;
use tymetafuncspec_core::{BoundedNat, IdxBox, IdxBoxHeapBak, Set, SetHeapBak};
use visit::{Visit, visiteventsink::VisitEventSink};
use words::{Implements, InverseImplements};

impl<SortId, Heap, L, LSub, MappedBNat: Copy> Visit<BoundedNat<()>, L, MappedBNat, Heap, NotAdtLike>
    for PatternBuilder<L, LSub, SortId>
where
    MappedBNat: CanonicallyConstructibleFrom<Heap, (BoundedNat<Heap>, ())>,
    // MappedBNat: Implements<Heap, LSub, VisitationAspect>,
    BoundedNat<()>: NamesLangspecSort<LSub>,
    SortId: Clone,
{
    fn visit(&mut self, heap: &Heap, t: &MappedBNat) {
        // // todo: generalize over all opaque tmfs
        let (nat, ()) = (*t).deconstruct(heap);
        self.literal::<Heap, BoundedNat<()>>(nat.to_literal())
    }
}

impl<
    SortId: Clone,
    Heap,
    L,
    LSub,
    ElemLWord,
    MappedSet: Copy,
    MappedElem: Copy,
    ElemVisitationImplementor,
    SetLWordInLSub,
> Visit<Set<(), ElemLWord>, L, MappedSet, Heap, NotAdtLike> for PatternBuilder<L, LSub, SortId>
where
    Heap: term::SuperHeap<SetHeapBak<Heap, MappedElem>>,
    Heap: InverseImplements<L, ElemLWord, VisitationAspect>,
    Heap:
        InverseImplements<L, ElemLWord, VisitationAspect, Implementor = ElemVisitationImplementor>,
    Heap: InverseImplements<
            L,
            Set<(), ElemLWord>,
            VisitationAspect,
            Implementor = Set<Heap, MappedElem>,
        >,
    MappedSet: CanonicallyConstructibleFrom<Heap, (Set<Heap, MappedElem>, ())>,
    MappedElem: CanonicallyConstructibleFrom<Heap, (ElemVisitationImplementor, ())>,
    Set<Heap, MappedElem>: Implements<Heap, L, VisitationAspect, LWord = Set<(), ElemLWord>>,
    Set<Heap, MappedElem>: Implements<Heap, LSub, VisitationAspect, LWord = SetLWordInLSub>,
    SetLWordInLSub: names_langspec_sort::NamesLangspecSort<LSub>,
    // // <MappedSet as Implements<Heap, LSub>>::LWord: NamesLangspecSort<LSub>,
    ElemLWord: Adtishness<VisitationAspect>,
    PatternBuilder<L, LSub, SortId>: Visit<
            ElemLWord,
            L,
            <Heap as InverseImplements<L, ElemLWord, VisitationAspect>>::Implementor,
            Heap,
            <ElemLWord as Adtishness<VisitationAspect>>::X,
        >,
    // Heap:
    //     InverseImplements<L, ElemLWord, VisitationAspect, Implementor = ElemVisitationImplementor>,
    // ElemVisitationImplementor: Implements<Heap, LSub, VisitationAspect, LWord = ElemLWordInLSub>,
    // ElemLWordInLSub: names_langspec_sort::NamesLangspecSort<LSub>,
{
    fn visit(&mut self, heap: &Heap, t: &MappedSet) {
        // todo: generalize over all collections
        let (set, ()) = (*t).deconstruct(heap);
        let total = set.len(heap) as u32;
        self.push(heap, &set, total);
        for (idx, elem) in set.iter(heap).enumerate() {
            <Self as VisitEventSink<Set<Heap, MappedElem>, Heap>>::proceed(self, idx as u32, total);
            let elem = elem.deconstruct(heap).0;
            self.visit(heap, &elem);
        }
        <Self as VisitEventSink<Set<Heap, MappedElem>, Heap>>::pop(self, total);
    }
}

impl<SortId, Heap, L, LSub, ElemLWord, MappedElem, MappedIdxBox: Copy>
    Visit<IdxBox<Heap, ElemLWord>, L, MappedIdxBox, Heap, NotAdtLike>
    for PatternBuilder<L, LSub, SortId>
where
    Heap: term::SuperHeap<IdxBoxHeapBak<Heap, ElemLWord>>,
    MappedIdxBox: CanonicallyConstructibleFrom<Heap, (IdxBox<Heap, ElemLWord>, ())>,
    Heap: InverseImplements<
            L,
            IdxBox<(), ElemLWord>,
            VisitationAspect,
            Implementor = Set<Heap, MappedElem>,
        >,
    PatternBuilder<L, LSub, SortId>:
        Visit<ElemLWord, L, MappedElem, Heap, <ElemLWord as Adtishness<VisitationAspect>>::X>,
    ElemLWord: aspect::Adtishness<aspect::VisitationAspect>,
{
    fn visit(&mut self, heap: &Heap, t: &MappedIdxBox) {
        // todo: generalize over all transparent tmfs
        // let (ib, ()) = (*t).deconstruct(heap);
        // let elem = ib.get(heap);
        // self.visit(heap, elem);
        todo!()
    }
}
