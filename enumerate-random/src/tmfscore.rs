// use term::co_visit::CoVisitable;
// use tymetafuncspec_core::{BoundedNat, Set, SetHeapBak};

// use crate::{
//     Enumerator,
//     distribution_provider::{CollectionSize, HasDistributionFor},
// };

// impl<Heap, Pmsp, DepthFuelUpperBits, DepthFuelLastBit, Fnlut, Dp>
//     CoVisitable<
//         Enumerator<Dp>,
//         Pmsp,
//         Heap,
//         typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>,
//         Fnlut,
//     > for BoundedNat<Heap>
// where
//     Dp: HasDistributionFor<BoundedNat<Heap>, Output = BoundedNat<Heap>>,
// {
//     fn co_visit(visitor: &mut Enumerator<Dp>, _: &mut Heap, _: Fnlut) -> Self {
//         visitor
//             .dp
//             .sample(&mut visitor.random_state, visitor.current_depth)
//     }
// }

// impl<Heap, Elem, Pmsp, DepthFuelUpperBits, DepthFuelLastBit, Fnlut, Dp>
//     CoVisitable<
//         Enumerator<Dp>,
//         Pmsp,
//         Heap,
//         typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>,
//         Fnlut,
//     > for Set<Heap, Elem>
// where
//     Fnlut: Copy,
//     typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>: std::ops::Sub<typenum::B1>,
//     Elem: term::co_visit::CoVisitable<
//             Enumerator<Dp>,
//             Pmsp,
//             Heap,
//             typenum::Sub1<typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>>,
//             Fnlut,
//         >,
//     Heap: term::SuperHeap<SetHeapBak<Heap, Elem>>,
//     Dp: HasDistributionFor<Set<Heap, Elem>, Output = CollectionSize>,
// {
//     fn co_visit(visitor: &mut Enumerator<Dp>, heap: &mut Heap, fnlut: Fnlut) -> Self {
//         let size = visitor
//             .dp
//             .sample(&mut visitor.random_state, visitor.current_depth);
//         let elems = (0..size.0)
//             .map(|_| Elem::co_visit(visitor, heap, fnlut))
//             .collect();
//         Set::new(heap, elems)
//     }
// }
