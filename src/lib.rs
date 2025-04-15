use crate::linked_list::LinkedList;

mod linked_list;



pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn setup_linked_list() -> LinkedList<u32>  {
    let mut ll: LinkedList<u32> = LinkedList::new();
    println!("adding values");
    ll.add_value(2);
    println!("adding values again");
    ll.add_value(4);
    return ll;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn linked_list_works() {
        let basic_ll = setup_linked_list();
        let result: Option<&u32> = basic_ll.peek_head_value();
        println!("running assert");
        assert!(result.is_some_and(|x| *x == 2));
    }

}
