// Following methods to be implemented
// [x] BinarySearchTree::new
// [x] BinarySearchTree::add_value
// [ ] BinarySearchTree::find_value - return true if present in tree
// [x] BinarySearchTree::remove_value
// [ ] BinarySearchTree::min -  return smallest value in tree
// [ ] BinarySearchTree::print_inorder
// [ ] BinarySearchTree::print_preorder
// [ ] BinarySearchTree::print_postorder
// [ ] BinarySearchTree::height
//

use std::mem;
use std::cmp::Ordering;


pub struct BinarySearchTree<T: Ord> {
    root: Option<Box<Node<T>>>
}

pub struct Node<T: Ord> {
    value: T,
    left_branch: Option<Box<Node<T>>>,
    right_branch: Option<Box<Node<T>>>
}

enum TreeTraversalOrders {
    Inorder, Preorder, Postorder
}

impl<T> BinarySearchTree<T> where T: Ord {
    
    /// 
    fn new() -> BinarySearchTree<T> {
        BinarySearchTree {
            root: None
        }
    }

    /// 
    ///
    /// * `value`: 
    fn add_value(&mut self, value: T) {
        match &mut self.root {
            Some(boxed_node) => {
                boxed_node.add_value_as_child(value);
            },
            None => {
                self.root = Some(Box::new(Node::new(value)));
            }
        }
    }

    fn remove_value(&mut self, value: T) {
        if self.root.is_some() {
            self.root = self.root.take().unwrap().remove_value_if_child(value);
        }
    }

    fn get_traverse_value_vec(&self, order: TreeTraversalOrders) -> Vec<&T> {
        let mut list = Vec::new();
        match order {
            TreeTraversalOrders::Inorder => { Node::inorder_walk(&self.root, &mut list); },
            TreeTraversalOrders::Preorder => { /* IMPLEMENT ME */ },
            TreeTraversalOrders::Postorder => { /* IMPLEMENT ME */ }
        };
        list
    }
}

impl<T> Node<T> where T: Ord {

    /// 
    ///
    /// * `value`: 
    fn new(value: T) -> Node<T> {
        Node {
            value,
            left_branch: None,
            right_branch: None
        }
    }

    fn peek_value(&self) -> &T {
        &self.value
    }

    /// 
    /// Needs a mutable self reference so it can assign to left_branch/right_branch members
    /// * `value`: 
    fn add_value_as_child(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Less => {
                if let Some(boxed_node) = &mut self.left_branch {
                    boxed_node.add_value_as_child(value);
                } else {
                    self.left_branch = Some(Box::new(Node::new(value)));
                }
            },
            Ordering::Greater => {
                if let Some(boxed_node) = &mut self.right_branch {
                    boxed_node.add_value_as_child(value);
                } else  {
                    self.right_branch = Some(Box::new(Node::new(value)));
                }
            },
            Ordering::Equal => ()
        }
    }


    /// Note: this is a recursive method that consumes its calling struct.
    /// If this Node's value member matches the input value, return None
    /// Otherwise, if input value member is less than Node's value member, compare it to
    ///     left_branch and assign left_branch member to be the return val of this function invoked
    ///     on the left_branch Node (if it is not None - if it is None, do nothing because this
    ///     value cannot be found in the tree)
    ///If this Node's value member is greater than Node's value member, compare it to right_branch
    ///     and assign right_branch member to be the return val of this function invoked on the
    ///     right_branch Node (if it is not None (see note above))
    /// * `value`: 
    fn remove_value_if_child(mut self, value: T) -> Option<Box<Node<T>>> {
        match value.cmp(&self.value) {
            Ordering::Less if self.left_branch.is_some() => {
                self.left_branch = self.left_branch.unwrap().remove_value_if_child(value);
            },
            Ordering::Greater if self.right_branch.is_some() => {
                // if let Some(boxed_node) = self.right_branch {
                self.right_branch = self.right_branch.unwrap().remove_value_if_child(value);
                // }
            },
            Ordering::Equal => { return None; },
            _ => ()
        };
        Some(Box::new(self))
    }


    fn inorder_walk<'a>(opt_node: &'a Option<Box<Node<T>>>, list: &mut Vec<&'a T>) {
        if let Some(boxed_node) = opt_node {
            Node::inorder_walk(&boxed_node.left_branch, list);
            list.push(&boxed_node.value);
            Node::inorder_walk(&boxed_node.right_branch, list);
        }
        // match opt_node {
        //     None => { return; }
        //     Some(boxed_node) => {
        //         Node::inorder_walk(&boxed_node.left_branch, list);
        //         list.push(&boxed_node.value);
        //     }
        // }

        // if boxed_node.is_none() { return; }
        // // let node = &boxed_node.unwrap();
        // boxed_node.as_ref().inspect(|x| {
        //     Node::inorder_walk(&x.left_branch, list)
        // });
        // Node::inorder_walk(&boxed_node.unwrap().left_branch, list);
        // list.push(&boxed_node.unwrap().value);
        // Node::inorder_walk(&boxed_node.unwrap().right_branch, list);
    }
}


////////////////////////////////////////////////////////////////////////////
//  TESTS

fn setup_bst() -> BinarySearchTree<u32> {
    let mut bst: BinarySearchTree<u32> = BinarySearchTree::new();
    bst.add_value(2);
    bst.add_value(1);
    bst.add_value(3);
    bst
}

#[cfg (test)]
mod tests {
    use super::*;

    #[test]
    fn bst_can_be_created_and_added_to()  {
        let bst = setup_bst();
        assert!(&bst.root.is_some());
        assert_eq!(bst.root.unwrap().value, 2);
    }

    #[test]
    fn bst_can_be_traversed_inorder() {
        let bst = setup_bst();
        let list = bst.get_traverse_value_vec(TreeTraversalOrders::Inorder);
        let mut list_iter = list.into_iter();
        // let mut list_iter = bst.get_traverse_value_vec(TreeTraversalOrders::Preorder).into_iter();
        assert_eq!(list_iter.next(), Some(&1));
        assert_eq!(list_iter.next(), Some(&2));
        assert_eq!(list_iter.next(), Some(&3));
    }
}
