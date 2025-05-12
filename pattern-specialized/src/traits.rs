use conslist::{ConsList, NonemptyConsList};

pub trait BoundVariable<Heap> {
    type Match;
    type PatternRoot;
    fn read(&self) -> Self::Match;
    fn write(&self, heap: &mut Heap, t: &mut Self::PatternRoot);
}
pub trait Pattern<Heap, L> {
    type Input: Copy;
    type Output;
    fn try_match(t: &Self::Input, heap: &Heap) -> Option<Self::Output>;
}
pub trait Subpatterns<Heap, L> {
    type AllInputs: ConsList + Copy;
    type AllOutputs;
    fn try_all_match(ts: &Self::AllInputs, heap: &Heap) -> Option<Self::AllOutputs>;
}
impl<Heap, L> Subpatterns<Heap, L> for () {
    type AllInputs = ();
    type AllOutputs = ();

    fn try_all_match(_: &Self::AllInputs, _: &Heap) -> Option<Self::AllOutputs> {
        Some(())
    }
}
impl<Sp: NonemptyConsList, Heap, L> Subpatterns<Heap, L> for Sp
where
    Sp::Car: Pattern<Heap, L>,
    Sp::Cdr: Subpatterns<Heap, L>,
    <Sp::Car as Pattern<Heap, L>>::Output:
        MergesWithOutput<<Sp::Cdr as Subpatterns<Heap, L>>::AllOutputs>,
{
    type AllInputs = (
        <Sp::Car as Pattern<Heap, L>>::Input,
        <Sp::Cdr as Subpatterns<Heap, L>>::AllInputs,
    );

    type AllOutputs = <<Sp::Car as Pattern<Heap, L>>::Output as MergesWithOutput<
        <Sp::Cdr as Subpatterns<Heap, L>>::AllOutputs,
    >>::Merged;

    fn try_all_match(ts: &Self::AllInputs, heap: &Heap) -> Option<Self::AllOutputs> {
        let (car, cdr) = ts.deconstruct();
        <Sp::Car as Pattern<_, _>>::try_match(&car, heap).and_then(|car| {
            <Sp::Cdr as Subpatterns<_, _>>::try_all_match(&cdr, heap).map(|cdr| car.merge(cdr))
        })
    }
}
pub trait MergesWithOutput<Other> {
    type Merged;
    fn merge(self, other: Other) -> Self::Merged;
}
impl<Other> MergesWithOutput<Other> for () {
    type Merged = Other;

    fn merge(self, other: Other) -> Self::Merged {
        other
    }
}
impl<This> MergesWithOutput<()> for This
where
    This: NonemptyConsList,
{
    type Merged = This;

    fn merge(self, _: ()) -> Self::Merged {
        self
    }
}
impl<This, Other> MergesWithOutput<Other> for This
where
    This: NonemptyConsList,
    Other: NonemptyConsList,
{
    type Merged = (This, (Other, ()));

    fn merge(self, other: Other) -> Self::Merged {
        (self, (other, ()))
    }
}
