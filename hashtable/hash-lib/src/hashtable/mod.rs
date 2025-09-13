mod hash_functions;

use std::{fmt::Debug, rc::{Rc}};

use hash_functions::*;
use list_lib::LinkedList;

#[derive(Debug)]
struct DataNode<T> {
  _key: String,
  data: T
}

#[derive(Debug)]
pub struct Hashtable<  T,  const N:usize>  where T: Debug{
  inner:  Vec<Option<Box<LinkedList<DataNode< T>>>>>,
  pub capacity: usize,
  pub size: usize,
  pub length: usize
}

impl<T, const N:usize> Hashtable< T,N> where T: Debug {
  pub fn new() -> Self {
    let capacity = N + (N / 3);
    Self {
      inner: Vec::with_capacity(capacity),
      length: 0,
      size: N,
      capacity
    }
  }

  pub fn insert(&mut self, key: String, value: T) -> Option<usize> {
    if self.length >= self.size {
      return None;
    }

    let idx = self.get_hashed_key(key.as_str());

    unsafe {
      if idx > self.inner.len() {
        self.inner.set_len(idx + 1);
      }
    }

    self.insert_collision(idx, key, value);

    self.inner.as_ptr();
    return Some(idx);
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

  pub fn search_mut(&mut self, key: &str) -> Option<&mut T> {
    let idx = self.get_hashed_key(key);

    return match self.inner.get_mut(idx)? {
          Some(list_box) => {
            let data_node = list_box.find_mut(| data_node | data_node._key == key)?;
            Some(&mut data_node.data)
          },
          None => None
      }
  }

  pub  fn delete (&mut self, key: &str) -> Option<T> {
    let idx = self.get_hashed_key(key);

    return match self.inner.get_mut(idx)? {
            Some(list_box) => {
              let data_node = list_box.delete(| data_node | data_node._key == key)?;

              if list_box.length == 0 {
                self.inner[idx] = None;
              }

               self.length -= 1;

              Some(data_node.data)
          },
          None => None
    };
  }

  fn insert_collision(&mut self, idx: usize, key: String, value: T){
      match self.inner.get_mut(idx) {
        Some( item ) => {
          match item {
            Some(list_box) =>{
              let kstr= key.as_str();
             if let Some(data_node) =  list_box.find_mut( | data_node | data_node._key ==  kstr ) {
               let new_data_node = DataNode { _key: key, data: value  };
               data_node.data = new_data_node.data;
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
