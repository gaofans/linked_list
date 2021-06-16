use std::mem;

struct Node{
    value:i32,
    next:Link
}
enum Link{
    Empty,More(Box<Node>)
}
pub struct List{
    head:Link
}

impl List{
    pub fn new() -> List{
        List{head:Link::Empty}
    }
    pub fn push(&mut self,value:i32){
        let node = Node{
            value,
            next:mem::replace(&mut self.head,Link::Empty)
        };
        self.head = Link::More(Box::new(node));
    }
    pub fn pop(&mut self) -> Option<i32>{

        match mem::replace(&mut self.head,Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.value)
            }
        }
    }
}