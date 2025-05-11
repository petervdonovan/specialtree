pub trait CovisitEventSink<T> {
    fn push(&mut self);
    fn proceed(&mut self, idx: u32, total: u32);
    fn pop(&mut self);
}
