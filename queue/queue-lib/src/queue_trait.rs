use std::rc::Rc;

pub trait Queue<T> {
    fn new() -> Self;
    fn enqueue(&mut self, value: T);
    fn dequeue(&mut self) -> Option<Rc<T>>;
    fn peek(&self) -> Option<&T>;
    fn is_empty(&self)-> bool;
    fn traverse<F>(&self, callback: F) where F: Fn(&T);
}
