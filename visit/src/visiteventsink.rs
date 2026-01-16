use aspect::Aspect;

pub enum PopOrProceed {
    Pop,
    Proceed,
}

pub trait AspectVisitor {
    type A: Aspect;
}

pub trait VisitEventSink<T, Heap>: AspectVisitor {
    fn push(&mut self, heap: &Heap, t: &T, total: u32) -> PopOrProceed;
    fn proceed(&mut self, idx: u32, total: u32) -> PopOrProceed;
    fn pop(&mut self, total: u32);
    fn deconstruction_failure(&mut self);
}
