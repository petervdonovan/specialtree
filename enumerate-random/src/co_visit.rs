use term::{case_split::Adt, co_case_split::AdmitNoMatchingCase, co_visit::CoVisitor};

use crate::{ChoosingEnumerator, Enumerator};

impl<A> CoVisitor<A> for Enumerator
where
    A: Adt,
{
    fn co_push(&mut self) {
        self.current_depth.0 += 1;
    }

    fn co_proceed(&mut self) {
        // do nothing
    }

    fn co_pop(&mut self) {
        if self.current_depth.0 == 0 {
            panic!("Enumerator::co_pop: depth underflow");
        }
        self.current_depth.0 -= 1;
    }
}

impl<Ctx, Heap, AllCurrentCases, Dp> AdmitNoMatchingCase<Heap, Ctx>
    for ChoosingEnumerator<AllCurrentCases, Dp>
{
    fn no_matching_case(&self, _heap: &mut Heap) -> (Ctx, Self::EndSelectCase) {
        unreachable!()
    }
}
#[macro_export]
macro_rules! impl_out_of_fuel_enumerable {
    ($($target:ty),*) => {
        impl<Heap, Pmsp, $target, Fnlut>
            term::co_visit::CoVisitable<Enumerator, Pmsp, Heap, typenum::U0, Fnlut> for $target
        where
            Fnlut: term::fnlut::HasFn<
                    Self,
                    FnType = fn(&mut $crate::Enumerator, &mut Heap, Fnlut) -> Self,
                >,
        {
            fn co_visit(visitor: &mut $crate::Enumerator, heap: &mut Heap, fnlut: Fnlut) -> Self {
                println!("dbg: recursion limit reached for enumerate; restarting");
                fnlut.get::<Self>()(visitor, heap, fnlut)
            }
        }
    };
}
