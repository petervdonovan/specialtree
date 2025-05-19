use ccf::CanonicallyConstructibleFrom;
use pattern_dyn::visitor::PatternBuilder;
use pmsp::TmfMetadata;
use visit::Visit;

use crate::{OrVariable, OrVariableHeapBak, OrVariableZeroOrMore, OrVariableZeroOrMoreHeapBak};

impl<SortId, Heap, L, MatchedTy, MappedOrVariable: Copy, MatchedTyMetadata>
    Visit<
        TmfMetadata<OrVariable<Heap, MatchedTy>, (MatchedTyMetadata, ())>,
        MappedOrVariable,
        Heap,
        L,
    > for PatternBuilder<L, SortId>
where
    SortId: Clone,
    Heap: term::SuperHeap<OrVariableHeapBak<Heap, MatchedTy>>,
    MappedOrVariable: CanonicallyConstructibleFrom<Heap, (OrVariable<Heap, MatchedTy>, ())>,
    PatternBuilder<L, SortId>: Visit<MatchedTyMetadata, MatchedTy, Heap, L>,
    MatchedTy: words::Implements<Heap, L>,
    <MatchedTy as words::Implements<Heap, L>>::LWord: names_langspec_sort::NamesLangspecSort<L>,
{
    fn visit(&mut self, heap: &Heap, t: &MappedOrVariable) {
        let (ov, ()) = (*t).deconstruct(heap);
        match ov {
            OrVariable::Ctor(t) => self.visit(heap, &t),
            OrVariable::Variable { name } => {
                let subheap = heap.subheap::<OrVariableHeapBak<_, _>>();
                self.variable::<Heap, MatchedTy>(subheap.names.resolve(name).unwrap().to_string())
            }
            OrVariable::Ignored(_) => self.ignored::<Heap, MatchedTy>(),
        }
    }
}

impl<SortId, Heap, L, MatchedTy, MappedOrVariableZeroOrMore: Copy, MatchedTyMetadata>
    Visit<
        TmfMetadata<OrVariableZeroOrMore<Heap, MatchedTy>, (MatchedTyMetadata, ())>,
        MappedOrVariableZeroOrMore,
        Heap,
        L,
    > for PatternBuilder<L, SortId>
where
    SortId: Clone,
    Heap: term::SuperHeap<OrVariableZeroOrMoreHeapBak<Heap, MatchedTy>>,
    MappedOrVariableZeroOrMore:
        CanonicallyConstructibleFrom<Heap, (OrVariableZeroOrMore<Heap, MatchedTy>, ())>,
    PatternBuilder<L, SortId>: Visit<MatchedTyMetadata, MatchedTy, Heap, L>,
    MatchedTy: words::Implements<Heap, L>,
    <MatchedTy as words::Implements<Heap, L>>::LWord: names_langspec_sort::NamesLangspecSort<L>,
{
    fn visit(&mut self, heap: &Heap, t: &MappedOrVariableZeroOrMore) {
        let (ov, ()) = (*t).deconstruct(heap);
        match ov {
            OrVariableZeroOrMore::Ctor(t) => self.visit(heap, &t),
            OrVariableZeroOrMore::Variable { name } => {
                let subheap = heap.subheap::<OrVariableZeroOrMoreHeapBak<_, _>>();
                self.variable::<Heap, MatchedTy>(subheap.names.resolve(name).unwrap().to_string())
            }
            OrVariableZeroOrMore::Ignored(_) => self.ignored::<Heap, MatchedTy>(),
            OrVariableZeroOrMore::ZeroOrMore { name } => self.vzom::<Heap, MatchedTy>({
                let subheap = heap.subheap::<OrVariableZeroOrMoreHeapBak<_, _>>();
                subheap.names.resolve(name).unwrap().to_string()
            }),
        }
    }
}
