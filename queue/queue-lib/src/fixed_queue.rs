use std::{cell::RefCell, rc::Rc, usize};

use crate::QueueTrait;

#[derive(Debug)]
pub struct FixedQueue<T:Sized, const N: usize> {
      capacity: usize,
      length: usize,
      front: RefCell<isize>,
      rear: RefCell<isize>,
      inner: [Option<Rc<T>>;N],
 }

 impl<T: Sized, const N: usize> FixedQueue<T,N> {
     pub fn is_full(&self) -> bool {
        self.length == self.capacity
      }
 }

impl<T: Sized, const N: usize> QueueTrait<T> for  FixedQueue<T,N> {
  fn new() -> Self {
      Self {
        capacity: N,
        length: 0,
        inner: [const {None}; N],
        front: RefCell::new(-1),
        rear: RefCell::new(-1)
      }
  }

  fn enqueue(&mut self, value: T) {
      let size = self.capacity as isize;
      let mut front_borrow_mut = self.front.borrow_mut();
      let mut rear_borrow_mut = self.rear.borrow_mut();

      if self.is_full() { return; }

      *front_borrow_mut = if *front_borrow_mut == -1 { 0 } else { *front_borrow_mut };

      *rear_borrow_mut = (*rear_borrow_mut + 1) % size;

      let rear_idx = *rear_borrow_mut as usize;

      self.inner[rear_idx] = Some(Rc::new(value));

      self.length = self.length + 1;
    }

  fn dequeue(&mut self) -> Option<std::rc::Rc<T>> {
          let size = self.capacity as isize;
          let mut front_borrow_mut = self.front.borrow_mut();

          if self.is_empty() { return None; }

          let front_idx = *front_borrow_mut as usize;

          let value = Rc::clone(&(self.inner[front_idx].as_mut().expect("")));

          self.inner[front_idx] = None;

          *front_borrow_mut = (*front_borrow_mut + 1) % size;

          self.length -= 1;

          return Some(value);
    }

  fn traverse<F>(&self, callback: F) where F: Fn(&T) {
    if self.is_empty() {return;}

    let front_borrow = self.front.borrow();

    let mut front_aux_idx = *front_borrow;
    for i in 1..(self.length + 1) {
       let Some(item) = &self.inner[front_aux_idx as usize] else {
        return;
        };

       callback(item.as_ref());

      front_aux_idx = (front_aux_idx + 1) % self.capacity as isize;
    }

  }

  fn peek(&self) -> Option<&T> {
        if self.is_empty(){ return None; }

        let front_idx = *self.front.borrow() as usize;
        let value  = &self.inner[front_idx];
        return value.as_deref();
    }

    fn is_empty(&self)-> bool {
        self.length == 0
    }
 }
