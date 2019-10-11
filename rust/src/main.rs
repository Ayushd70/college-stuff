mod queue;
mod stack;

use queue::Queue;
use stack::Stack;

fn main() {
    println!("Stack implementation");
    let mut stack = Stack::default();
    stack.push(1);
    println!("{}", stack.peek());
    stack.pop();
    stack.pop();

    println!("Queue implementation");
    let mut queue = Queue::default();
    queue.enqueue(1);
    println!("front: {}", queue.rear());
    println!("rear: {}", queue.front());
    queue.enqueue(2);
    println!("front: {}", queue.rear());
    println!("rear: {}", queue.front());
    println!("Successfully dequeued: {}", queue.dequeue());
    println!("front: {}", queue.rear());
    println!("rear: {}", queue.front());
    println!("Successfully dequeued: {}", queue.dequeue());
    println!("front: {}", queue.rear());
    println!("rear: {}", queue.front());
}

#[cfg(test)]
mod tests {

    use super::queue::Queue;
    use std::convert::TryInto;

    #[test]
    fn test_queue_enqueue() {
        let mut queue = Queue::default();
        assert!(queue.is_empty());
        queue.enqueue(1);
        assert!(!queue.is_empty());
        queue.dequeue();
        assert!(queue.is_empty());
        for i in 0..queue.capacity {
            assert!(!queue.is_full());
            queue.enqueue(i.try_into().unwrap());
        }
        assert!(queue.is_full());
    }
}
