use crate::{Heaped, co_case_split::CoCallable};

pub enum MaybeTryNextCase {
    AcceptThisCase,
    TryNextCase,
}

pub trait CoVisitor<T>: Heaped {
    fn co_push(&mut self, heap: &mut Self::Heap) -> MaybeTryNextCase;
    fn co_proceed(&mut self, heap: &mut Self::Heap);
    fn co_pop(&mut self, heap: &mut Self::Heap) -> T;
}

pub trait CoVisitable<CV: Heaped, PatternMatchStrategyProvider>: Sized + Heaped
where
    CV: Heaped<Heap = <Self as Heaped>::Heap>,
{
    fn co_visit(visitor: &mut CV, heap: &mut Self::Heap) -> Self;
}

struct CoCallablefyCoVisitor<CV, MatchedType, StrategyProvider> {
    cv: CV,
    ctx: MatchedType,
    phantom: std::marker::PhantomData<StrategyProvider>,
}

impl<V, MatchedType, StrategyProvider> CoCallablefyCoVisitor<V, MatchedType, StrategyProvider> {
    fn descend<Ctx, F: FnMut(&mut CoCallablefyCoVisitor<V, Ctx, StrategyProvider>)>(
        &mut self,
        ctx: Ctx,
        mut f: F,
    ) {
        take_mut::take(self, |this| {
            let CoCallablefyCoVisitor {
                cv, ctx: outer_ctx, ..
            } = this;
            let mut descended = CoCallablefyCoVisitor {
                cv,
                ctx,
                phantom: std::marker::PhantomData,
            };
            f(&mut descended);
            Self {
                cv: descended.cv,
                ctx: outer_ctx,
                phantom: std::marker::PhantomData,
            }
        });
    }
}

impl<V: Heaped, MatchedType, StrategyProvider> Heaped
    for CoCallablefyCoVisitor<V, MatchedType, StrategyProvider>
{
    type Heap = V::Heap;
}

impl<V, CoMatchedType, StrategyProvider> CoCallable<()>
    for CoCallablefyCoVisitor<V, CoMatchedType, StrategyProvider>
where
    CoMatchedType: Copy,
    V: CoVisitor<CoMatchedType>,
{
    fn call(&mut self, heap: &mut Self::Heap) -> Option<()> {
        Some(())
    }
    // fn call(&mut self, _: (), heap: &mut Self::Borrowed<'_, Self::Heap>) {
    //     self.visitor.pop(self.ctx, heap);
    // }

    // fn do_not_call(&mut self) {
    //     panic!("should be unreachable because this is the nil case");
    // }
}
