use std::{alloc::{alloc, dealloc, Layout}, fmt::Debug, ptr::{null, null_mut}};

#[derive(Debug)]
struct LLNode<T> {
  next: *mut LLNode<T>,
  prev: *mut LLNode<T>,
  value: T
}

#[derive(Debug)]
pub struct LinkedList<T> {
  head: *const LLNode<T>,
  tail: *mut LLNode<T>,
  length: usize,
  cursor: *mut LLNode<T>
}

impl<T>  LinkedList<T> {
  pub fn add(&mut self, value: T) {
    unsafe  {
         let layout = Layout::new::<LLNode<T>>();
         let node = alloc(layout) as *mut LLNode<T>;
        (*node).next =  null_mut();
        (*node).prev = if self.cursor != null_mut(){ self.cursor as *mut LLNode<T> } else { self.tail };
        (*node).value = value;

        if self.cursor != null_mut() {
          self.remove_nodes((*self.cursor).next as *mut LLNode<T>);
        }

        if self.tail != null_mut() {
          (*self.tail).next = node;
          (*self.cursor).next = node;
        }

        self.tail = node;
        self.cursor = node;

        if self.head == null() { self.head = node; }
        self.length = self.length + 1;
    }
  }

  pub fn back(&mut self) -> Option<&T> {
    if self.cursor != null_mut() {
      unsafe {
        let back_node = (*self.cursor).prev;
        if back_node == null_mut() {return None;}
        self.cursor = back_node;
        return Some((&(*back_node).value));
      }
    }
    return None;
  }

    pub fn forward(&mut self) -> Option<&T> {
    if self.cursor != null_mut() {
      unsafe {
        let next_node = (*self.cursor).next;
        if next_node == null_mut() {return None;}
        self.cursor = next_node;
        return Some((&(*next_node).value));
      }
    }
    return None;
  }

  pub fn current(&self) -> Option<&T> {
  if self.cursor != null_mut() {
      unsafe {
        return Some(&(*self.cursor).value);
    }
  }
    return None;
  }

  pub fn traverse(&self, callback: impl Fn(&T)){
    let mut cur_node = self.head;
    while cur_node != null() {
      unsafe {
        callback(&(*cur_node).value);
        cur_node = (*cur_node).next;
      }
    }
  }

    pub fn traverse_reverse(&self, callback: impl Fn(&T)){
    let mut cur_node = self.tail;
    while cur_node != null_mut() {
      unsafe {
        callback(&(*cur_node).value);
        cur_node = (*cur_node).prev;
      }
    }
  }
  pub fn clear(&mut self){
    self.remove_nodes(self.head as *mut LLNode<T>);
    self.head= null();
    self.tail = null_mut();
    self.cursor = null_mut();
  }

  fn remove_nodes(&mut self, node: *mut LLNode<T>){
     let mut cur_node = node;
     while cur_node != null_mut()  {
       unsafe {
        let layout = Layout::new::<LLNode<T>>();
        let aux_node = cur_node as *mut u8;
        cur_node = (*cur_node).next as *mut LLNode<T>;

        dealloc(aux_node, layout);
      }
    }
  }

  pub fn new() -> Self{
    LinkedList { head: null(), tail: null_mut(), length: 0,cursor: null_mut() }
  }
}
