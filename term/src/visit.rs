// use take_mut::Poisonable;

// use crate::{
//     Heaped,
//     case_split::{
//         Callable, CaseSplittable, ConsList, HasBorrowedHeapRef, HasPatternMatchStrategyFor,
//     },
// };

// pub enum MaybeAbortThisSubtree {
//     Abort,
//     Proceed,
// }

// pub trait Visitor<Heap, T>: HasBorrowedHeapRef {
//     fn push(&mut self, t: T, heap: &mut Self::Borrowed<'_, Heap>) -> MaybeAbortThisSubtree;
//     fn proceed(&mut self, t: T, heap: &mut Self::Borrowed<'_, Heap>);
//     fn pop(&mut self, t: T, heap: &mut Self::Borrowed<'_, Heap>);
// }

// pub trait Visitable<V: HasBorrowedHeapRef, PatternMatchStrategyProvider, Heap, DepthFuel, Fnlut>:
//     Sized
// {
//     fn visit(&self, visitor: &mut V, heap: &mut V::Borrowed<'_, Heap>, fnlut: Fnlut);
// }

// pub trait AllVisitable<
//     V: HasBorrowedHeapRef,
//     Heap,
//     Ctx,
//     PatternMatchStrategyProvider,
//     DepthFuel,
//     Fnlut,
// >: Sized
// {
//     fn all_visit(&self, visitor: &mut V, ctx: Ctx, heap: &mut V::Borrowed<'_, Heap>, fnlut: Fnlut);
// }

// impl<V, Heap, Ctx, StrategyProvider, FieldsCar, FieldsCdrCar, FieldsCdrCdr, DepthFuel, Fnlut: Copy>
//     AllVisitable<V, Heap, Ctx, StrategyProvider, DepthFuel, Fnlut>
//     for (FieldsCar, (FieldsCdrCar, FieldsCdrCdr))
// where
//     Self: Copy,
//     Ctx: Copy,
//     V: Visitor<Heap, Ctx>,
//     FieldsCar: Visitable<V, StrategyProvider, Heap, DepthFuel, Fnlut>,
//     V: HasBorrowedHeapRef,
//     StrategyProvider: HasPatternMatchStrategyFor<FieldsCar>,
//     (FieldsCdrCar, FieldsCdrCdr): AllVisitable<V, Heap, Ctx, StrategyProvider, DepthFuel, Fnlut>,
//     FieldsCdrCdr: ConsList,
// {
//     fn all_visit(&self, visitor: &mut V, ctx: Ctx, heap: &mut V::Borrowed<'_, Heap>, fnlut: Fnlut) {
//         let (car, cdr) = self.deconstruct();
//         car.visit(visitor, heap, fnlut);
//         visitor.proceed(ctx, heap);
//         cdr.all_visit(visitor, ctx, heap, fnlut);
//     }
// }
// impl<V, Heap, Ctx, StrategyProvider, FieldsCar, DepthFuel, Fnlut>
//     AllVisitable<V, Heap, Ctx, StrategyProvider, DepthFuel, Fnlut> for (FieldsCar, ())
// where
//     Self: Copy,
//     Ctx: Copy,
//     V: Visitor<Heap, Ctx>,
//     FieldsCar: Visitable<V, StrategyProvider, Heap, DepthFuel, Fnlut>,
//     V: HasBorrowedHeapRef,
//     StrategyProvider: HasPatternMatchStrategyFor<FieldsCar>,
// {
//     fn all_visit(&self, visitor: &mut V, _: Ctx, heap: &mut V::Borrowed<'_, Heap>, fnlut: Fnlut) {
//         let (car, _) = self.deconstruct();
//         car.visit(visitor, heap, fnlut);
//     }
// }

// struct CallablefyVisitor<V, MatchedType, StrategyProvider, DepthFuel, Fnlut> {
//     visitor: V,
//     ctx: MatchedType,
//     fnlut: Fnlut,
//     phantom: std::marker::PhantomData<(StrategyProvider, DepthFuel)>,
// }

// impl<V: HasBorrowedHeapRef, MatchedType, StrategyProvider, DepthFuel, Fnlut> HasBorrowedHeapRef
//     for CallablefyVisitor<V, MatchedType, StrategyProvider, DepthFuel, Fnlut>
// {
//     type Borrowed<'a, Heap: 'a> = V::Borrowed<'a, Heap>;
//     fn with_decayed_heapref<Heap, R, F: Fn(&Heap) -> R>(
//         heap: &mut Self::Borrowed<'_, Heap>,
//         f: F,
//     ) -> R {
//         V::with_decayed_heapref(heap, f)
//     }
// }

// impl<V: Heaped, MatchedType, StrategyProvider, DepthFuel, Fnlut> Heaped
//     for CallablefyVisitor<V, MatchedType, StrategyProvider, DepthFuel, Fnlut>
// {
//     type Heap = V::Heap;
// }

// impl<V, Heap, MatchedType, StrategyProvider, DepthFuel, Fnlut> Callable<Heap, ()>
//     for CallablefyVisitor<V, MatchedType, StrategyProvider, DepthFuel, Fnlut>
// where
//     MatchedType: Copy,
//     V: Visitor<Heap, MatchedType>,
// {
//     fn call(&mut self, _: (), heap: &mut Self::Borrowed<'_, Heap>) {
//         self.visitor.pop(self.ctx, heap);
//     }
// }

// impl<V, Heap, MatchedType, CaseCar, CaseCdr, StrategyProvider, DepthFuel, Fnlut: Copy>
//     Callable<Heap, (CaseCar, CaseCdr)>
//     for CallablefyVisitor<V, MatchedType, StrategyProvider, DepthFuel, Fnlut>
// where
//     MatchedType: Copy,
//     V: Visitor<Heap, MatchedType> + Visitor<Heap, CaseCar>,
//     (CaseCar, CaseCdr):
//         Copy + AllVisitable<V, Heap, MatchedType, StrategyProvider, DepthFuel, Fnlut>,
// {
//     fn call(&mut self, case: (CaseCar, CaseCdr), heap: &mut Self::Borrowed<'_, Heap>) {
//         case.all_visit(&mut self.visitor, self.ctx, heap, self.fnlut);
//     }
// }

// impl<V, Heap, PatternMatchStrategyProvider, T, DepthFuelUpperBits, DepthFuelLastBit, Fnlut>
//     Visitable<
//         V,
//         PatternMatchStrategyProvider,
//         Heap,
//         typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>,
//         Fnlut,
//     > for T
// where
//     typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>: std::ops::Sub<typenum::B1>,
//     T: CaseSplittable<
//             CallablefyVisitor<
//                 V,
//                 T,
//                 PatternMatchStrategyProvider,
//                 typenum::Sub1<typenum::UInt<DepthFuelUpperBits, DepthFuelLastBit>>,
//                 Fnlut,
//             >,
//             Heap,
//             PatternMatchStrategyProvider::Strategy,
//         > + Copy,
//     V: Visitor<Heap, T> + Poisonable,
//     PatternMatchStrategyProvider: HasPatternMatchStrategyFor<T>,
// {
//     fn visit(
//         &self,
//         visitor: &mut V,
//         heap: &mut <V as HasBorrowedHeapRef>::Borrowed<'_, Heap>,
//         fnlut: Fnlut,
//     ) {
//         if let MaybeAbortThisSubtree::Proceed = visitor.push(*self, heap) {
//             take_mut::take(visitor, |visitor| {
//                 let mut callable = CallablefyVisitor {
//                     visitor,
//                     ctx: *self,
//                     fnlut,
//                     phantom: std::marker::PhantomData,
//                 };
//                 <T as CaseSplittable<_, _, _>>::case_split(self, &mut callable, heap);
//                 (callable.visitor, ())
//             });
//         }
//         visitor.pop(*self, heap);
//     }
// }
