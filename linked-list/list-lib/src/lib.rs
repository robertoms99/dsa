use std::{alloc::{alloc, dealloc, Layout}, cell::{Ref, RefCell}, fmt::Debug, ops::Deref, ptr::{self, null, null_mut}, rc::Rc};

#[derive(Debug)]
struct LLNode<T> {
  next: *mut LLNode<T>,
  prev: *mut LLNode<T>,
  value: Rc<T>
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
    let rc_value =  Rc::new(value);
    unsafe  {
         let layout = Layout::new::<LLNode<T>>();
         let node_ptr = alloc(layout) as *mut LLNode<T>;

          let tmp = LLNode {
            next: null_mut(),
            prev:  if self.cursor != null_mut(){ self.cursor as *mut LLNode<T> } else { self.tail },
            value: rc_value,
        };

        ptr::write(node_ptr, tmp);

        if self.cursor != null_mut() {
          self.remove_nodes((*self.cursor).next as *mut LLNode<T>);
        }

        if self.tail != null_mut() {
          (*self.tail).next = node_ptr;
          (*self.cursor).next = node_ptr;
        }

        self.tail = node_ptr;
        self.cursor = node_ptr;

        if self.head == null() { self.head = node_ptr; }
        self.length = self.length + 1;
    }
  }


  pub fn get_head(&self) -> Option<&T> {
    if self.head != null() {
      unsafe  {
          return Some(&(*(*self.head).value));
      }
    }

    return None;
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

  pub fn delete_head(&mut self) -> Option<Rc<T>> {
    let value: Rc<T>;
    if self.head != null() {
      unsafe  {
        value = Rc::clone(&((*self.head).value));
        let next = (*self.head).next;
        self.remove_node(self.head as *mut LLNode<T>);
        self.head = next;
      }


      return Some(value);

    }


    return None;
  }

  fn remove_node(&mut self, node: *mut LLNode<T>){

    if node == null_mut() { return;}
    unsafe {
      let prev = (*node).prev;
      let next = (*node).next;

      if prev != null_mut() {
        (*prev).next = next;
      }

      if next != null_mut() {
        (*next).prev = prev;
      }

      (*node).next = null_mut();

      self.remove_nodes(node);
    }
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
