#![feature(custom_inner_attributes)]
#![rustfmt::skip]
#![allow(warnings)]
#![allow(unknown_lints)]
//! @generated by [term_trait_gen::generate]
use words::InverseImplements;
use tymetafuncspec_core::Set;
use tymetafuncspec_core::BoundedNat;
use term::TyMetaFunc;
use term::SuperHeap;
use crate::words_mod_fib as wmf;
use crate::words_mod_fib::sorts as wmfs;
use crate::term_trait_fib::owned as ttfo;
/// @generated by [term_trait_gen::heap_trait]
pub trait Heap: Sized + InverseImplements<
        wmf::L,
        wmfs::Plus,
        StructuralImplementor: ttfo::Plus<Self>,
    > + InverseImplements<
        wmf::L,
        wmfs::LeftOperand,
        StructuralImplementor: ttfo::LeftOperand<Self>,
    > + InverseImplements<
        wmf::L,
        wmfs::RightOperand,
        StructuralImplementor: ttfo::RightOperand<Self>,
    > + InverseImplements<
        wmf::L,
        wmfs::F,
        StructuralImplementor: ttfo::F<Self>,
    > + InverseImplements<
        wmf::L,
        wmfs::Sum,
        StructuralImplementor: ttfo::Sum<Self>,
    > + InverseImplements<
        wmf::L,
        wmfs::Nat,
        StructuralImplementor: ttfo::Nat<Self>,
    > + InverseImplements<
        wmf::L,
        Set<(), wmfs::Nat>,
        ExternBehavioralImplementor: TyMetaFunc,
    > + InverseImplements<
        wmf::L,
        BoundedNat<()>,
        ExternBehavioralImplementor: TyMetaFunc,
    > + SuperHeap<
        <<Self as InverseImplements<
            wmf::L,
            Set<(), wmfs::Nat>,
        >>::ExternBehavioralImplementor as TyMetaFunc>::HeapBak,
    > + SuperHeap<
        <<Self as InverseImplements<
            wmf::L,
            BoundedNat<()>,
        >>::ExternBehavioralImplementor as TyMetaFunc>::HeapBak,
    > {}
/// @generated by [term_trait_gen::owned::generate]
pub mod owned {
    use words::InverseImplements;
    use tymetafuncspec_core::Set;
    use tymetafuncspec_core::BoundedNat;
    use crate::words_mod_fib as wmf;
    use crate::words_mod_fib::sorts as wmfs;
    use crate::term_trait_fib as ttf;
    use ccf::CanonicallyConstructibleFrom;
    pub trait Plus<
        Heap,
    >: CanonicallyConstructibleFrom<
            Heap,
            (
                <Heap as InverseImplements<
                    wmf::L,
                    wmfs::LeftOperand,
                >>::StructuralImplementor,
                (
                    <Heap as InverseImplements<
                        wmf::L,
                        wmfs::RightOperand,
                    >>::StructuralImplementor,
                    (),
                ),
            ),
        >
    where
        Heap: crate::term_trait_fib::Heap,
    {}
    pub trait LeftOperand<
        Heap,
    >: CanonicallyConstructibleFrom<
            Heap,
            (<Heap as InverseImplements<wmf::L, wmfs::Nat>>::StructuralImplementor, ()),
        >
    where
        Heap: crate::term_trait_fib::Heap,
    {}
    pub trait RightOperand<
        Heap,
    >: CanonicallyConstructibleFrom<
            Heap,
            (<Heap as InverseImplements<wmf::L, wmfs::Nat>>::StructuralImplementor, ()),
        >
    where
        Heap: crate::term_trait_fib::Heap,
    {}
    pub trait F<
        Heap,
    >: CanonicallyConstructibleFrom<
            Heap,
            (<Heap as InverseImplements<wmf::L, wmfs::Nat>>::StructuralImplementor, ()),
        >
    where
        Heap: crate::term_trait_fib::Heap,
    {}
    pub trait Sum<
        Heap,
    >: CanonicallyConstructibleFrom<
            Heap,
            (
                <Heap as InverseImplements<
                    wmf::L,
                    Set<(), wmfs::Nat>,
                >>::StructuralImplementor,
                (),
            ),
        >
    where
        Heap: crate::term_trait_fib::Heap,
    {}
    pub trait Nat<
        Heap,
    >: CanonicallyConstructibleFrom<
            Heap,
            (
                <Heap as InverseImplements<
                    wmf::L,
                    BoundedNat<()>,
                >>::StructuralImplementor,
                (),
            ),
        > + CanonicallyConstructibleFrom<
            Heap,
            (<Heap as InverseImplements<wmf::L, wmfs::F>>::StructuralImplementor, ()),
        > + CanonicallyConstructibleFrom<
            Heap,
            (<Heap as InverseImplements<wmf::L, wmfs::Plus>>::StructuralImplementor, ()),
        > + CanonicallyConstructibleFrom<
            Heap,
            (<Heap as InverseImplements<wmf::L, wmfs::Sum>>::StructuralImplementor, ()),
        >
    where
        Heap: crate::term_trait_fib::Heap,
    {}
}
