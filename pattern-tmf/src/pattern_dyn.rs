use ccf::{CanonicallyConstructibleFrom, VisitationInfo};
use pattern_dyn::visitor::PatternBuilder;
use visit::Visit;
use words::{InverseImplements, NotAdtLike};

use crate::{
    NamedPattern, NamedPatternHeapBak, OrVariable, OrVariableHeapBak, OrVariableZeroOrMore,
    OrVariableZeroOrMoreHeapBak,
};

impl<SortId, Heap, L, LSub, MatchedTy, MatchedTyLWord, MappedOrVariable: Copy>
    Visit<OrVariable<(), MatchedTyLWord>, L, MappedOrVariable, Heap, NotAdtLike>
    for PatternBuilder<L, LSub, SortId>
where
    SortId: Clone,
    Heap: term::SuperHeap<OrVariableHeapBak<Heap, MatchedTy>>,
    Heap: InverseImplements<
            L,
            OrVariable<(), MatchedTyLWord>,
            ExternBehavioralImplementor = OrVariable<Heap, MatchedTy>,
        >,
    MappedOrVariable: CanonicallyConstructibleFrom<Heap, (OrVariable<Heap, MatchedTy>, ())>,
    MatchedTyLWord: VisitationInfo,
    PatternBuilder<L, LSub, SortId>:
        Visit<MatchedTyLWord, L, MatchedTy, Heap, <MatchedTyLWord as VisitationInfo>::AdtLikeOrNot>,
    MatchedTy: words::Implements<Heap, LSub>,
    <MatchedTy as words::Implements<Heap, LSub>>::LWord:
        names_langspec_sort::NamesLangspecSort<LSub>,
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
        // todo!()
    }
}

impl<SortId, Heap, L, LSub, MatchedTy, MatchedTyLWord, MappedOrVariableZeroOrMore: Copy>
    Visit<OrVariableZeroOrMore<(), MatchedTyLWord>, L, MappedOrVariableZeroOrMore, Heap, NotAdtLike>
    for PatternBuilder<L, LSub, SortId>
where
    SortId: Clone,
    Heap: term::SuperHeap<OrVariableZeroOrMoreHeapBak<Heap, MatchedTy>>,
    Heap: InverseImplements<
            L,
            OrVariable<(), MatchedTyLWord>,
            ExternBehavioralImplementor = OrVariableZeroOrMore<Heap, MatchedTy>,
        >,
    MappedOrVariableZeroOrMore:
        CanonicallyConstructibleFrom<Heap, (OrVariableZeroOrMore<Heap, MatchedTy>, ())>,
    MatchedTyLWord: VisitationInfo,
    PatternBuilder<L, LSub, SortId>:
        Visit<MatchedTyLWord, L, MatchedTy, Heap, <MatchedTyLWord as VisitationInfo>::AdtLikeOrNot>,
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

impl<SortId, Heap, L, LSub, Pattern, PatternLWord, MappedNamedPattern: Copy>
    Visit<NamedPattern<Heap, PatternLWord>, L, MappedNamedPattern, Heap, NotAdtLike>
    for PatternBuilder<L, LSub, SortId>
where
    SortId: Clone,
    Heap: term::SuperHeap<NamedPatternHeapBak<Heap, Pattern>>,
    Heap: InverseImplements<
            L,
            NamedPattern<(), PatternLWord>,
            ExternBehavioralImplementor = NamedPattern<Heap, Pattern>,
        >,
    MappedNamedPattern: CanonicallyConstructibleFrom<Heap, (NamedPattern<Heap, Pattern>, ())>,
    PatternLWord: VisitationInfo,
    PatternBuilder<L, LSub, SortId>:
        Visit<PatternLWord, L, Pattern, Heap, <PatternLWord as VisitationInfo>::AdtLikeOrNot>,
    Pattern: words::Implements<Heap, LSub>,
    <Pattern as words::Implements<Heap, LSub>>::LWord: names_langspec_sort::NamesLangspecSort<LSub>,
{
    fn visit(&mut self, heap: &Heap, t: &MappedNamedPattern) {
        let (np, ()) = (*t).deconstruct(heap);
        self.visit(heap, &np.pattern);
        self.named(np.name(heap))
    }
}
