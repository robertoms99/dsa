mod hash_functions;

use std::{fmt::Debug, ops::Deref, rc::{Rc, Weak}};

use hash_functions::*;
use list_lib::LinkedList;

#[derive(Debug)]
struct DataNode<'a,T> {
  _key: &'a str,
  data: T
}

#[derive(Debug)]
pub struct Hashtable< 'a, T,  const N:usize>  where T: Debug{
  inner:  Vec<Option<Box<LinkedList<DataNode<'a, T>>>>>,
  pub capacity: usize,
  pub length: usize
}

impl<'a,T, const N:usize> Hashtable<'a, T,N> where T: Debug {
  pub fn new() -> Self {
    let capacity = N + (N / 3);
    Self {
      inner: Vec::with_capacity(capacity),
      length: 0,
      capacity
    }
  }

  pub fn insert(&mut self, key: &'a str, value: T) -> usize {

    let idx = self.get_hashed_key(key);

    unsafe {
      if idx > self.inner.len() {
        self.inner.set_len(idx + 1);
      }
    }

    self.insert_collision(idx, key, value);


    return idx;
  }

  fn get_hashed_key(&self, key: &str) -> usize{
    let hash_key = hash(key);
    let idx = hash_key % self.capacity;

    return idx;
  }

  pub fn search(&self, key: &str) -> Option<&T> {
    let idx = self.get_hashed_key(key);

    return match self.inner.get(idx)? {
          Some(list_box) => {
            let data_node = list_box.find(| data_node | data_node._key == key)?;
            Some(&data_node.data)
          },
          None => None
      }
  }


  pub  fn delete (&mut self, key: &str) -> Option<T> {
    let idx = self.get_hashed_key(key);

    return match self.inner.get_mut(idx)? {
            Some(list_box) => {
              let rc_value = list_box.delete(| data_node | data_node._key == key)?;

              if list_box.length == 0 {
                self.inner[idx] = None;
              }

              let data_node = Rc::into_inner(rc_value)?;

               self.length -= 1;

              Some(data_node.data)
          },
          None => None
    };

  }

  fn insert_collision(&mut self, idx: usize, key: &'a str, value: T){
      match self.inner.get_mut(idx) {
        Some( item ) => {
          match item {
            Some(list_box) =>{
             if let Some(_) =  list_box.find( | data_node | data_node._key == key) {
               list_box.replace( | data_node | data_node._key == key, DataNode { _key: key, data: value  }) ;
               return ;
             }

             list_box.add(DataNode { _key: key, data: value });
             self.length += 1;
            },
              None => {
                let data_node = DataNode { _key: key, data: value };
                let mut list_box = Box::new(LinkedList::new());
                list_box.add(data_node);
                self.inner[idx] = Some(list_box);
                self.length += 1;
              }
            }
        },
        _ => {}
    };

  }
}
