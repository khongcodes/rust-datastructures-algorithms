//! Simple binary search tree implementation using Box references to Nodes.

// Following methods to be implemented
// [x] BinarySearchTree::new
// [x] BinarySearchTree::add_value
// [ ] BinarySearchTree::find_value - return true if present in tree
// [x] BinarySearchTree::remove_value
// [ ] BinarySearchTree::min -  return smallest value in tree
// [x] BinarySearchTree::print_inorder
// [x] BinarySearchTree::print_preorder
// [x] BinarySearchTree::print_postorder
// [ ] BinarySearchTree::height
//

use std::cmp::Ordering;
use crate::linked_list;


/// A binary search tree struct containing pointers to Node structs.
///
/// Values T in this tree must be comparable with < or > operators (or else this data structure is
/// nonsensical to use).
///
/// * `root`: An Option-wrapped reference to the root Node of the binary search tree.
///         This will be None if there are zero nodes in this tree.
pub struct BinarySearchTree<T: Ord> {
    root: Option<Box<Node<T>>>
}


/// A Node in a BinarySearchTree struct.
///
/// * `value`: Value held in this Node - must be comparable with < or > (implement Ord trait)
/// * `left_branch`: Option-wrapped reference to another Node, which should contain a Node with
///         lesser value. Initializes as None.
/// * `right_branch`: Option-wrapped reference to another Node, which should contain a Node with
///         greater value. Initializes as None.
pub struct Node<T: Ord> {
    value: T,
    left_branch: Option<Box<Node<T>>>,
    right_branch: Option<Box<Node<T>>>
}


/// Enum for traversal node order options on binary trees.
enum TreeTraversalOrders {
    Inorder, Preorder, Postorder
}


// Method implementation for BinarySearchTree struct
impl<T> BinarySearchTree<T> where T: Ord {
    
    /// Return a new, empty BinarySearchTree struct
    fn new() -> BinarySearchTree<T> {
        BinarySearchTree {
            root: None
        }
    }

    /// Add a Node to this BinarySearchTree struct.
    ///
    /// Accomplish this (if there is a root node) by beginning a recursive call to evaluate the new
    /// value against the current node's value, starting with the root member Node.
    ///
    /// It should be noted - if this value is evaluated as Ordering::Equal (== operator) to another Node's
    /// value in this tree, this value will be discarded without a Node being added.
    ///
    /// * `value`: The value to be added into the binary search tree.
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

    /// Find input value in the BinarySearchTree (using Ordering::Equal (== operator)) and remove
    /// it (and its enclosing Node).
    ///
    /// * `value`: Value to be removed from the binary search tree.
    fn remove_value(&mut self, value: T) {
        if self.root.is_some() {
            self.root = self.root.take().unwrap().remove_value_if_child(value);
        }
    }

    /// Create and return a vector containing references to the values held by Nodes in this
    /// BinarySearchTree struct.
    ///
    /// * `order`: A variant of TreeTraversalOrders enum that determines the orders of the value
    /// references in the returned vector
    fn collectpeek_traversal_values(&self, order: TreeTraversalOrders) -> Vec<&T> {
        let mut list = Vec::new();
        match order {
            TreeTraversalOrders::Inorder => { Node::collectpeek_inorder(&self.root, &mut list); },
            TreeTraversalOrders::Preorder => { Node::collectpeek_preorder(&self.root, &mut list); },
            TreeTraversalOrders::Postorder => { Node::collectpeek_postorder(&self.root, &mut list); }
        };
        list
    }

    /// Experimental version of previous method collectpeek_traversal_values_cratell that uses this
    /// crate's LinkedList struct instead of Vec. 
    ///
    /// Not fully implemented - LinkedList is less tested and therefore less reliable than standard library Vecs.
    ///
    /// A speed test should be created between this method and the previous method on
    ///     TreeTraversalOrders::Inorder.
    ///
    /// * `order`: A variant of TreeTraversalOrders enum that determines the orders of the value
    /// references in the returned vector
    fn collectpeek_traversal_values_cratell(&self, order: TreeTraversalOrders) -> linked_list::LinkedList<&T> {
        let mut list = linked_list::LinkedList::new();
        match order {
            TreeTraversalOrders::Inorder => { Node::collectpeek_inorder_cratell(&self.root, &mut list); },
            TreeTraversalOrders::Preorder => { todo!() },
            TreeTraversalOrders::Postorder => { todo!() }
        };
        list
    }
}


// Method implementations for Node struct in a BinarySearchTree struct.
impl<T> Node<T> where T: Ord {

    /// Return a new Node struct with the value T.
    ///
    /// * `value`: Value to be stored in the Node.
    fn new(value: T) -> Node<T> {
        Node {
            value,
            left_branch: None,
            right_branch: None
        }
    }

    /// Return a reference to the value held in this Struct.
    fn peek_value(&self) -> &T {
        &self.value
    }

    /// Add a child Node to this Node with the input value.
    ///
    /// In binary search trees, if a new value to be added is less than a parent node's value, it
    /// should be added as a left-branch child of the parent node. If the new value is greater, it
    /// should be added as the right-branch child of the parent node. If the branch node already
    /// exists, the same operation should be done on the corresponding branch child node - recursively
    /// invoke this method on that branch child node with the same value.
    ///
    /// Needs a mutable self reference so it can assign to left_branch/right_branch members.
    ///
    /// * `value`: value to be held by the child Node to be added to this Node.
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


    /// If this Node's value member matches the input value (== operator), return None - the calling Node struct
    /// is consumed by this, and the returned None will be assigned in this Node's place.
    /// 
    /// Otherwise, if input value member is less than Node's value member, compare it to
    ///     left_branch and assign left_branch member to be the return val of this function invoked
    ///     on the left_branch Node (if it is not None - if it is None, do nothing because this
    ///     value cannot be found in the tree)
    ///
    /// If this Node's value member is greater than Node's value member, compare it to right_branch
    ///     and assign right_branch member to be the return val of this function invoked on the
    ///     right_branch Node (if it is not None (see note above))
    ///     
    /// In both cases where this Node's value is not equal to the input value, return a new
    ///     allocation of this Node in the heap (Box) because we consume the original.
    ///
    /// * `value`: Value to be removed from the Node or its children branches.
    fn remove_value_if_child(mut self, value: &T) -> Option<Box<Node<T>>> {
        match value.cmp(&self.value) {
            Ordering::Less if self.left_branch.is_some() => {
                self.left_branch = self.left_branch.unwrap().remove_value_if_child(value);
            },
            Ordering::Greater if self.right_branch.is_some() => {
                self.right_branch = self.right_branch.unwrap().remove_value_if_child(value);
            },
            Ordering::Equal => { return None; },    // REPLACE THIS WITH return value of calling
                                                    // remove_self_from_tree on self
            _ => ()
        };
        Some(Box::new(self))
    }


    fn remove_self_from_tree(mut self) -> Option<Box<Node<T>>> {
        let left_child_exists = self.left_branch.is_some();
        let right_child_exists = self.right_branch.is_some();

        if !left_child_exists {
            if !right_child_exists {    // self has no children
                return None;
            } else {                    // self has only right child
                return self.right_branch;
            }
        }
        if !right_child_exists {        // self has only left child
            return self.left_branch;
        }
                                        // self has both left and right children
        
        // find the smallest node that is a child of this node's right child, and swap their values
    
        // then recursively call down call to delete the node with the old value
        // (remove_value_if_child), to make sure the new node's parent sets its 
        // reference to the Node-to-be-deleted as None

        // THIS IS A BOX RAW POINTER, needs to be turned back into box with unsafe
        // this raw pointer is expressed as form of reference on self.
        let node_with_new_value = self.right_branch.as_deref_mut().unwrap().find_minimum_child_below();

        
        std::mem::swap(&mut self.value, &mut node_with_new_value.value);

        Some(Box::new(self))
    }

    // CURRENTLY ONLY IN PROOF OF CONCEPT - return pointer to immediate left_branch child node
    // only gets called where we already know the node being passed in has two non-None children
    fn find_minimum_child_below(&mut self) -> Box<&mut Node<T>> {
        let option_wrapped_node = self.left_branch.as_deref_mut().unwrap();
        return Box::new(option_wrapped_node);
    }

    // TODO: DELETE THIS WHOLE BLOCK IF UNNECESSARY
    // // allocate a pointer in heap (Box) to smallest child of this node, return raw pointer
    // fn find_mut_ref_minimum_child_below(mut self, ) -> (Node<T>, *mut Node<T>) {
    //     if let Some(smaller_node) = self.left_branch {
    //         smaller_node.find_mut_ref_minimum_child_below()
    //     } else {
    //         (self, )
    //         Box::into_raw(Box::new(self))
    //     }
    // }
    //
    //
    // fn find_min_value_below(&self) -> &T {
    //     
    // }
    //
    //
    // fn swap_value(other_node: *mut &Node<T>, this_node: Node<T>) -> Option<Box<Node<T>>> {
    //    Some(Box::new(this_node)) 
    // }
    //
    // fn swap_with_min_child_below(&mut self) {
    //     let node_with_new_value = self.right_branch.as_deref_mut().unwrap();
    // }


    /// Assign to a Vec (using a mutable reference to it) node value references of this Node and
    /// its branch-children Nodes, using inorder traversal, recursively calling this method.
    ///
    /// * `opt_node`: Option-wrapped Node reference - can be called directly on references to a
    ///         Node's branch members
    /// * `list`: mutable references to the Vec where Node value references should be added.
    fn collectpeek_inorder<'a>(
        opt_node: &'a Option<Box<Node<T>>>,
        list: &mut Vec<&'a T>
    ) {
        if let Some(boxed_node) = opt_node {
            Node::collectpeek_inorder(&boxed_node.left_branch, list);
            list.push(&boxed_node.value);
            Node::collectpeek_inorder(&boxed_node.right_branch, list);
        }
    }

    /// Assign to a LinkedList (from this crate) (using a mutable reference to it) node value references of this Node and
    /// its branch-children Nodes, using inorder traversal, recursively calling this method.
    ///
    /// * `opt_node`: Option-wrapped Node reference - can be called directly on references to a
    ///         Node's branch members
    /// * `list`: mutable references to the LinkedList where Node value references should be added.
    fn collectpeek_inorder_cratell<'a>(
        opt_node: &'a Option<Box<Node<T>>>, 
        list: &mut linked_list::LinkedList<&'a T>
    ) {
        if let Some(boxed_node) = opt_node {
            Node::collectpeek_inorder_cratell(&boxed_node.left_branch, list);
            list.add_value(&boxed_node.value);
            Node::collectpeek_inorder_cratell(&boxed_node.right_branch, list);
        }
    }

    /// Assign to a Vec (using a mutable reference to it) node value references of this Node and
    /// its branch-children Nodes, using preorder traversal, recursively calling this method.
    ///
    /// * `opt_node`: Option-wrapped Node reference - can be called directly on references to a
    ///         Node's branch members
    /// * `list`: mutable references to the Vec where Node value references should be added.
    fn collectpeek_preorder<'a>(
        opt_node: &'a Option<Box<Node<T>>>, 
        list: &mut Vec<&'a T>
    ) {
        if let Some(boxed_node) = opt_node {
            list.push(&boxed_node.value);
            Node::collectpeek_preorder(&boxed_node.left_branch, list);
            Node::collectpeek_preorder(&boxed_node.right_branch, list);
        }
    }

    /// Assign to a Vec (using a mutable reference to it) node value references of this Node and
    /// its branch-children Nodes, using postorder traversal, recursively calling this method.
    ///
    /// * `opt_node`: Option-wrapped Node reference - can be called directly on references to a
    ///         Node's branch members
    /// * `list`: mutable references to the Vec where Node value references should be added.
    fn collectpeek_postorder<'a>(
        opt_node: &'a Option<Box<Node<T>>>, 
        list: &mut Vec<&'a T>
    ) {
        if let Some(boxed_node) = opt_node {
            Node::collectpeek_postorder(&boxed_node.left_branch, list);
            Node::collectpeek_postorder(&boxed_node.right_branch, list);
            list.push(&boxed_node.value);
        }
    }
}


////////////////////////////////////////////////////////////////////////////
//  TESTS

fn setup_bst() -> BinarySearchTree<u32> {
    let mut bst: BinarySearchTree<u32> = BinarySearchTree::new();
    bst.add_value(4);
    bst.add_value(2);
    bst.add_value(6);
    bst.add_value(1);
    bst.add_value(3);
    bst.add_value(5);
    bst
    //      4
    //     / \
    //   2    6
    //  /\   /
    // 1 3  5
}

#[cfg (test)]
mod tests {
    use super::*;

    #[test]
    fn bst_can_be_created_and_added_to()  {
        let bst = setup_bst();
        assert!(&bst.root.is_some());
        assert_eq!(bst.root.unwrap().value, 4);
    }

    // #[test]
    fn bst_height_can_be_evaluated() {
        let bst = setup_bst();
    }

    // #[test]
    fn bst_can_delete_nodes() {
        let bst = setup_bst();
    }

    #[test]
    fn bst_can_be_traversed_inorder() {
        let bst = setup_bst();
        let list = bst.collectpeek_traversal_values(TreeTraversalOrders::Inorder);
        let mut list_iter = list.into_iter();
        assert_eq!(list_iter.next(), Some(&1));
        assert_eq!(list_iter.next(), Some(&2));
        assert_eq!(list_iter.next(), Some(&3));
        assert_eq!(list_iter.next(), Some(&4));
        assert_eq!(list_iter.next(), Some(&5));
        assert_eq!(list_iter.next(), Some(&6));
    }

    #[test]
    fn bst_can_be_traversed_inorder_with_cratell() {
        let bst = setup_bst();
        let mut list = bst.collectpeek_traversal_values_cratell(TreeTraversalOrders::Inorder);
        assert_eq!(list.dequeue_value(), Some(&1));
        assert_eq!(list.dequeue_value(), Some(&2));
        assert_eq!(list.dequeue_value(), Some(&3));
        assert_eq!(list.dequeue_value(), Some(&4));
        assert_eq!(list.dequeue_value(), Some(&5));
        assert_eq!(list.dequeue_value(), Some(&6));
    }

    #[test]
    fn bst_can_be_traversed_preorder() {
        let bst = setup_bst();
        let list = bst.collectpeek_traversal_values(TreeTraversalOrders::Preorder);
        let mut list_iter = list.into_iter();
        assert_eq!(list_iter.next(), Some(&4));
        assert_eq!(list_iter.next(), Some(&2));
        assert_eq!(list_iter.next(), Some(&1));
        assert_eq!(list_iter.next(), Some(&3));
        assert_eq!(list_iter.next(), Some(&6));
        assert_eq!(list_iter.next(), Some(&5));
    }

    #[test]
    fn bst_can_be_traversed_postorder() {
        let bst = setup_bst();
        let list = bst.collectpeek_traversal_values(TreeTraversalOrders::Postorder);
        let mut list_iter = list.into_iter();
        assert_eq!(list_iter.next(), Some(&1));
        assert_eq!(list_iter.next(), Some(&3));
        assert_eq!(list_iter.next(), Some(&2));
        assert_eq!(list_iter.next(), Some(&5));
        assert_eq!(list_iter.next(), Some(&6));
        assert_eq!(list_iter.next(), Some(&4));
    }
}
