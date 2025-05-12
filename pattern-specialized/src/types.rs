use ccf::CanonicallyConstructibleFrom;
use pmsp::AdtMetadata;

use crate::{
    Predicate,
    traits::{BoundVariable, Pattern, Subpatterns},
};

pub struct Literal<TyMetadata, T, F>(std::marker::PhantomData<(TyMetadata, T, F)>);
pub struct Variable<TyMetadata, T>(std::marker::PhantomData<(TyMetadata, T)>);
pub struct ZeroOrMore<TyMetadata, T, Subpattern>(
    std::marker::PhantomData<(TyMetadata, T, Subpattern)>,
);
pub struct Ignored<TyMetadata, T>(std::marker::PhantomData<(TyMetadata, T)>);
pub struct Composite<TyMetadata, T, Subpatterns>(
    std::marker::PhantomData<(TyMetadata, T, Subpatterns)>,
);

pub struct BoundVarBaseCase<T>(T);

impl<T: Copy, Heap> BoundVariable<Heap> for BoundVarBaseCase<T> {
    type Match = T;

    type PatternRoot = T;

    fn read(&self) -> Self::Match {
        self.0
    }

    fn write(&self, _heap: &mut Heap, t: &mut Self::PatternRoot) {
        *t = self.0
    }
}

impl<TyMetadata, Heap, L, T: Copy, F> Pattern<Heap, L> for Literal<TyMetadata, T, F>
where
    F: Predicate<Heap, T>,
{
    type Input = T;

    type Output = ();

    fn try_match(t: &Self::Input, heap: &Heap) -> Option<Self::Output> {
        if F::eval(t, heap) { Some(()) } else { None }
    }
}
impl<TyMetadata, Heap, L, T: Copy> Pattern<Heap, L> for Variable<TyMetadata, T> {
    type Input = T;

    type Output = BoundVarBaseCase<T>;

    fn try_match(t: &Self::Input, _: &Heap) -> Option<Self::Output> {
        Some(BoundVarBaseCase(*t))
    }
}
impl<TyMetadata, Heap, L, T: Copy> Pattern<Heap, L> for Ignored<TyMetadata, T> {
    type Input = T;

    type Output = ();

    fn try_match(_: &Self::Input, _: &Heap) -> Option<Self::Output> {
        Some(())
    }
}
impl<Heap, L, T: Copy, Sp> Pattern<Heap, L> for Composite<AdtMetadata, T, Sp>
where
    Sp: Subpatterns<Heap, L>,
    T: CanonicallyConstructibleFrom<Heap, Sp::AllInputs>,
{
    type Input = T;

    type Output = Sp::AllOutputs;

    fn try_match(t: &Self::Input, heap: &Heap) -> Option<Self::Output> {
        if t.deconstruct_succeeds(heap) {
            let components = t.deconstruct(heap);
            Sp::try_all_match(&components, heap)
        } else {
            None
        }
    }
}
