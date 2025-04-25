// // use cst::data_structure::Heap;

// include!("fib_parse.rs");

// // fn round_trip<T>(input: &str)
// // where
// //     Cstfy<Heap, T>: term::co_visit::CoVisitable<Parser<'_, ()>, (), Heap, typenum::B0>,
// // {
// //     let mut heap = Heap::default();
// //     let mut errors = Vec::new();
// //     let (nl, offset) = <Cstfy<Heap, T> as term::co_visit::CoVisitable<
// //         Parser<'_, ()>,
// //         (),
// //         Heap,
// //         typenum::B0,
// //     >>::parse(input, 0.into(), &mut heap, &mut errors);
// //     println!(
// //         "parse result: {}",
// //         <Cstfy<Heap, T> as term::co_visit::CoVisitable<
// //         Parser<'_, ()>,
// //         (),
// //         Heap,
// //         typenum::B0,>>::unparse(&nl, &heap)
// //     );
// // }

// // fn round_trip_transparent<T>(input: &str)
// // where
// //     CstfyTransparent<Heap, T>: term::co_visit::CoVisitable<Parser<'_, ()>, (), Heap, typenum::B0>,
// // {
// //     let mut heap = Heap::default();
// //     let mut errors = Vec::new();
// //     let (nl, offset) = <CstfyTransparent<Heap, T> as term::co_visit::CoVisitable<
// //         Parser<'_, ()>,
// //         (),
// //         Heap,
// //         typenum::B0,
// //     >>::parse(input, 0.into(), &mut heap, &mut errors);
// //     println!(
// //         "parse result: {}",
// //         <CstfyTransparent<Heap, T> as term::co_visit::CoVisitable<
// //             Parser<'_, ()>,
// //             (),
// //             Heap,
// //             typenum::B0,
// //         >>::unparse(&nl, &heap)
// //     );
// // }

// // #[test]
// // fn main() {
// //     round_trip::<tymetafuncspec_core::BoundedNat<Heap>>("23");
// //     round_trip_transparent::<cst::data_structure::Nat>("23");
// // }
