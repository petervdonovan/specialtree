pub trait CovisitEventSink<T> {
    fn push(&mut self);
    fn proceed(&mut self);
    fn pop(&mut self);
}
