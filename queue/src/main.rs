#[derive(Debug)]
struct Queue {
    front: usize,
    pear: usize,
    size: usize,
    items: Vec<String>,
}

impl Queue {
    fn handle_pear(&mut self, increase: bool) {
        if increase {
            if self.pear >= self.size {
                self.pear = 0;
            }
            self.pear += 1;
        } else {
            if self.front == self.pear {
                self.pear = 0;
            }
        }
    }

    fn handle_front(&mut self, increase: bool) {
        if increase {
            if self.is_empty() {
                self.front += 1;
            }
        } else {
            if self.front >= self.size {
                if self.pear > 0 {
                    self.front = 1;
                } else {
                    self.front = 0;
                }
            }
            self.front += 1;
        }
    }

    pub fn is_full(&self) -> bool {
        return self.pear >= self.size && self.front == 1 || self.pear + 1 == self.front;
    }

    pub fn is_empty(&self) -> bool {
        return self.front <= 0;
    }

    pub fn enqueue(&mut self, item: String) {
        if !self.is_full() {
            self.handle_pear(true);
            self.handle_front(true);
            self.items[self.pear - 1] = item;
        }
    }

    pub fn dequeue(&mut self) {
        if !self.is_empty() {
            self.items[self.front - 1] = "".to_string();
            self.handle_front(false);
            self.handle_pear(false);
        }
    }

    pub fn peek(&self) -> String {
        return self.items[self.front - 1].to_string();
    }
}

fn main() {
    let mut q = Queue {
        front: 0,
        pear: 0,
        size: 10,
        items: vec!["".to_string(); 10],
    };

    println!("{:#?}", q);
    q.enqueue("Hi".to_string());
    q.enqueue("HiB".to_string());
    q.enqueue("HiC".to_string());
    q.enqueue("HiD".to_string());
    println!("{:#?}", q);
    q.enqueue("HiF".to_string());
    q.dequeue();
    println!("{:#?}", q);
    q.dequeue();
    q.dequeue();
    println!("{:#?}", q);
}
