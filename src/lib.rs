// use crate::linked_list::{ LinkedList, Node };

#[allow(dead_code)]
mod linked_list;

#[allow(dead_code)]
mod bst;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// INTEGRATION TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
}
