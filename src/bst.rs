// Following methods to be implemented
// [x] BinarySearchTree::new
// [/] BinarySearchTree::add_value
// [ ] BinarySearchTree::find_value - return true if present in tree
// [/] BinarySearchTree::remove_value
// [ ] BinarySearchTree::min -  return smallest value in tree
// [ ] BinarySearchTree::print_inorder
// [ ] BinarySearchTree::print_preorder
// [ ] BinarySearchTree::print_postorder
// [ ] BinarySearchTree::height
//

use std::cmp::Ordering;


pub struct BinarySearchTree<T: Ord> {
    root: Option<Box<Node<T>>>
}

pub struct Node<T: Ord> {
    value: T,
    left_branch: Option<Box<Node<T>>>,
    right_branch: Option<Box<Node<T>>>
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
        match &self.root {
            Some(boxed_node) => {
                // run boxed_node.add_value_as_child(value)
            },
            None => {
                self.root = Some(Box::new(Node::new(value)));
            }
        }
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
            Ordering::Greater => {},
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
}
