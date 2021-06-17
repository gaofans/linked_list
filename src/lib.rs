pub mod first;
pub mod second;

#[cfg(test)]
mod tests{
    use super::second::List;

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
    #[test]
    fn test_drop(){
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
    }
    #[test]
    fn test_peek(){
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        list.peek().map(|value| println!("{}",value));
        list.peek_mut().map(|value| *value = *value + 1);
        list.peek().map(|value| println!("{}",value));
    }
    #[test]
    fn test_into_iter(){
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        for value in list.into_iter() {
            println!("{}",value)
        }
    }
    #[test]
    fn test_iter(){
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        for value in list.iter() {
            println!("{}",value)
        }
    }
    #[test]
    fn test_iter_mut(){
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        for value in list.iter_mut() {
            *value = *value + 1;
        }
        for value in list.iter() {
            println!("{}",value)
        }
    }
}
