struct Stack {
    items:Vec<u32>,
    top:usize,
    capacity:usize
}
impl Stack{
    fn is_empty(&self)->bool{
        return self.items.len() < 1;
    }

    fn is_full(&self)->bool{
        return self.items.len() > self.capacity;
    }

    fn push(&mut self, item:u32) {
        if !self.is_full() {
            self.top += 1;
            self.items.push(item);
        }
    }

    fn pop(&mut self) {
        if !self.is_empty(){
            self.top -= 1;
            self.items.pop();
        }
    }

    fn peek(&self) -> u32{
        if !self.is_empty() {
            return self.items[self.top - 1];
        }
        return 0;
    }

}

fn main() {
    let mut mystack: Stack = Stack {items : Vec::with_capacity(10), top: 0, capacity:10};
    println!("is empty : {}", mystack.is_empty());
    mystack.push(12);
    mystack.push(24);
    mystack.push(8);
    mystack.push(11);
    println!("{}", mystack.peek());
    mystack.pop();
    println!("{}", mystack.peek());
    println!("is full ? {}", mystack.is_full());


}
