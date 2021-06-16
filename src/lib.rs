pub mod first;
#[cfg(test)]
mod tests{
    use super::first::List;
    #[test]
    fn start(){
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        assert_eq!(list.pop().unwrap(),4);
        assert_eq!(list.pop().unwrap(),3);
        assert_eq!(list.pop().unwrap(),2);
        list.push(5);
        assert_eq!(list.pop().unwrap(),5);
        assert_eq!(list.pop().unwrap(),1);
        assert_eq!(list.pop(),None);
    }
}
