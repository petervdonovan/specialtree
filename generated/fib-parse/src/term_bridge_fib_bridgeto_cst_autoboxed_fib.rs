#![feature(custom_inner_attributes)]
#![rustfmt::skip]
#![allow(warnings)]
#![allow(unknown_lints)]
use crate::term_specialized_cst_autoboxed_fib as tscaf;
/// @generated by [term_bridge_gen::generate_heap]
impl fib::term_trait_fib::Heap for tscaf::Heap {}
/// @generated by [term_bridge_gen::generate_owned_impls]
pub mod owned_impls {
    use tymetafuncspec_core::Pair;
    use tymetafuncspec_core::Maybe;
    use tymetafuncspec_core::Either;
    use std_parse_metadata::ParseMetadata;
    use std_parse_error::ParseError;
    use crate::term_specialized_cst_autoboxed_fib as tscaf;
    impl fib::term_trait_fib::owned::Plus<tscaf::Heap>
    for Either<
        tscaf::Heap,
        Pair<tscaf::Heap, tscaf::Plus, Maybe<tscaf::Heap, ParseMetadata<tscaf::Heap>>>,
        ParseError<tscaf::Heap>,
    > {}
    impl fib::term_trait_fib::owned::LeftOperand<tscaf::Heap>
    for Either<
        tscaf::Heap,
        Pair<
            tscaf::Heap,
            tscaf::LeftOperand,
            Maybe<tscaf::Heap, ParseMetadata<tscaf::Heap>>,
        >,
        ParseError<tscaf::Heap>,
    > {}
    impl fib::term_trait_fib::owned::RightOperand<tscaf::Heap>
    for Either<
        tscaf::Heap,
        Pair<
            tscaf::Heap,
            tscaf::RightOperand,
            Maybe<tscaf::Heap, ParseMetadata<tscaf::Heap>>,
        >,
        ParseError<tscaf::Heap>,
    > {}
    impl fib::term_trait_fib::owned::F<tscaf::Heap>
    for Either<
        tscaf::Heap,
        Pair<tscaf::Heap, tscaf::F, Maybe<tscaf::Heap, ParseMetadata<tscaf::Heap>>>,
        ParseError<tscaf::Heap>,
    > {}
    impl fib::term_trait_fib::owned::Sum<tscaf::Heap>
    for Either<
        tscaf::Heap,
        Pair<tscaf::Heap, tscaf::Sum, Maybe<tscaf::Heap, ParseMetadata<tscaf::Heap>>>,
        ParseError<tscaf::Heap>,
    > {}
    impl fib::term_trait_fib::owned::Nat<tscaf::Heap>
    for Either<
        tscaf::Heap,
        Pair<tscaf::Heap, tscaf::Nat, Maybe<tscaf::Heap, ParseMetadata<tscaf::Heap>>>,
        ParseError<tscaf::Heap>,
    > {}
}
/// @generated by [words::words_inverse_impls]
pub mod words_inverse_impls {
    use tymetafuncspec_core::Set;
    use tymetafuncspec_core::Pair;
    use tymetafuncspec_core::Maybe;
    use tymetafuncspec_core::Either;
    use tymetafuncspec_core::BoundedNat;
    use std_parse_metadata::ParseMetadata;
    use std_parse_error::ParseError;
    use fib::words_mod_fib::sorts::Sum;
    use fib::words_mod_fib::sorts::RightOperand;
    use fib::words_mod_fib::sorts::Plus;
    use fib::words_mod_fib::sorts::Nat;
    use fib::words_mod_fib::sorts::LeftOperand;
    use fib::words_mod_fib::sorts::F;
    use fib::words_mod_fib::L;
    use crate::term_specialized_cst_autoboxed_fib as tscaf;
    impl words::InverseImplements<L, Plus> for tscaf::Heap {
        type ExternBehavioralImplementor = Either<
            tscaf::Heap,
            Pair<
                tscaf::Heap,
                tscaf::Plus,
                Maybe<tscaf::Heap, ParseMetadata<tscaf::Heap>>,
            >,
            ParseError<tscaf::Heap>,
        >;
        type StructuralImplementor = Either<
            tscaf::Heap,
            Pair<
                tscaf::Heap,
                tscaf::Plus,
                Maybe<tscaf::Heap, ParseMetadata<tscaf::Heap>>,
            >,
            ParseError<tscaf::Heap>,
        >;
    }
    impl words::InverseImplements<L, LeftOperand> for tscaf::Heap {
        type ExternBehavioralImplementor = Either<
            tscaf::Heap,
            Pair<
                tscaf::Heap,
                tscaf::LeftOperand,
                Maybe<tscaf::Heap, ParseMetadata<tscaf::Heap>>,
            >,
            ParseError<tscaf::Heap>,
        >;
        type StructuralImplementor = Either<
            tscaf::Heap,
            Pair<
                tscaf::Heap,
                tscaf::LeftOperand,
                Maybe<tscaf::Heap, ParseMetadata<tscaf::Heap>>,
            >,
            ParseError<tscaf::Heap>,
        >;
    }
    impl words::InverseImplements<L, RightOperand> for tscaf::Heap {
        type ExternBehavioralImplementor = Either<
            tscaf::Heap,
            Pair<
                tscaf::Heap,
                tscaf::RightOperand,
                Maybe<tscaf::Heap, ParseMetadata<tscaf::Heap>>,
            >,
            ParseError<tscaf::Heap>,
        >;
        type StructuralImplementor = Either<
            tscaf::Heap,
            Pair<
                tscaf::Heap,
                tscaf::RightOperand,
                Maybe<tscaf::Heap, ParseMetadata<tscaf::Heap>>,
            >,
            ParseError<tscaf::Heap>,
        >;
    }
    impl words::InverseImplements<L, F> for tscaf::Heap {
        type ExternBehavioralImplementor = Either<
            tscaf::Heap,
            Pair<tscaf::Heap, tscaf::F, Maybe<tscaf::Heap, ParseMetadata<tscaf::Heap>>>,
            ParseError<tscaf::Heap>,
        >;
        type StructuralImplementor = Either<
            tscaf::Heap,
            Pair<tscaf::Heap, tscaf::F, Maybe<tscaf::Heap, ParseMetadata<tscaf::Heap>>>,
            ParseError<tscaf::Heap>,
        >;
    }
    impl words::InverseImplements<L, Sum> for tscaf::Heap {
        type ExternBehavioralImplementor = Either<
            tscaf::Heap,
            Pair<
                tscaf::Heap,
                tscaf::Sum,
                Maybe<tscaf::Heap, ParseMetadata<tscaf::Heap>>,
            >,
            ParseError<tscaf::Heap>,
        >;
        type StructuralImplementor = Either<
            tscaf::Heap,
            Pair<
                tscaf::Heap,
                tscaf::Sum,
                Maybe<tscaf::Heap, ParseMetadata<tscaf::Heap>>,
            >,
            ParseError<tscaf::Heap>,
        >;
    }
    impl words::InverseImplements<L, Nat> for tscaf::Heap {
        type ExternBehavioralImplementor = Either<
            tscaf::Heap,
            Pair<
                tscaf::Heap,
                tscaf::Nat,
                Maybe<tscaf::Heap, ParseMetadata<tscaf::Heap>>,
            >,
            ParseError<tscaf::Heap>,
        >;
        type StructuralImplementor = Either<
            tscaf::Heap,
            Pair<
                tscaf::Heap,
                tscaf::Nat,
                Maybe<tscaf::Heap, ParseMetadata<tscaf::Heap>>,
            >,
            ParseError<tscaf::Heap>,
        >;
    }
    impl words::InverseImplements<L, Set<(), Nat>> for tscaf::Heap {
        type ExternBehavioralImplementor = Set<
            tscaf::Heap,
            Either<
                tscaf::Heap,
                Pair<
                    tscaf::Heap,
                    tscaf::Nat,
                    Maybe<tscaf::Heap, ParseMetadata<tscaf::Heap>>,
                >,
                ParseError<tscaf::Heap>,
            >,
        >;
        type StructuralImplementor = Either<
            tscaf::Heap,
            Pair<
                tscaf::Heap,
                Set<
                    tscaf::Heap,
                    Either<
                        tscaf::Heap,
                        Pair<
                            tscaf::Heap,
                            tscaf::Nat,
                            Maybe<tscaf::Heap, ParseMetadata<tscaf::Heap>>,
                        >,
                        ParseError<tscaf::Heap>,
                    >,
                >,
                Maybe<tscaf::Heap, ParseMetadata<tscaf::Heap>>,
            >,
            ParseError<tscaf::Heap>,
        >;
    }
    impl words::InverseImplements<L, BoundedNat<()>> for tscaf::Heap {
        type ExternBehavioralImplementor = BoundedNat<tscaf::Heap>;
        type StructuralImplementor = Either<
            tscaf::Heap,
            Pair<
                tscaf::Heap,
                BoundedNat<tscaf::Heap>,
                Maybe<tscaf::Heap, ParseMetadata<tscaf::Heap>>,
            >,
            ParseError<tscaf::Heap>,
        >;
    }
}
