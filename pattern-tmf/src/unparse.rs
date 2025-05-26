use ccf::CanonicallyConstructibleFrom;
use pmsp::TmfMetadata;
use term::SuperHeap;
use unparse_adt::Unparser;
use visit::Visit;

use crate::{
    NamedPattern, NamedPatternHeapBak, OrVariable, OrVariableHeapBak, OrVariableZeroOrMore,
    OrVariableZeroOrMoreHeapBak,
};

impl<'a, Heap, L, MatchedTy, MatchedTyTmfMetadata, OvMapped: Copy>
    Visit<TmfMetadata<OrVariable<Heap, MatchedTy>, (MatchedTyTmfMetadata, ())>, OvMapped, Heap, L>
    for Unparser<'a, L>
where
    Heap: SuperHeap<OrVariableHeapBak<Heap, MatchedTy>>,
    Unparser<'a, L>: Visit<MatchedTyTmfMetadata, MatchedTy, Heap, L>,
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

impl<'a, Heap, L, MatchedTy, MatchedTyTmfMetadata, OvZomMapped: Copy>
    Visit<
        TmfMetadata<OrVariableZeroOrMore<Heap, MatchedTy>, (MatchedTyTmfMetadata, ())>,
        OvZomMapped,
        Heap,
        L,
    > for Unparser<'a, L>
where
    Heap: SuperHeap<OrVariableZeroOrMoreHeapBak<Heap, MatchedTy>>,
    Unparser<'a, L>: Visit<MatchedTyTmfMetadata, MatchedTy, Heap, L>,
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

impl<'a, Heap, L, Pattern, PatternTmfMetadata, NamedPatternMapped: Copy>
    Visit<
        TmfMetadata<NamedPattern<Heap, Pattern>, (PatternTmfMetadata, ())>,
        NamedPatternMapped,
        Heap,
        L,
    > for Unparser<'a, L>
where
    Heap: SuperHeap<NamedPatternHeapBak<Heap, Pattern>>,
    Unparser<'a, L>: Visit<PatternTmfMetadata, Pattern, Heap, L>,
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
