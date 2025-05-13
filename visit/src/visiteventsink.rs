pub enum PopOrProceed {
    Pop,
    Proceed,
}

pub trait VisitEventSink<T, Heap> {
    fn push(&mut self, heap: &Heap, t: &T, total: u32) -> PopOrProceed;
    fn proceed(&mut self, idx: u32, total: u32) -> PopOrProceed;
    fn pop(&mut self, total: u32);
    fn deconstruction_failure(&mut self);
}
