use std::{cell::RefCell, fmt::Debug};

use super::queue_trait::Queue as QueueTrait;
use list_lib::LinkedList;

#[derive(Debug)]
pub struct Queue<T> {
  inner_list: LinkedList<T>
}

impl<T: Debug> Queue<T> {
   pub fn new() -> Self {
    Self {
      inner_list: LinkedList::<T>::new()
    }
  }
}

impl<T: Debug> QueueTrait<T> for Queue<T> {
  fn size(&self) -> usize {
    self.inner_list.length
  }

 fn enqueue(&mut self, value: T) {
    self.inner_list.add(value);
  }

 fn dequeue(&mut self) -> Option<T> {
    self.inner_list.delete_head()
  }

   fn peek(&self) -> Option<&T> {
    self.inner_list.get_head()
  }

  fn peek_mut(&mut self) -> Option<&mut T> {
    self.inner_list.get_head_mut()
  }

  fn traverse<F>(&self, callback: F) where F: Fn(&T) {
      self.inner_list.traverse(callback);
  }

  fn clear(&mut self) {
      while let Some(_) = self.dequeue() {}
  }

  fn is_empty(&self)-> bool {
        match self.inner_list.get_head() {
          Some(v) => true,
          None => false
        }
    }
}
