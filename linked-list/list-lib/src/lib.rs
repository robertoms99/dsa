use std::{alloc::{alloc, dealloc, Layout}, cell::{Ref, RefCell}, fmt::Debug, ops::{Deref, DerefMut},  ptr::{self, null, null_mut}, rc::Rc};

#[derive(Debug)]
struct LLNode<T> {
  next: *mut LLNode<T>,
  prev: *mut LLNode<T>,
  value: Rc<T>
}

#[derive(Debug)]
pub struct LinkedList<T> {
  head: *mut LLNode<T>,
  tail: *mut LLNode<T>,
  pub length: usize,
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

        if self.head == null_mut() { self.head = node_ptr; }
        self.length = self.length + 1;
    }
  }


  pub fn get_head(&self) -> Option<&T> {
    if self.head != null_mut() {
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

  fn _traverse<R>(&self, callback: impl Fn(&T)-> R ){
    let mut cur_node = self.head;
    while cur_node != null_mut() {
      unsafe {
        callback(&(*cur_node).value);
        cur_node = (*cur_node).next;
      }
    }
  }

  fn _find_node(&self,callback: impl Fn(&T) -> bool) -> Option<&mut LLNode<T>>{
        let mut cur_node = self.head;
        while cur_node != null_mut() {
          unsafe {
            if callback(&(*cur_node).value) {
              return Some(&mut (*cur_node));
            }
            cur_node = (*cur_node).next;
          }
        }
        return None;
  }

  pub fn find(&self,callback: impl Fn(&T) -> bool) -> Option<&T> {
          match self._find_node(callback) {
            Some(node) => Some(&node.value),
            None => None
          }
  }

  pub fn replace(&mut self,callback: impl Fn(&T) -> bool, new_value: T) -> Option<Rc<T>>{
    let node =  self._find_node(callback)?;
    let prev_value = Rc::clone(&node.value);
    node.value = Rc::new(new_value);

    return Some(prev_value);
  }

  pub fn delete(&mut self,callback: impl Fn(&T) -> bool) -> Option<Rc<T>>{
      let node = self._find_node(callback)?;
      let value = Rc::clone(&node.value);
      self.remove_node(node as *mut LLNode<T>);

      return Some(value)
  }

  pub fn traverse<R>(&self, callback: impl Fn(&T) -> R){
    return self._traverse(callback);
  }

    pub fn traverse_reverse<R>(&self, callback: impl Fn(&T)-> R){
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
    self.head = null_mut();
    self.tail = null_mut();
    self.cursor = null_mut();
  }

  pub fn delete_head(&mut self) -> Option<Rc<T>> {
    let value: Rc<T>;
    if self.head != null_mut() {
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
        let aux_node = cur_node;
        cur_node = (*cur_node).next as *mut LLNode<T>;

        ptr::drop_in_place(aux_node);
        dealloc(aux_node  as *mut u8, layout);

        self.length = self.length - 1;
      }
    }
  }

  pub fn new() -> Self{
    LinkedList { head: null_mut(), tail: null_mut(), length: 0,cursor: null_mut() }
  }
}
