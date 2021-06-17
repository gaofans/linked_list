struct Node<T>{
    value:T,
    next:Link<T>
}
type Link<T> = Option<Box<Node<T>>>;
pub struct List<T>{
    head:Link<T>
}
pub struct IntoIter<T>(List<T>);
impl <T> Iterator for IntoIter<T>{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
pub struct Iter<'a,T>{
    next:Option<&'a Node<T>>
}
impl <'a,T> Iterator for Iter<'a,T>{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node|{
            self.next = node.next.as_deref();
            &node.value
        })
    }
}
pub struct IterMut<'a,T>{
    next:&'a mut Node<T>
}
impl <'a,T > Iterator for IterMut<'a,T>{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next().take().map(|node|{
            self.next = node.next().as_deref_mut();
            &mut node.value
        })
    }
}
impl <T> List<T> {
    pub fn new() -> List<T>{
        List{head:None}
    }
    pub fn push(&mut self,value:T){
        let node = Node{
            value,
            next:self.head.take()
        };
        self.head = Link::Some(Box::new(node));
    }
    pub fn pop(&mut self) -> Option<T>{
        self.head.take().map(|node|{
            self.head = node.next;
            node.value
        })
    }
    pub fn peek(&self) -> Option<&T>{
        self.head.as_ref().map(|node| &node.value)
    }
    pub fn peek_mut(&mut self) -> Option<&mut T>{
        self.head.as_mut().map(|node| &mut node.value)
    }
    pub fn into_iter(self) -> IntoIter<T>{
        IntoIter(self)
    }
    pub fn iter(&self) -> Iter<T>{
        Iter{next:self.head.as_deref()}
    }
    pub fn iter_mut(&mut self) -> IterMut<T>{
        IterMut{next:self.head.task().as_deref_mut()}
    }

}

impl <T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current_link = self.head.take();
        while let Some(mut node) = current_link{
            current_link = node.next.take();
        }
    }
}