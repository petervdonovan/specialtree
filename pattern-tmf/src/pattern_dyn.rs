use ccf::CanonicallyConstructibleFrom;
use pattern_dyn::visitor::PatternBuilder;
use aspect::VisitationAspect;
use visit::Visit;
use aspect::{Adtishness, NotAdtLike};
use words::InverseImplements;

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
            VisitationAspect,
            Implementor = OrVariable<Heap, MatchedTy>,
        >,
    Heap: InverseImplements<L, MatchedTyLWord, VisitationAspect>,
    MappedOrVariable: CanonicallyConstructibleFrom<Heap, (OrVariable<Heap, MatchedTy>, ())>,
    MatchedTy: CanonicallyConstructibleFrom<
            Heap,
            (
                <Heap as InverseImplements<L, MatchedTyLWord, VisitationAspect>>::Implementor,
                (),
            ),
        >,
    MatchedTyLWord: Adtishness<VisitationAspect>,
    PatternBuilder<L, LSub, SortId>: Visit<
            MatchedTyLWord,
            L,
            <Heap as InverseImplements<L, MatchedTyLWord, VisitationAspect>>::Implementor,
            Heap,
            <MatchedTyLWord as Adtishness<VisitationAspect>>::X,
        >,
    // MatchedTyLWord: names_langspec_sort::NamesLangspecSort<LSub>,
{
    fn visit(&mut self, heap: &Heap, t: &MappedOrVariable) {
        // let (ov, ()) = (*t).deconstruct(heap);
        // match ov {
        //     OrVariable::Ctor(t) => self.visit(heap, &t.deconstruct(heap).0),
        //     OrVariable::Variable { name } => {
        //         let subheap = heap.subheap::<OrVariableHeapBak<_, _>>();
        //         self.variable::<Heap, MatchedTyLWord>(
        //             subheap.names.resolve(name).unwrap().to_string(),
        //         )
        //     }
        //     OrVariable::Ignored(_) => self.ignored::<Heap, MatchedTyLWord>(),
        // }
        todo!()
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
            OrVariableZeroOrMore<(), MatchedTyLWord>,
            VisitationAspect,
            Implementor = OrVariableZeroOrMore<Heap, MatchedTy>,
        >,
    MappedOrVariableZeroOrMore:
        CanonicallyConstructibleFrom<Heap, (OrVariableZeroOrMore<Heap, MatchedTy>, ())>,
    MatchedTyLWord: Adtishness<VisitationAspect>,
    PatternBuilder<L, LSub, SortId>:
        Visit<MatchedTyLWord, L, MatchedTy, Heap, <MatchedTyLWord as Adtishness<VisitationAspect>>::X>,
    // MatchedTyLWord: words::Implements<Heap, LSub>,
    MatchedTyLWord: names_langspec_sort::NamesLangspecSort<LSub>,
{
    fn visit(&mut self, heap: &Heap, t: &MappedOrVariableZeroOrMore) {
        let (ov, ()) = (*t).deconstruct(heap);
        match ov {
            OrVariableZeroOrMore::Ctor(t) => self.visit(heap, &t),
            OrVariableZeroOrMore::Variable { name } => {
                let subheap = heap.subheap::<OrVariableZeroOrMoreHeapBak<_, _>>();
                self.variable::<Heap, MatchedTyLWord>(
                    subheap.names.resolve(name).unwrap().to_string(),
                )
            }
            OrVariableZeroOrMore::Ignored(_) => self.ignored::<Heap, MatchedTyLWord>(),
            OrVariableZeroOrMore::ZeroOrMore { name } => self.vzom::<Heap, MatchedTyLWord>({
                let subheap = heap.subheap::<OrVariableZeroOrMoreHeapBak<_, _>>();
                subheap.names.resolve(name).unwrap().to_string()
            }),
        }
        // todo!()
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
            VisitationAspect,
            Implementor = NamedPattern<Heap, Pattern>,
        >,
    Heap: InverseImplements<L, PatternLWord, VisitationAspect>,
    MappedNamedPattern: CanonicallyConstructibleFrom<Heap, (NamedPattern<Heap, Pattern>, ())>,
    Pattern: CanonicallyConstructibleFrom<
            Heap,
            (
                <Heap as InverseImplements<L, PatternLWord, VisitationAspect>>::Implementor,
                (),
            ),
        >,
    PatternLWord: Adtishness<VisitationAspect>,
    PatternBuilder<L, LSub, SortId>: Visit<
            PatternLWord,
            L,
            <Heap as InverseImplements<L, PatternLWord, VisitationAspect>>::Implementor,
            Heap,
            <PatternLWord as Adtishness<VisitationAspect>>::X,
        >,
    // PatternLWord: words::Implements<Heap, LSub>,
    // <Pattern as words::Implements<Heap, LSub>>::LWord: names_langspec_sort::NamesLangspecSort<LSub>,
{
    fn visit(&mut self, heap: &Heap, t: &MappedNamedPattern) {
        let (np, ()) = (*t).deconstruct(heap);
        let name = np.name(heap);
        self.visit(heap, &np.pattern.deconstruct(heap).0);
        self.named(name)
    }
}
