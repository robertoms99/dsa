pub trait Queue<T> {
    fn enqueue(&mut self, value: T);
    fn dequeue(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
    fn peek_mut(&mut self) -> Option<&mut T>;
    fn is_empty(&self)-> bool;
    fn traverse<F>(&self, callback: F) where F: Fn(&T);
    fn size(&self) -> usize;
    fn clear(&mut self);
}
