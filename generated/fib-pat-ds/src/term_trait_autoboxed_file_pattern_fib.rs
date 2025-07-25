#![feature(custom_inner_attributes)]
#![rustfmt::skip]
#![allow(warnings)]
#![allow(unknown_lints)]
//! @generated by [term_trait_gen::generate]
use words::InverseImplements;
use tymetafuncspec_core::Set;
use tymetafuncspec_core::IdxBox;
use tymetafuncspec_core::BoundedNat;
use term::TyMetaFunc;
use term::SuperHeap;
use pattern_tmf::OrVariableZeroOrMore;
use pattern_tmf::OrVariable;
use pattern_tmf::NamedPattern;
use file_tmf::File;
use crate::words_mod_autoboxed_file_pattern_fib as wmafpf;
use crate::words_mod_autoboxed_file_pattern_fib::sorts as wmafpfs;
use crate::term_trait_autoboxed_file_pattern_fib::owned as ttafpfo;
/// @generated by [term_trait_gen::heap_trait]
pub trait Heap: Sized + InverseImplements<
        wmafpf::L,
        wmafpfs::Plus,
        StructuralImplementor: ttafpfo::Plus<Self>,
    > + InverseImplements<
        wmafpf::L,
        wmafpfs::LeftOperand,
        StructuralImplementor: ttafpfo::LeftOperand<Self>,
    > + InverseImplements<
        wmafpf::L,
        wmafpfs::RightOperand,
        StructuralImplementor: ttafpfo::RightOperand<Self>,
    > + InverseImplements<
        wmafpf::L,
        wmafpfs::F,
        StructuralImplementor: ttafpfo::F<Self>,
    > + InverseImplements<
        wmafpf::L,
        wmafpfs::Sum,
        StructuralImplementor: ttafpfo::Sum<Self>,
    > + InverseImplements<
        wmafpf::L,
        wmafpfs::Nat,
        StructuralImplementor: ttafpfo::Nat<Self>,
    > + InverseImplements<
        wmafpf::L,
        wmafpfs::FileItem,
        StructuralImplementor: ttafpfo::FileItem<Self>,
    > + InverseImplements<
        wmafpf::L,
        OrVariable<(), wmafpfs::LeftOperand>,
        ExternBehavioralImplementor: TyMetaFunc,
    > + InverseImplements<
        wmafpf::L,
        OrVariable<(), wmafpfs::RightOperand>,
        ExternBehavioralImplementor: TyMetaFunc,
    > + InverseImplements<
        wmafpf::L,
        OrVariable<(), wmafpfs::Nat>,
        ExternBehavioralImplementor: TyMetaFunc,
    > + InverseImplements<
        wmafpf::L,
        OrVariable<(), Set<(), OrVariableZeroOrMore<(), wmafpfs::Nat>>>,
        ExternBehavioralImplementor: TyMetaFunc,
    > + InverseImplements<
        wmafpf::L,
        Set<(), OrVariableZeroOrMore<(), wmafpfs::Nat>>,
        ExternBehavioralImplementor: TyMetaFunc,
    > + InverseImplements<
        wmafpf::L,
        OrVariableZeroOrMore<(), wmafpfs::Nat>,
        ExternBehavioralImplementor: TyMetaFunc,
    > + InverseImplements<
        wmafpf::L,
        BoundedNat<()>,
        ExternBehavioralImplementor: TyMetaFunc,
    > + InverseImplements<
        wmafpf::L,
        IdxBox<(), wmafpfs::F>,
        ExternBehavioralImplementor: TyMetaFunc,
    > + InverseImplements<
        wmafpf::L,
        IdxBox<(), wmafpfs::Plus>,
        ExternBehavioralImplementor: TyMetaFunc,
    > + InverseImplements<
        wmafpf::L,
        NamedPattern<(), wmafpfs::Plus>,
        ExternBehavioralImplementor: TyMetaFunc,
    > + InverseImplements<
        wmafpf::L,
        NamedPattern<(), wmafpfs::LeftOperand>,
        ExternBehavioralImplementor: TyMetaFunc,
    > + InverseImplements<
        wmafpf::L,
        NamedPattern<(), wmafpfs::RightOperand>,
        ExternBehavioralImplementor: TyMetaFunc,
    > + InverseImplements<
        wmafpf::L,
        NamedPattern<(), wmafpfs::F>,
        ExternBehavioralImplementor: TyMetaFunc,
    > + InverseImplements<
        wmafpf::L,
        NamedPattern<(), wmafpfs::Sum>,
        ExternBehavioralImplementor: TyMetaFunc,
    > + InverseImplements<
        wmafpf::L,
        File<(), wmafpfs::FileItem>,
        ExternBehavioralImplementor: TyMetaFunc,
    > + SuperHeap<
        <<Self as InverseImplements<
            wmafpf::L,
            OrVariable<(), wmafpfs::LeftOperand>,
        >>::ExternBehavioralImplementor as TyMetaFunc>::HeapBak,
    > + SuperHeap<
        <<Self as InverseImplements<
            wmafpf::L,
            OrVariable<(), wmafpfs::RightOperand>,
        >>::ExternBehavioralImplementor as TyMetaFunc>::HeapBak,
    > + SuperHeap<
        <<Self as InverseImplements<
            wmafpf::L,
            OrVariable<(), wmafpfs::Nat>,
        >>::ExternBehavioralImplementor as TyMetaFunc>::HeapBak,
    > + SuperHeap<
        <<Self as InverseImplements<
            wmafpf::L,
            OrVariable<(), Set<(), OrVariableZeroOrMore<(), wmafpfs::Nat>>>,
        >>::ExternBehavioralImplementor as TyMetaFunc>::HeapBak,
    > + SuperHeap<
        <<Self as InverseImplements<
            wmafpf::L,
            Set<(), OrVariableZeroOrMore<(), wmafpfs::Nat>>,
        >>::ExternBehavioralImplementor as TyMetaFunc>::HeapBak,
    > + SuperHeap<
        <<Self as InverseImplements<
            wmafpf::L,
            OrVariableZeroOrMore<(), wmafpfs::Nat>,
        >>::ExternBehavioralImplementor as TyMetaFunc>::HeapBak,
    > + SuperHeap<
        <<Self as InverseImplements<
            wmafpf::L,
            BoundedNat<()>,
        >>::ExternBehavioralImplementor as TyMetaFunc>::HeapBak,
    > + SuperHeap<
        <<Self as InverseImplements<
            wmafpf::L,
            IdxBox<(), wmafpfs::F>,
        >>::ExternBehavioralImplementor as TyMetaFunc>::HeapBak,
    > + SuperHeap<
        <<Self as InverseImplements<
            wmafpf::L,
            IdxBox<(), wmafpfs::Plus>,
        >>::ExternBehavioralImplementor as TyMetaFunc>::HeapBak,
    > + SuperHeap<
        <<Self as InverseImplements<
            wmafpf::L,
            NamedPattern<(), wmafpfs::Plus>,
        >>::ExternBehavioralImplementor as TyMetaFunc>::HeapBak,
    > + SuperHeap<
        <<Self as InverseImplements<
            wmafpf::L,
            NamedPattern<(), wmafpfs::LeftOperand>,
        >>::ExternBehavioralImplementor as TyMetaFunc>::HeapBak,
    > + SuperHeap<
        <<Self as InverseImplements<
            wmafpf::L,
            NamedPattern<(), wmafpfs::RightOperand>,
        >>::ExternBehavioralImplementor as TyMetaFunc>::HeapBak,
    > + SuperHeap<
        <<Self as InverseImplements<
            wmafpf::L,
            NamedPattern<(), wmafpfs::F>,
        >>::ExternBehavioralImplementor as TyMetaFunc>::HeapBak,
    > + SuperHeap<
        <<Self as InverseImplements<
            wmafpf::L,
            NamedPattern<(), wmafpfs::Sum>,
        >>::ExternBehavioralImplementor as TyMetaFunc>::HeapBak,
    > + SuperHeap<
        <<Self as InverseImplements<
            wmafpf::L,
            File<(), wmafpfs::FileItem>,
        >>::ExternBehavioralImplementor as TyMetaFunc>::HeapBak,
    > {}
/// @generated by [term_trait_gen::owned::generate]
pub mod owned {
    use words::InverseImplements;
    use tymetafuncspec_core::Set;
    use tymetafuncspec_core::IdxBox;
    use tymetafuncspec_core::BoundedNat;
    use pattern_tmf::OrVariableZeroOrMore;
    use pattern_tmf::OrVariable;
    use pattern_tmf::NamedPattern;
    use crate::words_mod_autoboxed_file_pattern_fib as wmafpf;
    use crate::words_mod_autoboxed_file_pattern_fib::sorts as wmafpfs;
    use crate::term_trait_autoboxed_file_pattern_fib as ttafpf;
    use ccf::CanonicallyConstructibleFrom;
    pub trait Plus<
        Heap,
    >: CanonicallyConstructibleFrom<
            Heap,
            (
                <Heap as InverseImplements<
                    wmafpf::L,
                    OrVariable<(), wmafpfs::LeftOperand>,
                >>::StructuralImplementor,
                (
                    <Heap as InverseImplements<
                        wmafpf::L,
                        OrVariable<(), wmafpfs::RightOperand>,
                    >>::StructuralImplementor,
                    (),
                ),
            ),
        >
    where
        Heap: crate::term_trait_autoboxed_file_pattern_fib::Heap,
    {}
    pub trait LeftOperand<
        Heap,
    >: CanonicallyConstructibleFrom<
            Heap,
            (
                <Heap as InverseImplements<
                    wmafpf::L,
                    OrVariable<(), wmafpfs::Nat>,
                >>::StructuralImplementor,
                (),
            ),
        >
    where
        Heap: crate::term_trait_autoboxed_file_pattern_fib::Heap,
    {}
    pub trait RightOperand<
        Heap,
    >: CanonicallyConstructibleFrom<
            Heap,
            (
                <Heap as InverseImplements<
                    wmafpf::L,
                    OrVariable<(), wmafpfs::Nat>,
                >>::StructuralImplementor,
                (),
            ),
        >
    where
        Heap: crate::term_trait_autoboxed_file_pattern_fib::Heap,
    {}
    pub trait F<
        Heap,
    >: CanonicallyConstructibleFrom<
            Heap,
            (
                <Heap as InverseImplements<
                    wmafpf::L,
                    OrVariable<(), wmafpfs::Nat>,
                >>::StructuralImplementor,
                (),
            ),
        >
    where
        Heap: crate::term_trait_autoboxed_file_pattern_fib::Heap,
    {}
    pub trait Sum<
        Heap,
    >: CanonicallyConstructibleFrom<
            Heap,
            (
                <Heap as InverseImplements<
                    wmafpf::L,
                    OrVariable<(), Set<(), OrVariableZeroOrMore<(), wmafpfs::Nat>>>,
                >>::StructuralImplementor,
                (),
            ),
        >
    where
        Heap: crate::term_trait_autoboxed_file_pattern_fib::Heap,
    {}
    pub trait Nat<
        Heap,
    >: CanonicallyConstructibleFrom<
            Heap,
            (
                <Heap as InverseImplements<
                    wmafpf::L,
                    BoundedNat<()>,
                >>::StructuralImplementor,
                (),
            ),
        > + CanonicallyConstructibleFrom<
            Heap,
            (
                <Heap as InverseImplements<
                    wmafpf::L,
                    IdxBox<(), wmafpfs::F>,
                >>::StructuralImplementor,
                (),
            ),
        > + CanonicallyConstructibleFrom<
            Heap,
            (
                <Heap as InverseImplements<
                    wmafpf::L,
                    IdxBox<(), wmafpfs::Plus>,
                >>::StructuralImplementor,
                (),
            ),
        > + CanonicallyConstructibleFrom<
            Heap,
            (
                <Heap as InverseImplements<
                    wmafpf::L,
                    wmafpfs::Sum,
                >>::StructuralImplementor,
                (),
            ),
        >
    where
        Heap: crate::term_trait_autoboxed_file_pattern_fib::Heap,
    {}
    pub trait FileItem<
        Heap,
    >: CanonicallyConstructibleFrom<
            Heap,
            (
                <Heap as InverseImplements<
                    wmafpf::L,
                    NamedPattern<(), wmafpfs::Plus>,
                >>::StructuralImplementor,
                (),
            ),
        > + CanonicallyConstructibleFrom<
            Heap,
            (
                <Heap as InverseImplements<
                    wmafpf::L,
                    NamedPattern<(), wmafpfs::LeftOperand>,
                >>::StructuralImplementor,
                (),
            ),
        > + CanonicallyConstructibleFrom<
            Heap,
            (
                <Heap as InverseImplements<
                    wmafpf::L,
                    NamedPattern<(), wmafpfs::RightOperand>,
                >>::StructuralImplementor,
                (),
            ),
        > + CanonicallyConstructibleFrom<
            Heap,
            (
                <Heap as InverseImplements<
                    wmafpf::L,
                    NamedPattern<(), wmafpfs::F>,
                >>::StructuralImplementor,
                (),
            ),
        > + CanonicallyConstructibleFrom<
            Heap,
            (
                <Heap as InverseImplements<
                    wmafpf::L,
                    NamedPattern<(), wmafpfs::Sum>,
                >>::StructuralImplementor,
                (),
            ),
        >
    where
        Heap: crate::term_trait_autoboxed_file_pattern_fib::Heap,
    {}
}
