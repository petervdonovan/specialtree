use ccf::CanonicallyConstructibleFrom;
use pmsp::Visitation;
use term::SuperHeap;
use unparse_adt::Unparser;
use visit::Visit;
use words::{Adtishness, InverseImplements, NotAdtLike};

use crate::{
    NamedPattern, NamedPatternHeapBak, OrVariable, OrVariableHeapBak, OrVariableZeroOrMore,
    OrVariableZeroOrMoreHeapBak,
};

impl<'a, Heap, L, MatchedTy, MatchedTyLWord, OvMapped: Copy>
    Visit<OrVariable<(), MatchedTyLWord>, L, OvMapped, Heap, NotAdtLike> for Unparser<'a, L>
where
    Heap: SuperHeap<OrVariableHeapBak<Heap, MatchedTy>>,
    Heap: InverseImplements<
            L,
            OrVariable<(), MatchedTyLWord>,
            ExternBehavioralImplementor = OrVariable<Heap, MatchedTy>,
        >,
    MatchedTyLWord: Adtishness<Visitation>,
    Unparser<'a, L>:
        Visit<MatchedTyLWord, L, MatchedTy, Heap, <MatchedTyLWord as Adtishness<Visitation>>::X>,
    OvMapped: CanonicallyConstructibleFrom<Heap, (OrVariable<Heap, MatchedTy>, ())>,
{
    fn visit(&mut self, heap: &Heap, ov: &OvMapped) {
        let (ov, ()) = ov.deconstruct(heap);
        match ov {
            OrVariable::Ctor(term) => self.visit(heap, &term),
            OrVariable::Variable { name } => {
                let subheap = heap.subheap::<OrVariableHeapBak<_, _>>();
                self.unparse.static_text("$");
                self.unparse
                    .dynamic_text(subheap.names.resolve(name).unwrap().to_string());
            }
            OrVariable::Ignored(_) => self.unparse.static_text("_"),
        }
    }
}

impl<'a, Heap, L, MatchedTy, MatchedTyLWord, OvZomMapped: Copy>
    Visit<OrVariableZeroOrMore<(), MatchedTyLWord>, L, OvZomMapped, Heap, NotAdtLike>
    for Unparser<'a, L>
where
    Heap: SuperHeap<OrVariableZeroOrMoreHeapBak<Heap, MatchedTy>>,
    Heap: InverseImplements<
            L,
            OrVariable<(), MatchedTyLWord>,
            ExternBehavioralImplementor = OrVariableZeroOrMore<Heap, MatchedTy>,
        >,
    MatchedTyLWord: Adtishness<Visitation>,
    Unparser<'a, L>:
        Visit<MatchedTyLWord, L, MatchedTy, Heap, <MatchedTyLWord as Adtishness<Visitation>>::X>,
    OvZomMapped: CanonicallyConstructibleFrom<Heap, (OrVariableZeroOrMore<Heap, MatchedTy>, ())>,
{
    fn visit(&mut self, heap: &Heap, ovzom: &OvZomMapped) {
        let (ov, ()) = ovzom.deconstruct(heap);
        match ov {
            OrVariableZeroOrMore::Ctor(term) => self.visit(heap, &term),
            OrVariableZeroOrMore::Variable { name } => {
                let subheap = heap.subheap::<OrVariableZeroOrMoreHeapBak<_, _>>();
                self.unparse.static_text("$");
                self.unparse
                    .dynamic_text(subheap.names.resolve(name).unwrap().to_string());
            }
            OrVariableZeroOrMore::Ignored(_) => self.unparse.static_text("_"),
            OrVariableZeroOrMore::ZeroOrMore { name } => {
                self.unparse.static_text("...");
                let subheap = heap.subheap::<OrVariableZeroOrMoreHeapBak<_, _>>();
                self.unparse
                    .dynamic_text(subheap.names.resolve(name).unwrap().to_string());
            }
        }
    }
}

impl<'a, Heap, L, Pattern, PatternLWord, NamedPatternMapped: Copy>
    Visit<NamedPattern<Heap, PatternLWord>, L, NamedPatternMapped, Heap, NotAdtLike>
    for Unparser<'a, L>
where
    Heap: SuperHeap<NamedPatternHeapBak<Heap, Pattern>>,
    Heap: InverseImplements<
            L,
            NamedPattern<(), PatternLWord>,
            ExternBehavioralImplementor = NamedPattern<Heap, Pattern>,
        >,
    PatternLWord: Adtishness<Visitation>,
    Unparser<'a, L>:
        Visit<PatternLWord, L, Pattern, Heap, <PatternLWord as Adtishness<Visitation>>::X>,
    NamedPatternMapped: CanonicallyConstructibleFrom<Heap, (NamedPattern<Heap, Pattern>, ())>,
{
    fn visit(&mut self, heap: &Heap, np: &NamedPatternMapped) {
        let (np, ()) = np.deconstruct(heap);
        self.unparse.static_text("@");
        self.unparse.dynamic_text(np.name(heap).to_string());
        self.unparse.static_text("=");
        self.visit(heap, &np.pattern);
    }
}
