pub struct NamedDynPattern<SortId> {
    pub snake_ident: String,
    pub pattern: DynPattern<SortId>,
}

pub enum DynPattern<SortId> {
    Composite(CompositePattern<SortId>),
    Ignored(SortId),
    ZeroOrMore(Box<DynPattern<SortId>>),
    Variable(Variable<SortId>),
    Literal(LiteralPattern<SortId>),
    // Predicate(PredicatePattern<SortId>),
}

pub struct Variable<SortId> {
    pub sid: SortId,
    pub ident: String,
}

pub struct CompositePattern<SortId> {
    pub rs_ty: SortId,
    pub components: Vec<DynPattern<SortId>>,
}

pub struct LiteralPattern<SortId> {
    pub sid: SortId,
    pub equal_to: syn::Expr,
}

// pub struct PredicatePattern<SortId> {
//     pub sid: SortId,
//     pub f: syn::Expr,
//     pub subtree: Box<Pattern<SortId>>,
// }

pub trait Predicate<T, Heap> {
    fn eval(heap: &Heap, t: &T) -> bool;
}

pub struct BoundVarBaseCase<T>(T);

impl<T: Copy, Heap> compiled::BoundVariable<Heap> for BoundVarBaseCase<T> {
    type Match = T;

    type PatternRoot = T;

    fn read(&self) -> Self::Match {
        self.0
    }

    fn write(&self, _heap: &mut Heap, t: &mut Self::PatternRoot) {
        *t = self.0
    }
}

pub mod dynamic {

    use crate::DynPattern;

    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum ToDynPatternError {
        #[error("invalid sequence of pattern match components")]
        InvalidSequenceOfComponents(),
    }

    pub trait ToDynPattern<L, SortId> {
        fn pattern(&self) -> Result<DynPattern<SortId>, ToDynPatternError>;
    }
}

pub mod compiled {
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
}

pub mod pattern_types {
    use ccf::CanonicallyConstructibleFrom;
    use pmsp::AdtMetadata;

    use crate::{
        BoundVarBaseCase, Predicate,
        compiled::{Pattern, Subpatterns},
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
}
