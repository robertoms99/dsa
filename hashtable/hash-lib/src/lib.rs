mod hashtable;

pub use hashtable::Hashtable;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_hashmap_insert() {
       let mut hashtable = Hashtable::<i32,10>::new();
       hashtable.insert("hola".to_string(), 1);
       hashtable.insert("holx".to_string(), 2);

       assert_eq!(hashtable.search("hola"), Some(&1));
    }

    #[test]
    fn it_works_hashmap_delete() {
       let mut hashtable = Hashtable::<i32,10>::new();
       hashtable.insert("holx".to_string(), 2);
       hashtable.insert("holx".to_string(), 3);

       assert_eq!(hashtable.length, 1);
       assert_eq!(hashtable.delete("holx"), Some(3));
       assert_eq!(hashtable.length, 0);
    }
}
