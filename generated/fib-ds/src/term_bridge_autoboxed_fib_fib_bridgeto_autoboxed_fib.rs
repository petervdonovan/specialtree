#![feature(custom_inner_attributes)]
#![rustfmt::skip]
#![allow(warnings)]
#![allow(unknown_lints)]
use crate::term_specialized_autoboxed_fib as tsaf;
/// @generated by [term_bridge_gen::generate_heap]
impl crate::term_trait_autoboxed_fib::Heap for tsaf::Heap {}
/// @generated by [term_bridge_gen::generate_owned_impls]
pub mod owned_impls {
    use crate::term_specialized_autoboxed_fib as tsaf;
    impl crate::term_trait_autoboxed_fib::owned::Plus<tsaf::Heap> for tsaf::Plus {}
    impl crate::term_trait_autoboxed_fib::owned::LeftOperand<tsaf::Heap>
    for tsaf::LeftOperand {}
    impl crate::term_trait_autoboxed_fib::owned::RightOperand<tsaf::Heap>
    for tsaf::RightOperand {}
    impl crate::term_trait_autoboxed_fib::owned::F<tsaf::Heap> for tsaf::F {}
    impl crate::term_trait_autoboxed_fib::owned::Sum<tsaf::Heap> for tsaf::Sum {}
    impl crate::term_trait_autoboxed_fib::owned::Nat<tsaf::Heap> for tsaf::Nat {}
}
/// @generated by [words::words_inverse_impls]
pub mod words_inverse_impls {
    use tymetafuncspec_core::Set;
    use tymetafuncspec_core::IdxBox;
    use tymetafuncspec_core::BoundedNat;
    use crate::words_mod_autoboxed_fib as wmaf;
    use crate::words_mod_autoboxed_fib::sorts as wmafs;
    use crate::term_specialized_autoboxed_fib as tsaf;
    impl words::InverseImplements<wmaf::L, wmafs::Plus> for tsaf::Heap {
        type ExternBehavioralImplementor = tsaf::Plus;
        type StructuralImplementor = tsaf::Plus;
    }
    impl words::InverseImplements<wmaf::L, wmafs::LeftOperand> for tsaf::Heap {
        type ExternBehavioralImplementor = tsaf::LeftOperand;
        type StructuralImplementor = tsaf::LeftOperand;
    }
    impl words::InverseImplements<wmaf::L, wmafs::RightOperand> for tsaf::Heap {
        type ExternBehavioralImplementor = tsaf::RightOperand;
        type StructuralImplementor = tsaf::RightOperand;
    }
    impl words::InverseImplements<wmaf::L, wmafs::F> for tsaf::Heap {
        type ExternBehavioralImplementor = tsaf::F;
        type StructuralImplementor = tsaf::F;
    }
    impl words::InverseImplements<wmaf::L, wmafs::Sum> for tsaf::Heap {
        type ExternBehavioralImplementor = tsaf::Sum;
        type StructuralImplementor = tsaf::Sum;
    }
    impl words::InverseImplements<wmaf::L, wmafs::Nat> for tsaf::Heap {
        type ExternBehavioralImplementor = tsaf::Nat;
        type StructuralImplementor = tsaf::Nat;
    }
    impl words::InverseImplements<wmaf::L, Set<(), wmafs::Nat>> for tsaf::Heap {
        type ExternBehavioralImplementor = Set<tsaf::Heap, tsaf::Nat>;
        type StructuralImplementor = Set<tsaf::Heap, tsaf::Nat>;
    }
    impl words::InverseImplements<wmaf::L, BoundedNat<()>> for tsaf::Heap {
        type ExternBehavioralImplementor = BoundedNat<tsaf::Heap>;
        type StructuralImplementor = BoundedNat<tsaf::Heap>;
    }
    impl words::InverseImplements<wmaf::L, IdxBox<(), wmafs::F>> for tsaf::Heap {
        type ExternBehavioralImplementor = IdxBox<tsaf::Heap, tsaf::F>;
        type StructuralImplementor = IdxBox<tsaf::Heap, tsaf::F>;
    }
    impl words::InverseImplements<wmaf::L, IdxBox<(), wmafs::Plus>> for tsaf::Heap {
        type ExternBehavioralImplementor = IdxBox<tsaf::Heap, tsaf::Plus>;
        type StructuralImplementor = IdxBox<tsaf::Heap, tsaf::Plus>;
    }
}
