use std::usize;
struct Queue {
    front: i32,
    back: i32, 
    size: i32, 
    max_size: i32,
    store: Vec<i32>,
}

impl Queue {
    fn new(max_size: usize) -> Self {
        Queue {
            front: 0,
            back: -1,
            size: 0,
            max_size: max_size as i32,
            store: Vec::with_capacity(max_size),
        }
    }

    fn is_empty(&self) -> bool {
        if self.size == 0 {
            true
        } else {
            false
        }
    }

    fn is_full(&self) -> bool {
        if self.size == self.max_size {
            true
        } else {
            false
        }
    }

    fn enqueue(&mut self, value: i32) {
        if self.is_full() {
            panic!("Queue is full");
        } else {
            self.back = (self.back + 1) % self.max_size;
            self.store.push(value);
            self.size += 1;
            println!("{:?}", self.store);
        }
    }

    fn dequeue(&mut self) -> i32 {
        if self.is_empty() {
            panic!("Queue is empty");
        } else {
            let item = self.store[self.front as usize];
            self.front = (self.front + 1) % self.max_size;
            self.size -= 1;
            println!("{:?} front: {} size: {}", self.store, self.front, self.size);
            item
        }
    }

    fn get_current_items(&self) -> Vec<i32> {
        let mut vec = Vec::new();

        let s = self.size;

        for i in 0..s {
            // println!("{}", self.front as usize + i);
            if self.front + i <= s {
                vec.push(self.store[self.front as usize + i as usize]);
            } else {
                vec.push(self.store[self.front as usize + i as usize - s as usize]);
            }
        };

        vec
    }
}

impl std::fmt::Display for Queue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Queue: {:?}", self.get_current_items())
    }
}

fn main() {
    let mut q = Queue::new(5);
    q.enqueue(1);
    q.enqueue(2);
    q.enqueue(3);
    q.enqueue(4);
    q.enqueue(5);
    q.dequeue();
    q.enqueue(5);
    q.dequeue();
    q.dequeue();
    q.enqueue(5);
    q.enqueue(5);
    // q.enqueue(5);
    // q.dequeue();
    // q.dequeue();
    // q.dequeue();

    println!("{}", q);
}
