mod dynamic_queue;
mod fixed_queue;
mod queue_trait;

pub use dynamic_queue::Queue as DynamicQueue;
pub use fixed_queue::FixedQueue;
pub use queue_trait::Queue as QueueTrait;

#[cfg(test)]
mod test {
  use crate::{dynamic_queue::*,fixed_queue::*, QueueTrait};

  #[test]
  pub fn it_should_peek_dynamic_queue(){
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

  #[test]
  pub fn it_should_peek_fixed_queue(){
  let mut routing_queue: FixedQueue<i32,5> = FixedQueue::new();

  routing_queue.enqueue(1);
  routing_queue.enqueue(2);
  routing_queue.enqueue(3);
  routing_queue.enqueue(4);
  routing_queue.enqueue(5);

  routing_queue.dequeue();
  routing_queue.dequeue();

  routing_queue.enqueue(6);
  routing_queue.enqueue(7);

   if let Some(value) = routing_queue.peek() {
      assert_eq!(*value, 3);
    } else {
      panic!("Given NoNe value from queue peek");
    }

  }
}
