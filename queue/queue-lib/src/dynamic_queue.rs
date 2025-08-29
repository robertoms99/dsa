use std::{rc::Rc};
use super::queue_trait::Queue as QueueTrait;
use list_lib::LinkedList;

pub struct Queue<T> {
  inner_list: LinkedList<T>
}

impl<T> QueueTrait<T> for Queue<T> {
  fn new() -> Self {
    Self {
      inner_list: LinkedList::<T>::new()
    }
  }

 fn enqueue(&mut self, value: T) {
    self.inner_list.add(value);
  }

 fn dequeue(&mut self) -> Option<Rc<T>> {
    self.inner_list.delete_head()
  }

   fn peek(&self) -> Option<&T> {
    self.inner_list.get_head()
  }

  fn traverse<F>(&self, callback: F) where F: Fn(&T) {
      self.inner_list.traverse(callback);
  }

  fn is_empty(&self)-> bool {
        match self.inner_list.get_head() {
          Some(v) => true,
          None => false
        }
    }
}
