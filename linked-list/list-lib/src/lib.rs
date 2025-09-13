use std::{
    alloc::{alloc, dealloc, Layout}, cell::RefCell, fmt::Debug, ops::Deref, ptr::{self, null_mut}, rc::Rc
};

#[derive(Debug)]
struct LLNode<T> {
    next: *mut LLNode<T>,
    prev: *mut LLNode<T>,
    value: T,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: *mut LLNode<T>,
    tail: *mut LLNode<T>,
    pub length: usize,
    cursor: *mut LLNode<T>,
}

impl<T: Debug> LinkedList<T> {
    pub fn add(&mut self, value: T) {
        unsafe {
            let layout = Layout::new::<LLNode<T>>();
            let node_ptr = alloc(layout) as *mut LLNode<T>;

            let tmp = LLNode {
                next: null_mut(),
                prev: if self.cursor != null_mut() {
                    self.cursor
                } else {
                    self.tail
                },
                value,
            };

            ptr::write(node_ptr, tmp);

            if self.cursor != null_mut() {
                self.remove_nodes((*self.cursor).next as *mut LLNode<T>);
            }

            if self.head == null_mut() {
                self.head = node_ptr;
            }

            if self.tail != null_mut() {
                (*self.tail).next = node_ptr;
                (*self.cursor).next = node_ptr;
            }

            self.tail = node_ptr;
            self.cursor = node_ptr;

            self.length = self.length + 1;
        }
    }

    pub fn get_head(&self) -> Option<&T> {
        if self.head != null_mut() {
            unsafe {
                let value = &(*self.head).value;

                return Some(value);
            }
        }

        return None;
    }

       pub fn get_head_mut(&self) -> Option<&mut T> {
        if self.head != null_mut() {
            unsafe {
                return Some(&mut (*self.head).value);
            }
        }

        return None;
    }

    pub fn back(&mut self) -> Option<&T> {
        if self.cursor != null_mut() {
            unsafe {
                let back_node = (*self.cursor).prev;
                if back_node == null_mut() {
                    return None;
                }
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
                if next_node == null_mut() {
                    return None;
                }
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

    fn _find_node(&self, callback: impl Fn(&T) -> bool) -> Option<&mut LLNode<T>> {
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

    pub fn find(&self, callback: impl Fn(&T) -> bool) -> Option<&T> {
        match self._find_node(callback) {
            Some(node) => Some(&node.value),
            None => None,
        }
    }

    pub fn find_mut(&self, callback: impl Fn(&T) -> bool) -> Option<&mut T> {
        match self._find_node(callback) {
            Some(node) => Some(&mut node.value),
            None => None,
        }
    }

    pub fn replace(&mut self, callback: impl Fn(&T) -> bool, new_value: T) -> Option<T> {
        let node = self._find_node(callback)?;
        let node_clone_value: T;

        unsafe {
            node_clone_value = ptr::read(&node.value as *const T);
        }

        node.value = new_value;

        return Some(node_clone_value);
    }

    pub fn delete(&mut self, callback: impl Fn(&T) -> bool) -> Option<T> {
        let node = self._find_node(callback)?;
        let node_clone_value: T;
        unsafe {
          let node_ptr = node as *mut LLNode<T>;
          node_clone_value = ptr::read(&node.value as *const T);
          self.remove_node(node_ptr);
        }

        return Some(node_clone_value);
    }

    pub fn traverse<R>(&self, callback: impl Fn(&T) -> R) {
        let mut cur_node = self.head;
        while cur_node != null_mut() {
            unsafe {
                callback(&(*cur_node).value);
                cur_node = (*cur_node).next;
            }
        }
    }

    pub fn traverse_reverse<R>(&self, callback: impl Fn(&T) -> R) {
        let mut cur_node = self.tail;
        while cur_node != null_mut() {
            unsafe {
                callback(&(*cur_node).value);
                cur_node = (*cur_node).prev;
            }
        }
    }
    pub fn clear(&mut self) {
        self.remove_nodes(self.head as *mut LLNode<T>);
        self.head = null_mut();
        self.tail = null_mut();
        self.cursor = null_mut();
    }

    pub fn delete_head(&mut self) -> Option<T> {
        let node_clone_value: T;
        if self.head != null_mut() {
            unsafe {
                node_clone_value = ptr::read(&(*self.head).value as *const T);
                let next = (*self.head).next;
                self.remove_node(self.head as *mut LLNode<T>);
                self.head = next;
                if self.head == null_mut() {
                    self.tail = null_mut();
                    self.cursor = null_mut();
                }
            }

            return Some(node_clone_value);
        }

        return None;
    }

    fn remove_node(&mut self, node: *mut LLNode<T>) {
        if node == null_mut() {
            return;
        }
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

    fn remove_nodes(&mut self, node: *mut LLNode<T>) {
        let mut cur_node = node;
        while cur_node != null_mut() {
            unsafe {
                let layout = Layout::new::<LLNode<T>>();
                let aux_node = cur_node;
                cur_node = (*cur_node).next as *mut LLNode<T>;
                ptr::drop_in_place(aux_node);
                dealloc(aux_node as *mut u8, layout);


                self.length = self.length - 1;
            }
        }
    }

    pub fn new() -> Self {
        LinkedList {
            head: null_mut(),
            tail: null_mut(),
            length: 0,
            cursor: null_mut(),
        }
    }
}
