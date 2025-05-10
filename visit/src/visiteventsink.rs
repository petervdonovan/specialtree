pub enum PopOrProceed {
    Pop,
    Proceed,
}

pub trait VisitEventSink<T, Heap> {
    fn push(&mut self, heap: &Heap, t: &T) -> PopOrProceed;
    fn proceed(&mut self) -> PopOrProceed;
    fn pop(&mut self);
    fn deconstruction_failure(&mut self);
}
