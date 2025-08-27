use std::{rc::Rc};

use list_lib::LinkedList;

pub struct Queue<T> {
  inner_list: LinkedList<T>
}

impl<T> Queue<T> {
  pub fn new() -> Self {
    Self {
      inner_list: LinkedList::<T>::new()
    }
  }

  pub fn enqueue(&mut self, value: T) {
    self.inner_list.add(value);
  }

  pub fn dequeue(&mut self) -> Option<Rc<T>> {
    self.inner_list.delete_head()
  }

  pub fn peek(&self) -> Option<&T> {
    self.inner_list.get_head()
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  pub fn it_should_peek(){
    let mut queue : Queue<i32> = Queue::new();

    queue.enqueue(5);
    queue.enqueue(0);
    queue.enqueue(-3);
    queue.dequeue();

    if let Some(value) = queue.peek() {
      assert_eq!(*value, 0);
    } else {
      panic!("Given NoNe value from queue peek");
    }
  }
}
