use crate::linked_list::linked_list::LinkedList;

mod linked_list;



pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn setup_linked_list() -> LinkedList<u32>  {
    let mut ll: LinkedList<u32> = LinkedList::new_ll();
    ll.add_value(2);
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
        assert_eq!(result.is_some_and(|x| *x == 2), true) 
    }

}
