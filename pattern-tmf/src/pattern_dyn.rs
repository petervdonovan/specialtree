use ccf::CanonicallyConstructibleFrom;
use pattern_dyn::visitor::PatternBuilder;
use pmsp::TmfMetadata;
use visit::Visit;

use crate::{
    NamedPattern, NamedPatternHeapBak, OrVariable, OrVariableHeapBak, OrVariableZeroOrMore,
    OrVariableZeroOrMoreHeapBak,
};

impl<SortId, Heap, L, LSub, MatchedTy, MappedOrVariable: Copy, MatchedTyMetadata>
    Visit<
        TmfMetadata<OrVariable<Heap, MatchedTy>, (MatchedTyMetadata, ())>,
        MappedOrVariable,
        Heap,
        L,
    > for PatternBuilder<L, LSub, SortId>
where
// SortId: Clone,
// Heap: term::SuperHeap<OrVariableHeapBak<Heap, MatchedTy>>,
// MappedOrVariable: CanonicallyConstructibleFrom<Heap, (OrVariable<Heap, MatchedTy>, ())>,
// PatternBuilder<L, LSub, SortId>: Visit<MatchedTyMetadata, MatchedTy, Heap, L>,
// MatchedTy: words::Implements<Heap, LSub>,
// <MatchedTy as words::Implements<Heap, LSub>>::LWord:
//     names_langspec_sort::NamesLangspecSort<LSub>,
{
    fn visit(&mut self, heap: &Heap, t: &MappedOrVariable) {
        // let (ov, ()) = (*t).deconstruct(heap);
        // match ov {
        //     OrVariable::Ctor(t) => self.visit(heap, &t),
        //     OrVariable::Variable { name } => {
        //         let subheap = heap.subheap::<OrVariableHeapBak<_, _>>();
        //         self.variable::<Heap, MatchedTy>(subheap.names.resolve(name).unwrap().to_string())
        //     }
        //     OrVariable::Ignored(_) => self.ignored::<Heap, MatchedTy>(),
        // }
        todo!()
    }
}

impl<SortId, Heap, L, LSub, MatchedTy, MappedOrVariableZeroOrMore: Copy, MatchedTyMetadata>
    Visit<
        TmfMetadata<OrVariableZeroOrMore<Heap, MatchedTy>, (MatchedTyMetadata, ())>,
        MappedOrVariableZeroOrMore,
        Heap,
        L,
    > for PatternBuilder<L, LSub, SortId>
where
    SortId: Clone,
    Heap: term::SuperHeap<OrVariableZeroOrMoreHeapBak<Heap, MatchedTy>>,
    MappedOrVariableZeroOrMore:
        CanonicallyConstructibleFrom<Heap, (OrVariableZeroOrMore<Heap, MatchedTy>, ())>,
    PatternBuilder<L, LSub, SortId>: Visit<MatchedTyMetadata, MatchedTy, Heap, L>,
    MatchedTy: words::Implements<Heap, LSub>,
    <MatchedTy as words::Implements<Heap, LSub>>::LWord:
        names_langspec_sort::NamesLangspecSort<LSub>,
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

impl<SortId, Heap, L, LSub, Pattern, MappedNamedPattern: Copy, PatternMetadata>
    Visit<
        TmfMetadata<NamedPattern<Heap, Pattern>, (PatternMetadata, ())>,
        MappedNamedPattern,
        Heap,
        L,
    > for PatternBuilder<L, LSub, SortId>
where
    SortId: Clone,
    Heap: term::SuperHeap<NamedPatternHeapBak<Heap, Pattern>>,
    MappedNamedPattern: CanonicallyConstructibleFrom<Heap, (NamedPattern<Heap, Pattern>, ())>,
    PatternBuilder<L, LSub, SortId>: Visit<PatternMetadata, Pattern, Heap, L>,
    Pattern: words::Implements<Heap, LSub>,
    <Pattern as words::Implements<Heap, LSub>>::LWord: names_langspec_sort::NamesLangspecSort<LSub>,
{
    fn visit(&mut self, heap: &Heap, t: &MappedNamedPattern) {
        let (np, ()) = (*t).deconstruct(heap);
        self.visit(heap, &np.pattern);
        self.named(np.name(heap))
    }
}
